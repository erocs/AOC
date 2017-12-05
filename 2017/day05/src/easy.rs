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

fn read_data_file<P>(filename: P) -> Result<Vec<i64>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut vals = Vec::new();
  for line in data.lines() {
    let mut row = line.split_whitespace()
                      .map(to_i64)
                      .collect::<Result<Vec<_>>>()?;
    vals.append(&mut row);
  }
  Ok(vals)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut jumps = read_data_file(input)?;
  let mut idx: i64 = 0;
  let mut steps = 0;
  loop {
    steps += 1;
    let i = idx as usize;
    idx += jumps[i];
    jumps[i] += 1;
    if idx < 0 || idx >= jumps.len() as i64 {
      break;
    }
  }
  Ok(format!("{}", steps))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
