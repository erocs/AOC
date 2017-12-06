#[macro_use] extern crate failure;

use std::collections::HashMap;
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

fn format_banks(banks: &Vec<i64>) -> Result<String> {
  use std::io::Cursor;
  use std::io::Write;
  let mut buf = Cursor::new(Vec::<u8>::new());
  let mut sep = "";
  for n in banks {
    write!(&mut buf, "{}{}", sep, n)?;
    sep = ",";
  }
  Ok(String::from_utf8_lossy(buf.get_ref()).into_owned())
}

// Returns index in banks of the largest bank.
fn find_largest(banks: &Vec<i64>) -> usize {
  let mut res_idx = 0;
  for (idx, n) in banks.iter().enumerate() {
    if *n > banks[res_idx] {
      res_idx = idx;
    }
  }
  res_idx
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut banks = read_data_file(input)?;
  let mut seen = HashMap::new();
  let mut count = 1;
  seen.insert(format_banks(&banks)?, count);
  loop {
    let idx = find_largest(&banks);
    let mut n = banks[idx];
    banks[idx] = 0;
    let mut i = idx + 1;
    while n > 0 {
      if i >= banks.len() {
        i = 0;
      }
      banks[i] += 1;
      n -= 1;
      i += 1;
    }
    count += 1;
    let latest = format_banks(&banks)?;
    if seen.contains_key(&latest) {
      let m = seen.get(&latest).unwrap();
      return Ok(format!("{}", count - m));
    }
    seen.insert(latest, count);
  }
  // Unreachable
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
