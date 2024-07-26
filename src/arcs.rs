use liberty_db::{
  ast::GroupAttri,
  common::items::WordSet,
  common::table::TableLookUp,
  timing::{
    items::TimingSenseType,
    items::TimingSenseType::{NegativeUnate, PositiveUnate},
    Timing, TimingType,
  },
  Cell, Pin,
};
use ordered_float::NotNan;
use std::{
  collections::{BinaryHeap, HashMap, HashSet},
  fs::{self, File},
  io::{BufWriter, Write},
  path::Path,
  str::FromStr,
};

const INFO: [(&str, &str, &str, &str, &str, &str, bool, TimingSenseType); 54] = [
  ("AN2", "AN2D1BWP30P140", "Z", "A2", "001", "A1", true, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A1", "002", "A2", true, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A1", "003", "A2", false, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A2", "004", "A1", false, PositiveUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A1", "001", "A2", false, NegativeUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A2", "002", "A1", true, NegativeUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A1", "003", "A1", true, NegativeUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A2", "004", "A2", false, NegativeUnate),
  ("INV", "INVD1BWP30P140", "ZN", "I", "01", "", true, NegativeUnate),
  ("INV", "INVD1BWP30P140", "ZN", "I", "02", "", false, NegativeUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A1", "001", "!A2", true, NegativeUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A2", "002", "!A1", true, NegativeUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A2", "003", "!A1", false, NegativeUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A1", "004", "!A2", false, NegativeUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A2", "001", "!A1", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A1", "002", "!A2", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A1", "003", "!A2", false, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A2", "004", "!A1", false, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "001", "A2", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "002", "A1", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "003", "!A1", false, NegativeUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "004", "!A2", false, NegativeUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "005", "!A2", true, NegativeUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "006", "A1", false, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "007", "!A1", true, NegativeUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "008", "A2", false, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "001", "A2", true, NegativeUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "002", "A1", true, NegativeUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "003", "!A2", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "004", "A1", false, NegativeUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "005", "!A1", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "006", "A2", false, NegativeUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "007", "!A1", false, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "008", "!A2", false, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "001", "!A1&A2", false, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A1", "002", "A2&!B", false, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A2", "003", "A1&!B", true, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A1", "004", "A2&!B", true, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "005", "!A1&A2", true, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "006", "A1&!A2", false, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A2", "007", "A1&!B", false, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "008", "!A1&!A2", false, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "009", "A1&!A2", true, NegativeUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "010", "!A1&!A2", true, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "001", "A1&A2", false, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "002", "!A1&A2", false, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "003", "A1&!A2", false, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A1", "004", "!A2&B", false, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A2", "005", "!A1&B", false, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "006", "A1&A2", true, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A2", "007", "!A1&B", true, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "008", "!A1&A2", true, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A1", "009", "!A2&B", true, NegativeUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "010", "A1&!A2", true, NegativeUnate),
];

struct Arc {
  nominal: TableLookUp,
  mean_shift: TableLookUp,
  std_dev: TableLookUp,
  skewness: TableLookUp,
}

fn update_cell(
  info: (&str, &str, &str, &str, &str, &str, bool, TimingSenseType),
  template_lib: &mut liberty_db::Library,
) -> anyhow::Result<()> {
  let (cell_group, cell, pin, related_pin, arc_num, when, is_rise, timing_sense) = info;
  let when = if when == "" {
    None
  } else {
    Some(liberty_db::expression::BooleanExpression::from_str(when)?.into())
  };
  let timing = template_lib
    .cell
    .get_mut(&Cell::new_id(cell.into()))
    .expect("msg_cell")
    .pin
    .get_mut(&Pin::new_id(pin.into()))
    .expect("msg_pin")
    .timing
    .get_mut(&Timing::new_id(
      WordSet { inner: HashSet::from([related_pin.into()]) },
      Some(timing_sense),
      TimingType::COMBINATIONAL,
      when,
    ))
    .expect("msg_timing");

  let (mut delay_arc, mut transition_arc) = if is_rise {
    (
      Arc {
        nominal: timing.cell_rise.clone().expect("msg_table"),
        mean_shift: timing.ocv_mean_shift_cell_rise.clone().expect("msg_table"),
        std_dev: timing.ocv_std_dev_cell_rise.clone().expect("msg_table"),
        skewness: timing.ocv_skewness_cell_rise.clone().expect("msg_table"),
      },
      Arc {
        nominal: timing.rise_transition.clone().expect("msg_table"),
        mean_shift: timing.ocv_mean_shift_rise_transition.clone().expect("msg_table"),
        std_dev: timing.ocv_std_dev_rise_transition.clone().expect("msg_table"),
        skewness: timing.ocv_skewness_rise_transition.clone().expect("msg_table"),
      },
    )
  } else {
    (
      Arc {
        nominal: timing.cell_fall.clone().expect("msg_table"),
        mean_shift: timing.ocv_mean_shift_cell_fall.clone().expect("msg_table"),
        std_dev: timing.ocv_std_dev_cell_fall.clone().expect("msg_table"),
        skewness: timing.ocv_skewness_cell_fall.clone().expect("msg_table"),
      },
      Arc {
        nominal: timing.fall_transition.clone().expect("msg_table"),
        mean_shift: timing.ocv_mean_shift_fall_transition.clone().expect("msg_table"),
        std_dev: timing.ocv_std_dev_fall_transition.clone().expect("msg_table"),
        skewness: timing.ocv_skewness_fall_transition.clone().expect("msg_table"),
      },
    )
  };
  timing.comments._self.push(format!("{cell} {arc_num}").into());
  for index in 0..64 {
    let csv_file = format!("/code/ActiveLVF/char/{cell_group}/tt0p8v25c/{cell}/arc{arc_num}/{index}_moments.csv");
    if Path::new(&csv_file).exists() {
      let s = std::fs::read_to_string(Path::new(&csv_file)).expect("msg1");
      let mut s = s.split("\n");
      s.next();
      let v: Vec<f64> = s
        .next()
        .expect("msg2")
        .split(",")
        .map(|s| f64::from_str(s).expect("msg3"))
        .collect();
      let (
        delay_mean,
        delay_std_dev,
        delay_skewness,
        transition_mean,
        transition_std_dev,
        transition_skewness,
      ) = (v[0] * 1e9, v[1] * 1e9, v[2] * 1e9, v[3] * 1e9, v[4] * 1e9, v[5] * 1e9);
      delay_arc.mean_shift.values.inner[index] =
        NotNan::new(delay_mean).unwrap() - delay_arc.nominal.values.inner[index];
      delay_arc.std_dev.values.inner[index] = NotNan::new(delay_std_dev).unwrap();
      delay_arc.skewness.values.inner[index] = NotNan::new(delay_skewness).unwrap();
      transition_arc.mean_shift.values.inner[index] = NotNan::new(transition_mean)
        .unwrap()
        - transition_arc.nominal.values.inner[index];
      transition_arc.std_dev.values.inner[index] =
        NotNan::new(transition_std_dev).unwrap();
      transition_arc.skewness.values.inner[index] =
        NotNan::new(transition_skewness).unwrap();
    } else {
      println!("{cell_group} {cell} {pin} {related_pin} {arc_num} {index}");
    }
  }
  delay_arc
    .mean_shift
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  delay_arc
    .std_dev
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  delay_arc
    .skewness
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  transition_arc
    .mean_shift
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  transition_arc
    .std_dev
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  transition_arc
    .skewness
    .comments
    ._self
    .push(format!("{cell} {arc_num}").into());
  if is_rise {
    timing.ocv_mean_shift_cell_rise = Some(delay_arc.mean_shift);
    timing.ocv_std_dev_cell_rise = Some(delay_arc.std_dev);
    timing.ocv_skewness_cell_rise = Some(delay_arc.skewness);
    timing.ocv_mean_shift_rise_transition = Some(transition_arc.mean_shift);
    timing.ocv_std_dev_rise_transition = Some(transition_arc.std_dev);
    timing.ocv_skewness_rise_transition = Some(transition_arc.skewness);
  } else {
    timing.ocv_mean_shift_cell_fall = Some(delay_arc.mean_shift);
    timing.ocv_std_dev_cell_fall = Some(delay_arc.std_dev);
    timing.ocv_skewness_cell_fall = Some(delay_arc.skewness);
    timing.ocv_mean_shift_fall_transition = Some(transition_arc.mean_shift);
    timing.ocv_std_dev_fall_transition = Some(transition_arc.std_dev);
    timing.ocv_skewness_fall_transition = Some(transition_arc.skewness);
  }
  Ok(())
}

#[test]
fn collect_by_cell() -> anyhow::Result<()> {
  let template_file = "pruned_100kMC.lib";
  let mut map: HashMap<
    &str,
    Vec<&(&str, &str, &str, &str, &str, &str, bool, TimingSenseType)>,
  > = HashMap::new();
  for info in INFO.iter() {
    let cell = info.1;
    match map.get_mut(cell) {
      Some(v) => {
        v.push(info);
      }
      None => {
        map.insert(cell, vec![info]);
      }
    }
  }
  for (cell_name, info_list) in map.iter() {
    match liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(
      template_file,
    ))?) {
      Ok(mut template_lib) => {
        for info in info_list {
          update_cell(**info, &mut template_lib)?;
        }
        let lib_path = format!("pruned_active_lvf_{cell_name}.lib");
        let mut writer = BufWriter::new(File::create(lib_path)?);
        write!(&mut writer, "{}", template_lib)?;
      }
      Err(_) => todo!(),
    }
  }
  Ok(())
}

