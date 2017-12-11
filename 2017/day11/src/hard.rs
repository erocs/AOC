#[macro_use] extern crate failure;

use std::default::Default;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

// https://www.redblobgames.com/grids/hexagons/
#[derive(Debug, Default)]
struct HexAxialCoord {
  q: i64,
  r: i64,
}

impl HexAxialCoord {
  fn new() -> HexAxialCoord {
    Default::default()
  }

  fn from(q: i64, r: i64) -> HexAxialCoord {
    HexAxialCoord {
      q: q,
      r: r,
    }
  }

  fn to_vec(dir: &str) -> Result<HexAxialCoord> {
    // TODO: Make these static to reduce overhead.
    match dir {
      "n" => Ok(HexAxialCoord::from(0, -1)),
      "s" => Ok(HexAxialCoord::from(0, 1)),
      "nw" => Ok(HexAxialCoord::from(-1, 0)),
      "ne" => Ok(HexAxialCoord::from(1, -1)),
      "sw" => Ok(HexAxialCoord::from(-1, 1)),
      "se" => Ok(HexAxialCoord::from(1, 0)),
      _ => Err(format_err!("Unknown direction: {}", dir)),
    }
  }

  fn step_dir(&mut self, dir: &str) -> Result<()> {
    let vec = HexAxialCoord::to_vec(dir)?;
    self.q += vec.q;
    self.r += vec.r;
    Ok(())
  }

  fn distance_from(&self, other: &HexAxialCoord) -> i64 {
    (  i64::abs(self.q - other.q)
     + i64::abs(self.q + self.r - other.q - other.r)
     + i64::abs(self.r - other.r)) / 2
  }
}

impl Display for HexAxialCoord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{})", self.q, self.r)
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<String>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  for s in data.split_whitespace() {
    let mut ss = s.split(',')
                  .map(ToOwned::to_owned)
                  .collect::<Vec<_>>();
    res.append(&mut ss);
  }
  Ok(res)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let directions = read_data_file(input)?;
  let zero = HexAxialCoord::new();
  let mut hex = HexAxialCoord::new();
  let mut max = 0;
  for dir in directions {
    hex.step_dir(&dir)?;
    max = i64::max(max, hex.distance_from(&zero));
  }
  Ok(format!("{}", max))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
