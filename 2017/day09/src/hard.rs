extern crate failure;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn read_data_file<P>(filename: P) -> Result<String>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut in_garbage = false;
  let mut skip_next = false;
  let mut score = 0;
  for ch in data.chars() {
    if skip_next {
      skip_next = false;
      continue;
    }
    if in_garbage {
      match ch {
        '!' => skip_next = true,
        '>' => in_garbage = false,
        _ => score += 1,
      }
    } else {
      match ch {
        '{' => {},
        '}' => {},
        '<' => in_garbage = true,
        _ => {},
      }
    }
  }
  Ok(format!("{}", score))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
