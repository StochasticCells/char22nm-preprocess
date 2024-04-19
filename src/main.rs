use liberty_db::{cell::Cell, GroupSet};
use serde::{Deserialize, Serialize};
use std::{
  fs::{self, File},
  io::{BufWriter, Write},
  path::Path,
  str::FromStr,
};
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Config {
  Name: String,
  Voltage: f32,
  Temperature: f32,
  LibFilePath: String,
  NetListPath: String,
  ModelPath: String,
  ModelSection: String,
  LvfType: String,
  LVFSamplingNum: usize,
  NumCPU: usize,
  HspicePath: String,
  CellNameList: Vec<String>,
}
const PVT: [(&str, &str, f32, f32); 16] = [
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

const CELL_GROUP: [(&str, (&str, &str, &str, bool), &[&str]); 12] = [
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
  (
    "BUFF",
    ("Z", "I", "", true),
    &[
      "BUFFD0BWP30P140",
      "BUFFD0P7BWP30P140",
      "BUFFD12BWP30P140",
      "BUFFD16BWP30P140",
      "BUFFD1BWP30P140",
      "BUFFD20BWP30P140",
      "BUFFD24BWP30P140",
      "BUFFD2BWP30P140",
      "BUFFD4BWP30P140",
      "BUFFD6BWP30P140",
      "BUFFD8BWP30P140",
    ],
  ),
  (
    "ND2",
    ("Z", "I", "", true),
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
    ("CO", "A", "B&!C", true),
    &["FA1D0BWP30P140", "FA1D1BWP30P140", "FA1D2BWP30P140", "FA1D4BWP30P140"],
  ),
  (
    "HA1",
    ("CO", "A", "", true),
    &["HA1D0BWP30P140", "HA1D1BWP30P140", "HA1D2BWP30P140", "HA1D4BWP30P140"],
  ),
];

const RUN: [(&str, usize, &str); 2] =
  [("golden", 50000, "QmcSample"), ("baseline", 10000, "McSample")];

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
  if let Ok(mut library) =
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(file_name))?)
  {
    let mut _library = library.clone();
    _library.cell.clear();
    for (cell_group, (pin_name, related, when_str, rise), cell_names) in CELL_GROUP {
      let mut cells: GroupSet<Cell> = GroupSet::new();
      let when = if when_str == "" {
        None
      } else {
        Some(liberty_db::expression::BooleanExpression::from_str(when_str)?.into())
      };
      for cell_name in cell_names {
        let mut cell = library.cell.take(&Cell::id(cell_name.to_string())).expect("msg");
        cell.leakage_power.clear();
        for pin in cell.pin.iter_mut() {
          pin.internal_power.clear();
          if pin.name == pin_name {
            pin
              .timing
              .retain(|t| t.related_pin.inner.contains(related) && t.when == when);
            for timing in pin.timing.iter_mut() {
              if rise {
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
              Voltage: v,
              Temperature: t,
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
