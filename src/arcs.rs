use std::{
  collections::{BinaryHeap, HashMap, HashSet},
  fs::{self, File},
  io::{BufWriter, Write},
  path::Path,
  str::FromStr,
};

use liberty_db::{
  common::items::{TableLookUp, WordSet},
  timing::{
    items::TimingSenseType,
    items::TimingSenseType::{NegativeUnate, PositiveUnate},
    Timing, TimingType,
  },
  Cell, Pin,
};

const INFO: [(&str, &str, &str, &str, &str, &str, bool, TimingSenseType); 62] = [
  ("AN2", "AN2D1BWP30P140", "Z", "A2", "001", "", true, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A1", "002", "", true, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A1", "003", "", true, PositiveUnate),
  ("AN2", "AN2D1BWP30P140", "Z", "A2", "004", "", true, PositiveUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A1", "001", "", true, PositiveUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A2", "002", "", true, PositiveUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A1", "003", "", true, PositiveUnate),
  ("ND2", "ND2D1BWP30P140", "ZN", "A2", "004", "", true, PositiveUnate),
  ("INV", "INVD1BWP30P140", "ZN", "I", "01", "", true, PositiveUnate),
  ("INV", "INVD1BWP30P140", "ZN", "I", "02", "", true, PositiveUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A1", "001", "", true, PositiveUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A2", "002", "", true, PositiveUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A2", "003", "", true, PositiveUnate),
  ("NR2", "NR2D1BWP30P140", "ZN", "A1", "004", "", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A2", "001", "", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A1", "002", "", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A1", "003", "", true, PositiveUnate),
  ("OR2", "OR2D1BWP30P140", "Z", "A2", "004", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "001", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "002", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "003", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "004", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "005", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "006", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A2", "007", "", true, PositiveUnate),
  ("XNR2", "XNR2D1BWP30P140", "ZN", "A1", "008", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "001", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "002", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "003", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "004", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "005", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "006", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A2", "007", "", true, PositiveUnate),
  ("XOR2", "XOR2D1BWP30P140", "Z", "A1", "008", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "001", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A1", "002", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A2", "003", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A1", "004", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "005", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "006", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "A2", "007", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "008", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "009", "", true, PositiveUnate),
  ("AOI21", "AOI21D1BWP30P140", "ZN", "B", "010", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "001", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "002", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "003", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A1", "004", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A2", "005", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "006", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A2", "007", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "008", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "A1", "009", "", true, PositiveUnate),
  ("OAI21", "OAI21D1BWP30P140", "ZN", "B", "010", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "CO", "B", "001", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "S", "B", "002", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "CO", "A", "003", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "S", "A", "004", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "CO", "A", "005", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "S", "A", "006", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "CO", "B", "007", "", true, PositiveUnate),
  ("HA1", "HA1D1BWP30P140", "S", "B", "008", "", true, PositiveUnate),
];

#[test]
fn collect() -> anyhow::Result<()> {
  struct Arc {
    nominal: TableLookUp,
    mean_shift: TableLookUp,
    std_dev: TableLookUp,
    skewness: TableLookUp,
  }
  let template_file = "pruned_golden.lib";
  match liberty_db::library::Library::parse(&std::fs::read_to_string(Path::new(
    template_file,
  ))?) {
    Ok(mut template_lib) => {
      for (cell_group, cell, pin, related_pin, arc_num, when, is_rise, timing_sense) in
        INFO
      {
        let when = if when == "" {
          None
        } else {
          Some(liberty_db::expression::BooleanExpression::from_str(when)?.into())
        };
        let timing = template_lib
          .cell
          .get_mut(&Cell::new_id(cell.to_string()))
          .expect("msg_cell")
          .pin
          .get_mut(&Pin::new_id(pin.to_string()))
          .expect("msg_pin")
          .timing
          .get_mut(&Timing::new_id(
            WordSet { inner: HashSet::from([related_pin.to_string()]) },
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
              mean_shift: timing
                .ocv_mean_shift_rise_transition
                .clone()
                .expect("msg_table"),
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
              mean_shift: timing
                .ocv_mean_shift_fall_transition
                .clone()
                .expect("msg_table"),
              std_dev: timing.ocv_std_dev_fall_transition.clone().expect("msg_table"),
              skewness: timing.ocv_skewness_fall_transition.clone().expect("msg_table"),
            },
          )
        };
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
            ) = (v[0], v[1], v[2], v[3], v[4], v[5]);
            delay_arc.mean_shift.values.inner[index] =
              delay_mean - delay_arc.nominal.values.inner[index];
            delay_arc.std_dev.values.inner[index] = delay_std_dev;
            delay_arc.skewness.values.inner[index] = delay_skewness;
            transition_arc.mean_shift.values.inner[index] =
              transition_mean - transition_arc.nominal.values.inner[index];
            transition_arc.std_dev.values.inner[index] = transition_std_dev;
            transition_arc.skewness.values.inner[index] = transition_skewness;
          } else {
            println!("{cell_group} {cell} {pin} {related_pin} {arc_num} {index}");
          }
        }
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
      }
      let lib_path = "pruned_active_lvf.lib";
      let mut writer = BufWriter::new(File::create(lib_path)?);
      write!(&mut writer, "{}", template_lib)?;
    }
    Err(_) => todo!(),
  }
  Ok(())
}
