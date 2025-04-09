// mod arcs;
use liberty_db::{
  ast::GroupSet,
  cell::{self, Cell},
  DefaultCtx,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::{BinaryHeap, HashMap, HashSet},
  fs::{self, File},
  io::{BufWriter, Write},
  path::Path,
};
#[expect(non_snake_case)]
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
  pub Name: String,
  pub Voltage: f32,
  pub Temperature: f32,
  pub LibFilePath: String,
  pub NetListPath: String,
  pub ModelPath: String,
  pub ModelSection: String,
  pub LvfType: String,
  pub LVFSamplingNum: usize,
  pub NumCPU: usize,
  pub HspicePath: String,
  pub CellNameList: Vec<String>,
}
impl Config {
  fn push_cell(&mut self, cell: String) {
    self.CellNameList.push(cell)
  }
}

pub const PVT: &[(&str, &str, f32, f32)] = &[
  ("ffg0p88v0c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.88, 0.0),
  ("ffg0p88v125c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.88, 125.0),
  ("ffg0p88vm40c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.88, -40.0),
  ("ffg0p99v0c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.99, 0.0),
  ("ffg0p99v125c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.99, 125.0),
  ("ffg0p99vm40c", "FFGlobalCorner_LocalMC_MOS_MOSCAP", 0.99, -40.0),
  ("ssg0p72v0c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.72, 0.0),
  ("ssg0p72v125c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.72, 125.0),
  ("ssg0p72vm40c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.72, -40.0),
  ("ssg0p81v0c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.81, 0.0),
  ("ssg0p81v125c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.81, 125.0),
  ("ssg0p81vm40c", "SSGlobalCorner_LocalMC_MOS_MOSCAP", 0.81, -40.0),
  ("tt0p8v25c", "TTGlobalCorner_LocalMC_MOS_MOSCAP", 0.8, 25.0),
  ("tt0p8v85c", "TTGlobalCorner_LocalMC_MOS_MOSCAP", 0.8, 85.0),
  ("tt0p9v25c", "TTGlobalCorner_LocalMC_MOS_MOSCAP", 0.9, 25.0),
  ("tt0p9v85c", "TTGlobalCorner_LocalMC_MOS_MOSCAP", 0.9, 85.0),
];

pub const CELL_GROUP: &[(&str, (&str, &str, &str, bool), &[&str])] = &[
  (
    "INV",
    ("ZN", "I", "", true),
    &[
      "INVD0BWP30P140",
      "INVD0P7BWP30P140",
      "INVD12BWP30P140",
      "INVD16BWP30P140",
      "INVD18BWP30P140",
      "INVD1BWP30P140",
      "INVD20BWP30P140",
      "INVD24BWP30P140",
      "INVD2BWP30P140",
      "INVD32BWP30P140",
      "INVD4BWP30P140",
      "INVD6BWP30P140",
      "INVD8BWP30P140",
    ],
  ),
  // (
  //   "BUFF",
  //   ("Z", "I", "", true),
  //   &[
  //     "BUFFD0BWP30P140",
  //     "BUFFD0P7BWP30P140",
  //     "BUFFD12BWP30P140",
  //     "BUFFD16BWP30P140",
  //     "BUFFD1BWP30P140",
  //     "BUFFD20BWP30P140",
  //     "BUFFD24BWP30P140",
  //     "BUFFD2BWP30P140",
  //     "BUFFD4BWP30P140",
  //     "BUFFD6BWP30P140",
  //     "BUFFD8BWP30P140",
  //   ],
  // ),
  (
    "ND2",
    ("ZN", "A1", "", true),
    &[
      "ND2D0BWP30P140",
      "ND2D16BWP30P140",
      "ND2D1BWP30P140",
      "ND2D2BWP30P140",
      "ND2D3BWP30P140",
      "ND2D4BWP30P140",
      "ND2D6BWP30P140",
      "ND2D8BWP30P140",
    ],
  ),
  (
    "NR2",
    ("ZN", "A1", "", true),
    &[
      "NR2D0BWP30P140",
      "NR2D16BWP30P140",
      "NR2D1BWP30P140",
      "NR2D2BWP30P140",
      "NR2D3BWP30P140",
      "NR2D4BWP30P140",
      "NR2D6BWP30P140",
      "NR2D8BWP30P140",
    ],
  ),
  (
    "AN2",
    ("Z", "A1", "", true),
    &[
      "AN2D0BWP30P140",
      "AN2D16BWP30P140",
      "AN2D1BWP30P140",
      "AN2D2BWP30P140",
      "AN2D4BWP30P140",
      "AN2D6BWP30P140",
      "AN2D8BWP30P140",
    ],
  ),
  (
    "OR2",
    ("Z", "A1", "", true),
    &[
      "OR2D0BWP30P140",
      "OR2D16BWP30P140",
      "OR2D1BWP30P140",
      "OR2D2BWP30P140",
      "OR2D4BWP30P140",
      "OR2D6BWP30P140",
      "OR2D8BWP30P140",
    ],
  ),
  (
    "XOR2",
    ("Z", "A1", "!A2", true),
    &["XOR2D0BWP30P140", "XOR2D1BWP30P140", "XOR2D2BWP30P140", "XOR2D4BWP30P140"],
  ),
  (
    "XNR2",
    ("ZN", "A1", "A2", true),
    &["XNR2D0BWP30P140", "XNR2D1BWP30P140", "XNR2D2BWP30P140", "XNR2D4BWP30P140"],
  ),
  (
    "OAI21",
    ("ZN", "A1", "", true),
    &[
      "OAI21D0BWP30P140",
      "OAI21D16BWP30P140",
      "OAI21D1BWP30P140",
      "OAI21D2BWP30P140",
      "OAI21D4BWP30P140",
      "OAI21D6BWP30P140",
      "OAI21D8BWP30P140",
    ],
  ),
  (
    "AOI21",
    ("ZN", "A1", "", true),
    &[
      "AOI21D0BWP30P140",
      "AOI21D16BWP30P140",
      "AOI21D1BWP30P140",
      "AOI21D2BWP30P140",
      "AOI21D4BWP30P140",
      "AOI21D6BWP30P140",
      "AOI21D8BWP30P140",
    ],
  ),
  (
    "FA1",
    ("CO", "A", "B&!CI", true),
    &["FA1D0BWP30P140", "FA1D1BWP30P140", "FA1D2BWP30P140", "FA1D4BWP30P140"],
  ),
  (
    "HA1",
    ("CO", "A", "", true),
    &["HA1D0BWP30P140", "HA1D1BWP30P140", "HA1D2BWP30P140", "HA1D4BWP30P140"],
  ),
];

pub const RUN: [(&str, usize, &str); 1] = [("10k_QMC", 10000, "QmcSample")];
// const RUN: [(&str, usize, &str); 1] = [("100kMC", 100000, "McSample")];
// [("golden", 50001, "QmcSample"), ("baseline", 10001, "McSample")];

#[test]
fn check() -> anyhow::Result<()> {
  let cpu_num = 32;
  let run_dir = fs::canonicalize(&Path::new("../run"))?;
  let netlist_path =  "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Back_End/spice/tcbn22ullbwp30p140_110a/tcbn22ullbwp30p140_110a.spi";
  let model_path = "/data/junzhuo/tech/tsmc/22nm/iPDK_CRN22ULL_shrink_T-N22-CR-SP-004-W1_v1.3_1p1a_20211230_all/models/hspice/25/cln22ull_2d5_elk_v1d3_1p1_shrink0d855_embedded_usage.l";
  let hspice_path = "/toolset/eda/synopsys/hspice/2021.09/bin/hspice";
  let btdcell_path = "/data/junzhuo/HOME/SHARE/junzhuo/btdcell/bin/btdcell";
  let temp_dir = fs::canonicalize(&Path::new("../template3"))?;
  let conf2_dir = fs::canonicalize(&Path::new("../config3"))?;
  let cli2_dir = fs::canonicalize(&Path::new("../cli3"))?;
  let run2_dir = fs::canonicalize(&Path::new("../run3"))?;
  let mut config_map: HashMap<(&'static str, &'static str, &'static str), Config> =
    HashMap::new();
  for (cell_group, _, cell_names) in CELL_GROUP {
    for cell_name in cell_names.iter() {
      for (run_name, sample_num, sample_type) in RUN {
        'L1: for (pvt_name, p, v, t) in PVT {
          if let Ok(files) = run_dir
            .join(format!("{cell_group}_{run_name}_{pvt_name}"))
            .join("deck")
            .join(cell_name)
            .join("01_combinational")
            .read_dir()
          {
            for element in files {
              let path = element.unwrap().path();
              if let Some(extension) = path.extension() {
                if extension == "csv" {
                  continue 'L1;
                }
              }
            }
          }
          if let Some(config) = config_map.get_mut(&(cell_group, run_name, pvt_name)) {
            config.push_cell(cell_name.to_string());
            config.NumCPU += 1;
          } else {
            config_map.insert(
              (cell_group, run_name, pvt_name),
              Config {
                Name: format!("{cell_group}_{run_name}_{pvt_name}"),
                Voltage: *v,
                Temperature: *t,
                LibFilePath: format!(
                  "{}",
                  temp_dir.join(format!("{cell_group}.lib")).display()
                ),
                NetListPath: netlist_path.to_string(),
                ModelPath: model_path.to_string(),
                ModelSection: p.to_string(),
                LvfType: sample_type.to_string(),
                LVFSamplingNum: sample_num,
                NumCPU: 1,
                HspicePath: hspice_path.to_string(),
                CellNameList: vec![cell_name.to_string()],
              },
            );
          }
        }
      }
    }
  }
  let mut task_list: Vec<Config> = config_map.into_values().collect();
  task_list.sort_by(|a, b| a.NumCPU.cmp(&b.NumCPU));
  let mut cli_list: Vec<(usize, Vec<_>)> = Vec::new();
  'L1: for task in task_list {
    let yaml_path = conf2_dir.join(format!("{}.yaml", task.Name));
    serde_yaml::to_writer(BufWriter::new(File::create(yaml_path.clone())?), &task)?;
    for (cli_cap, cli_cmds) in cli_list.iter_mut() {
      if *cli_cap + task.NumCPU <= cpu_num {
        *cli_cap += task.NumCPU;
        cli_cmds.push(format!("{} {}", btdcell_path, yaml_path.display()));
        continue 'L1;
      }
    }
    cli_list
      .push((task.NumCPU, vec![format!("{} {}", btdcell_path, yaml_path.display())]));
  }
  for (idx, (_, paths)) in cli_list.into_iter().enumerate() {
    let cli2_path = cli2_dir.join(format!("run_{idx}.sh"));
    write!(
      BufWriter::new(File::create(cli2_path)?),
      "#!/bin/bash\nsource /env.d/eda.shrc\ncd {}\n{}\nwait",
      run2_dir.display(),
      paths.join("\n")
    )?;
  }
  Ok(())
}
#[test]
fn pruned_lib() -> anyhow::Result<()> {
  let cell_list: HashSet<String> = vec![
    "HA1D1BWP30P140",
    "AOI21D1BWP30P140",
    "XNR2D1BWP30P140",
    "OAI21D1BWP30P140",
    "XOR2D1BWP30P140",
    "OR2D1BWP30P140",
    "AN2D1BWP30P140",
    "INVD1BWP30P140",
    "ND2D1BWP30P140",
    "NR2D1BWP30P140",
    "DFCNQD1BWP30P140",
  ]
  .into_iter()
  .map(String::from)
  .collect();
  let file_name = "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/timing_power_noise/NLDM/tcbn22ullbwp30p140_110b/tcbn22ullbwp30p140tt0p8v25c.lib";
  if let Ok(mut library) = liberty_db::library::Library::<DefaultCtx>::parse_lib(
    &std::fs::read_to_string(Path::new(file_name))?,
  ) {
    library.cell.retain(|f| cell_list.contains(&f.name));
    let lib_path = "pruned.lib";
    let mut writer = BufWriter::new(File::create(lib_path)?);
    write!(&mut writer, "{}", library)?;
    let data: BinaryHeap<_> = library.cell.iter().collect();
    // let data: BinaryHeap<_> = a_lot_of_numbers.collect()
  }
  Ok(())
}

#[test]
fn pruned_lvf_lib() -> anyhow::Result<()> {
  let cell_list: HashSet<String> = vec![
    "HA1D1BWP30P140",
    "AOI21D1BWP30P140",
    "XNR2D1BWP30P140",
    "OAI21D1BWP30P140",
    "XOR2D1BWP30P140",
    "OR2D1BWP30P140",
    "AN2D1BWP30P140",
    "INVD1BWP30P140",
    "ND2D1BWP30P140",
    "NR2D1BWP30P140",
    "DFCNQD1BWP30P140",
  ]
  .into_iter()
  .map(String::from)
  .collect();
  let file_name = "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/LVF/CCS/tcbn22ullbwp30p140_110b/tcbn22ullbwp30p140tt0p8v25c_hm_lvf_p_ccs.lib";
  if let Ok(mut library) = liberty_db::library::Library::<DefaultCtx>::parse_lib(
    &std::fs::read_to_string(Path::new(file_name))?,
  ) {
    library.cell.retain(|f| cell_list.contains(&f.name));
    let lib_path = "pruned_lvf.lib";
    let mut writer = BufWriter::new(File::create(lib_path)?);
    write!(&mut writer, "{}", library)?;
  }
  Ok(())
}

#[test]
fn valid_cells() -> anyhow::Result<()> {
  let cell_list: HashSet<&str> = [
    "HA1D0BWP30P140",
    "HA1D1BWP30P140",
    "AOI21D0BWP30P140",
    "AOI21D1BWP30P140",
    "XNR2D0BWP30P140",
    "XNR2D1BWP30P140",
    "OAI21D0BWP30P140",
    "OAI21D1BWP30P140",
    "XOR2D0BWP30P140",
    "XOR2D1BWP30P140",
    "OR2D0BWP30P140",
    "OR2D1BWP30P140",
    "OR2D2BWP30P140",
    "AN2D0BWP30P140",
    "AN2D1BWP30P140",
    "AN2D2BWP30P140",
    "INVD0BWP30P140",
    "INVD1BWP30P140",
    "INVD2BWP30P140",
    "ND2D0BWP30P140",
    "ND2D1BWP30P140",
    "NR2D0BWP30P140",
    "NR2D1BWP30P140",
    "DFCNQD1BWP30P140",
  ]
  .into_iter()
  .collect();
  let file_name = "/data/junzhuo/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/timing_power_noise/NLDM/tcbn22ullbwp30p140_110b/tcbn22ullbwp30p140tt0p8v25c.lib";
  if let Ok(library) = liberty_db::library::Library::<DefaultCtx>::parse_lib(
    &std::fs::read_to_string(Path::new(file_name))?,
  ) {
    for cell in library.cell.iter() {
      if !cell_list.contains(cell.name.as_str()) {
        println!("{},", cell.name);
      }
    }
  }
  Ok(())
}

#[test]
fn replace_timing_baseline() -> anyhow::Result<()> {
  let template_file = "pruned_active_lvf.lib";
  let data1_file = "/code/char0425/baseline1/out/btdcell.lib";
  let data2_file = "/code/char0425/baseline2/out/btdcell.lib";
  match (
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(template_file),
    )?),
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(data1_file),
    )?),
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(data2_file),
    )?),
  ) {
    (Ok(mut template_lib), Ok(mut data1_lib), Ok(data2_lib)) => {
      data1_lib
        .cell
        .insert(template_lib.cell.get("DFCNQD1BWP30P140").expect("msg").clone());
      data1_lib.cell.extend(data2_lib.cell.into_iter());
      for template_cell in template_lib.cell.iter_mut() {
        let data_cell = data1_lib
          .cell
          .get_mut(&template_cell.name)
          .expect(&format!("{:?}", template_cell.name));
        for template_pin in template_cell.pin.iter_mut() {
          let data_pin = data_cell
            .pin
            .get_mut(template_pin.name.as_ref())
            .expect(&format!("{:?}", template_pin.name.as_ref()));
          template_pin.timing = data_pin.timing.clone();
        }
      }
      let lib_path = "pruned_baseline.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", template_lib)?;
    }
    _ => todo!(),
  }
  Ok(())
}

#[test]
fn lvf_lib() -> anyhow::Result<()> {
  let file_name = "pruned_lvf.lib";
  match liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
    Path::new(file_name),
  )?) {
    Ok(library) => {
      let mut _library = library.clone();
      _library.cell.clear();
      for cell in library.cell {
        let mut c = cell.clone();
        c.pin.clear();
        for pin in cell.pin {
          let mut p = pin.clone();
          p.timing.clear();
          p.output_ccb.clear();
          p.input_ccb.clear();
          p.receiver_capacitance.clear();
          for timing in pin.timing {
            let mut t = liberty_db::timing::Timing::<DefaultCtx>::default();
            t.related_pin = timing.related_pin.clone();
            t.timing_sense = timing.timing_sense.clone();
            t.timing_type = timing.timing_type.clone();
            t.when = timing.when.clone();
            t.sdf_cond = timing.sdf_cond.clone();
            let mut changed = false;
            if let Some(lut) = &timing.cell_rise {
              t.cell_rise = Some(lut.clone());
              changed = true;
            }
            if let Some(lut) = &timing.cell_fall {
              t.cell_fall = Some(lut.clone());
              changed = true;
            }
            if let Some(lut) = &timing.rise_transition {
              t.rise_transition = Some(lut.clone());
              changed = true;
            }
            if let Some(lut) = &timing.fall_transition {
              t.fall_transition = Some(lut.clone());
              changed = true;
            }
            if changed {
              p.timing.insert(t);
            }
          }
          c.pin.insert(p);
        }
        _library.cell.insert(c);
      }
      let lib_path = "lvf.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", _library)?;
    }
    Err(e) => println!("{:?}", e),
  }
  Ok(())
}

