extern crate failure;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn read_data_file<P>(filename: P) -> Result<Vec<HashMap<String, i32>>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut lines = Vec::new();
  for line in data.lines() {
    let words = line.split_whitespace()
                    .fold(HashMap::new(), |mut acc, s| {
                           *acc.entry(s.to_string()).or_insert(0) += 1; acc
                         });
    lines.push(words);
  }
  Ok(lines)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let lines = read_data_file(input)?;
  let total = lines.iter().map(|ss| ss.values().any(|n| *n > 1)).filter(|b| !*b).count();
  Ok(format!("{}", total))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