#[test]
fn collect() -> anyhow::Result<()> {
  let template_file = "pruned_100kMC.lib";
  match liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(
    template_file,
  ))?) {
    Ok(mut template_lib) => {
      for info in INFO {
        update_cell(info, &mut template_lib)?;
      }
      let lib_path = "pruned_active_lvf_0503.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", template_lib)?;
    }
    Err(_) => todo!(),
  }
  Ok(())
}

#[test]
fn replace_timing_5kQMC() -> anyhow::Result<()> {
  let template_file = "pruned_active_lvf.lib";
  let data1_file = "/code/char0425/5kQMC_1/out/btdcell.lib";
  let data2_file = "/code/char0425/5kQMC_2/out/btdcell.lib";
  match (
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(
      template_file,
    ))?),
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(data1_file))?),
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(data2_file))?),
  ) {
    (Ok(mut temp_lib), Ok(data1_lib), Ok(data2_lib)) => {
      for data_lib in [data1_lib, data2_lib] {
        for cell in data_lib.cell.into_iter() {
          if cell.name == "HA1D1BWP30P140" {
            continue;
          }
          let temp_cell = temp_lib.cell.get_mut(cell.id()).expect("cell");
          for pin in cell.pin.into_iter() {
            let temp_pin = temp_cell.pin.get_mut(pin.id()).expect("pin");
            for timing in pin.timing.into_iter() {
              let temp_timing = temp_pin.timing.get_mut(timing.id()).expect("timing");
              temp_timing.cell_fall = timing.cell_fall;
              temp_timing.cell_rise = timing.cell_rise;
              temp_timing.rise_transition = timing.rise_transition;
              temp_timing.fall_transition = timing.fall_transition;
              temp_timing.ocv_mean_shift_cell_fall = timing.ocv_mean_shift_cell_fall;
              temp_timing.ocv_mean_shift_cell_rise = timing.ocv_mean_shift_cell_rise;
              temp_timing.ocv_mean_shift_rise_transition =
                timing.ocv_mean_shift_rise_transition;
              temp_timing.ocv_mean_shift_fall_transition =
                timing.ocv_mean_shift_fall_transition;
              temp_timing.ocv_std_dev_cell_fall = timing.ocv_std_dev_cell_fall;
              temp_timing.ocv_std_dev_cell_rise = timing.ocv_std_dev_cell_rise;
              temp_timing.ocv_std_dev_rise_transition =
                timing.ocv_std_dev_rise_transition;
              temp_timing.ocv_std_dev_fall_transition =
                timing.ocv_std_dev_fall_transition;
              temp_timing.ocv_skewness_cell_fall = timing.ocv_skewness_cell_fall;
              temp_timing.ocv_skewness_cell_rise = timing.ocv_skewness_cell_rise;
              temp_timing.ocv_skewness_rise_transition =
                timing.ocv_skewness_rise_transition;
              temp_timing.ocv_skewness_fall_transition =
                timing.ocv_skewness_fall_transition;
            }
          }
        }
      }
      let lib_path = "pruned_5kQMC.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", temp_lib)?;
    }
    (Ok(_), Ok(_), Err(_)) => todo!(),
    (Ok(_), Err(_), Ok(_)) => todo!(),
    (Ok(_), Err(_), Err(_)) => todo!(),
    (Err(_), Ok(_), Ok(_)) => todo!(),
    (Err(_), Ok(_), Err(_)) => todo!(),
    (Err(_), Err(_), Ok(_)) => todo!(),
    (Err(_), Err(_), Err(_)) => todo!(),
  }
  Ok(())
}