#[test]
fn collect() -> anyhow::Result<()> {
  let template_file = "lvf.lib";
  let data1_file = "/code/char0425/baseline1/out/btdcell.lib";
  let data2_file = "/code/char0425/baseline2/out/btdcell.lib";
  match (
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(template_file),
    )?),
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(data1_file),
    )?),
    liberty_db::library::Library::<DefaultCtx>::parse_lib(&std::fs::read_to_string(
      Path::new(data2_file),
    )?),
  ) {
    (Ok(mut template_lib), Ok(mut data1_lib), Ok(data2_lib)) => {
      data1_lib
        .cell
        .insert(template_lib.cell.get("DFCNQD1BWP30P140").expect("msg").clone());
      data1_lib.cell.extend(data2_lib.cell.into_iter());
      for template_cell in template_lib.cell.iter_mut() {
        let data_cell = data1_lib
          .cell
          .get_mut(&template_cell.name)
          .expect(&format!("{:?}", template_cell.name));
        for template_pin in template_cell.pin.iter_mut() {
          let data_pin = data_cell
            .pin
            .get_mut(template_pin.name.as_ref())
            .expect(&format!("{:?}", template_pin.name.as_ref()));
          template_pin.timing = data_pin.timing.clone();
        }
      }
      let lib_path = "pruned_baseline.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", template_lib)?;
    }
    _ => unreachable!(),
  }
  Ok(())
}

fn main() -> anyhow::Result<()> {
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
