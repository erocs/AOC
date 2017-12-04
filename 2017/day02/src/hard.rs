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

fn read_data_file<P>(filename: P) -> Result<Vec<Vec<i64>>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut rows = Vec::new();
  for line in data.lines() {
    let row = line.split_whitespace()
                  .map(to_i64)
                  .collect::<Result<Vec<_>>>()?;
    rows.push(row);
  }
  Ok(rows)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let rows = read_data_file(input)?;
  let mut checksum = 0;
  for row in rows {
    for (i, n) in row.iter().enumerate() {
      for j in (i + 1)..row.len() {
        let m = row[j];
        if n % m == 0 {
          checksum += n / m;
          break;
        } else if m % n == 0 {
          checksum += m / n;
          break;
        }
      }
    }
  }
  Ok(format!("{}", checksum))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
