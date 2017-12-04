#[macro_use] extern crate failure;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_digit(c: char, radix: u32) -> Result<u32> {
  match c.to_digit(radix) {
    Some(n) => Ok(n),
    None => Err(format_err!("Invalid digit: {}", c.escape_debug())),
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<u32>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  data.chars()
         .filter(|c| !c.is_whitespace())
         .map(|c| to_digit(c, 10))
         .collect::<Result<Vec<_>>>()
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let xs = read_data_file(input)?;
  let steps = xs.len() / 2;
  let mut t = 0;
  for (i, n) in xs.iter().enumerate() {
    let j = (i + steps) % xs.len();
    let m = xs[j];
    if m == *n {
      t += m;
    }
  }
  Ok(format!("{}", t))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