#[test]
fn replace_timing_100kMC() -> anyhow::Result<()> {
  let template_file = "pruned_active_lvf.lib";
  let data1_file = "/code/char0425/100kMC_1/out/btdcell.lib";
  let data2_file = "/code/char0425/100kMC_2/out/btdcell.lib";
  match (
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(
      template_file,
    ))?),
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(data1_file))?),
    liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(data2_file))?),
  ) {
    (Ok(mut temp_lib), Ok(data1_lib), Ok(data2_lib)) => {
      for data_lib in [data1_lib, data2_lib] {
        for cell in data_lib.cell.into_iter() {
          if cell.name == "HA1D1BWP30P140" {
            continue;
          }
          let temp_cell = temp_lib.cell.get_mut(cell.id()).expect("cell");
          for pin in cell.pin.into_iter() {
            let temp_pin = temp_cell.pin.get_mut(pin.id()).expect("pin");
            for timing in pin.timing.into_iter() {
              let temp_timing = temp_pin.timing.get_mut(timing.id()).expect("timing");
              temp_timing.cell_fall = timing.cell_fall;
              temp_timing.cell_rise = timing.cell_rise;
              temp_timing.rise_transition = timing.rise_transition;
              temp_timing.fall_transition = timing.fall_transition;
              temp_timing.ocv_mean_shift_cell_fall = timing.ocv_mean_shift_cell_fall;
              temp_timing.ocv_mean_shift_cell_rise = timing.ocv_mean_shift_cell_rise;
              temp_timing.ocv_mean_shift_rise_transition =
                timing.ocv_mean_shift_rise_transition;
              temp_timing.ocv_mean_shift_fall_transition =
                timing.ocv_mean_shift_fall_transition;
              temp_timing.ocv_std_dev_cell_fall = timing.ocv_std_dev_cell_fall;
              temp_timing.ocv_std_dev_cell_rise = timing.ocv_std_dev_cell_rise;
              temp_timing.ocv_std_dev_rise_transition =
                timing.ocv_std_dev_rise_transition;
              temp_timing.ocv_std_dev_fall_transition =
                timing.ocv_std_dev_fall_transition;
              temp_timing.ocv_skewness_cell_fall = timing.ocv_skewness_cell_fall;
              temp_timing.ocv_skewness_cell_rise = timing.ocv_skewness_cell_rise;
              temp_timing.ocv_skewness_rise_transition =
                timing.ocv_skewness_rise_transition;
              temp_timing.ocv_skewness_fall_transition =
                timing.ocv_skewness_fall_transition;
            }
          }
        }
      }
      let lib_path = "pruned_100kMC.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", temp_lib)?;
    }
    (Ok(_), Ok(_), Err(_)) => todo!(),
    (Ok(_), Err(_), Ok(_)) => todo!(),
    (Ok(_), Err(_), Err(_)) => todo!(),
    (Err(_), Ok(_), Ok(_)) => todo!(),
    (Err(_), Ok(_), Err(_)) => todo!(),
    (Err(_), Err(_), Ok(_)) => todo!(),
    (Err(_), Err(_), Err(_)) => todo!(),
  }
  Ok(())
}
