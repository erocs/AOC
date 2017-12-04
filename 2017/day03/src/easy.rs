#[macro_use] extern crate failure;

use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from: {}", s)),
  }
}

fn read_data_file<P>(filename: P) -> Result<i64>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let row = data.split_whitespace()
                .map(to_i64)
                .collect::<Result<Vec<_>>>()?;
  if row.is_empty() {
    Err(format_err!("Insufficient input data"))
  } else if row[0] < 1 {
    Err(format_err!("Value too small: {}", row[0]))
  } else {
    Ok(row[0])
  }
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let n = read_data_file(input)?;
  if n == 1 {
    return Ok("0".to_string());
  }
  let mut lvl = 0;
  let mut min: i64;
  let mut max = 1;
  let mut side: i64;
  loop {
    lvl += 1;
    side = lvl * 2;
    min = max + 1;
    max = max + side * 4;
    if min <= n && n <= max {
      break;
    }
  }
  let norm = (n - min) % side;
  let mid = side / 2 - 1;
  let steps = (mid - norm).abs() + lvl;
  Ok(format!("{}", steps))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
