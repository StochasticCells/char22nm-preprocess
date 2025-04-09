// cargo run --bin setup --release

use anyhow::Context as _;
use liberty_db::{ast::GroupSet, Cell, DefaultCtx};
use std::{
  collections::BTreeMap,
  fs::{self, File},
  io::{BufWriter, Write},
  path::Path,
};

use char22nm_preprocess::{Config, CELL_GROUP, PVT, RUN};

fn main() -> anyhow::Result<()> {
  #[derive(Debug, serde::Serialize)]
  struct TableInfo {
    p: String,
    v: f32,
    t: f32,
    index1: Vec<f64>,
    index2: Vec<f64>,
  }
  fn process_one(args: &(&str, &str, f32, f32)) -> anyhow::Result<(String, TableInfo)> {
    let (pvt_name, p, v, t) = args;
    let file_name = format!("/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/timing_power_noise/NLDM/tcbn22ullbwp30p140_110b/tcbn22ullbwp30p140{pvt_name}.lib") ;
    let library = liberty_db::library::Library::<DefaultCtx>::parse_lib(
      &fs::read_to_string(file_name)?,
    )?;
    let cell_dff = library.cell.get("DFCNQD1BWP30P140").context("Failed to get cell")?;
    let pin_d = cell_dff.pin.get("D".into()).context("Failed to get pin D")?;
    let timing = pin_d
      .timing
      .get(
        "CP".into(),
        None,
        Some(&liberty_db::timing::TimingType::SETUP_RISING),
        Some(&cell_dff.parse_logic_boolexpr("CDN")?),
      )
      .context("Failed to get timing")?;
    let setup_table =
      timing.rise_constraint.as_ref().context("Failed to get setup_table")?;
    Ok((
      pvt_name.to_string(),
      TableInfo {
        p: p.to_string(),
        v: *v,
        t: *t,
        index1: setup_table.index_1.clone(),
        index2: setup_table.index_2.clone(),
      },
    ))
  }
  let infos = PVT.iter().map(process_one).collect::<Result<BTreeMap<_, _>, _>>()?;
  let writer = BufWriter::new(File::create("DFCNQD1BWP30P140.json")?);
  serde_json::to_writer_pretty(writer, &infos)?;
  Ok(())
}

#[test]
fn main11() -> anyhow::Result<()> {
  let file_name = "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/timing_power_noise/NLDM/tcbn22ullbwp30p140_110b/tcbn22ullbwp30p140tt0p8v25c.lib";
  let netlist_path =  "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Back_End/spice/tcbn22ullbwp30p140_110a/tcbn22ullbwp30p140_110a.spi";
  let model_path = "/data/junzhuo/tech/tsmc/22nm/iPDK_CRN22ULL_shrink_T-N22-CR-SP-004-W1_v1.3_1p1a_20211230_all/models/hspice/25/cln22ull_2d5_elk_v1d3_1p1_shrink0d855_embedded_usage.l";
  let hspice_path = "/toolset/eda/synopsys/hspice/2021.09/bin/hspice";
  let btdcell_path = "/data/junzhuo/HOME/SHARE/junzhuo/btdcell/bin/btdcell";
  let temp_dir = fs::canonicalize(&Path::new("../template"))?;
  let conf_dir = fs::canonicalize(&Path::new("../config"))?;
  let cli_dir = fs::canonicalize(&Path::new("../cli"))?;
  let run_dir = fs::canonicalize(&Path::new("../run"))?;
  let cpu_num: usize = 32;
  let mut task_list = Vec::new();
  if let Ok(mut library) = liberty_db::library::Library::<DefaultCtx>::parse_lib(
    &std::fs::read_to_string(Path::new(file_name))?,
  ) {
    let mut _library = library.clone();
    _library.cell.clear();
    for (cell_group, (pin_name, related, when_str, rise), cell_names) in CELL_GROUP {
      let mut cells = GroupSet::<Cell<DefaultCtx>>::default();
      for &cell_name in cell_names.iter() {
        let mut cell = library.cell.take(cell_name).expect("msg");
        let when = if *when_str == "" {
          None
        } else {
          Some(cell.parse_logic_boolexpr(when_str)?)
        };
        cell.leakage_power.clear();
        for pin in cell.pin.iter_mut() {
          pin.internal_power.clear();
          if pin.name.as_ref() == (*pin_name).into() {
            pin
              .timing
              .retain(|t| t.related_pin.contains(related) && t.when == when);
            for timing in pin.timing.iter_mut() {
              if *rise {
                timing.cell_fall = None;
                timing.fall_transition = None;
                timing.rise_constraint = None;
                timing.fall_constraint = None;
              } else {
                timing.cell_rise = None;
                timing.rise_transition = None;
                timing.rise_constraint = None;
                timing.fall_constraint = None;
              }
            }
          } else {
            pin.timing.clear();
          }
        }
        cells.insert(cell);
      }
      _library.cell = cells;
      let lib_path = temp_dir.join(format!("{cell_group}.lib"));
      let mut writer = BufWriter::new(File::create(lib_path.clone())?);
      write!(&mut writer, "{}", _library)?;
      for (run_name, sample_num, sample_type) in RUN {
        for (pvt_name, p, v, t) in PVT {
          let name = format!("{cell_group}_{run_name}_{pvt_name}");
          let yaml_path = conf_dir.join(format!("{name}.yaml"));
          let _cpu_num = cell_names.len();
          serde_yaml::to_writer(
            BufWriter::new(File::create(yaml_path.clone())?),
            &Config {
              Name: name,
              Voltage: *v,
              Temperature: *t,
              LibFilePath: format!("{}", lib_path.display()),
              NetListPath: netlist_path.to_string(),
              ModelPath: model_path.to_string(),
              ModelSection: p.to_string(),
              LvfType: sample_type.to_string(),
              LVFSamplingNum: sample_num,
              NumCPU: cell_names.len(),
              HspicePath: hspice_path.to_string(),
              CellNameList: cell_names.iter().map(ToString::to_string).collect(),
            },
          )?;
          task_list.push((_cpu_num, format!("{btdcell_path} {}&", yaml_path.display())));
        }
      }
    }
  }
  task_list.sort_by(|a, b| a.0.cmp(&b.0));
  let mut cli_list: Vec<(usize, Vec<_>)> = Vec::new();
  'L1: for (task_cost, task) in task_list {
    for (cli_cap, cli_cmds) in cli_list.iter_mut() {
      if *cli_cap + task_cost <= cpu_num {
        *cli_cap += task_cost;
        cli_cmds.push(task);
        continue 'L1;
      }
    }
    cli_list.push((task_cost, vec![task]));
  }
  for (idx, (_, paths)) in cli_list.into_iter().enumerate() {
    let cli_path = cli_dir.join(format!("run_{idx}.sh"));
    write!(
      BufWriter::new(File::create(cli_path)?),
      "#!/bin/bash\nsource /env.d/eda.shrc\ncd {}\n{}\nwait",
      run_dir.display(),
      paths.join("\n")
    )?;
  }
  Ok(())
}
