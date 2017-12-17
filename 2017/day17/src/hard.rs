#[macro_use] extern crate failure;

use std::fs::File;
use std::usize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_usize(s: &str) -> Result<usize> {
  match s.parse::<usize>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse usize from '{}'", s)),
  }
}

fn read_data_file<P>(filename: P) -> Result<usize>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  to_usize(data.lines()
               .next().ok_or(format_err!("No input lines found"))?)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let cycle = read_data_file(input)?;
  let mut n_at_1 = 0;
  let mut idx = 0;
  let mut lst_len = 1;
  for i in 1..50_000_001 {
    idx = (idx + cycle) % lst_len + 1;
    if idx == 1 {
      n_at_1 = i;
    }
    lst_len += 1;
  }
  Ok(format!("{}", n_at_1))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
