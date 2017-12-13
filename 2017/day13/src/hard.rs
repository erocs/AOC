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

#[derive(Debug)]
struct Scanner {
  depth: i64,
  range: i64,
  period: i64,
}

impl Scanner {
  fn new(depth: i64, range: i64) -> Scanner {
    Scanner {
      depth: depth,
      range: range,
      period: (range-1) * 2,
    }
  }

  fn is_at0(&self, time: i64) -> bool {
    (time % self.period) == 0
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<Scanner>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  for line in data.lines() {
    let mut toks =
        line.split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
            .map(|s| to_i64(&s));
    let depth = toks.next().ok_or(format_err!("Missing depth: {}", line))??;
    let range = toks.next().ok_or(format_err!("Missing range: {}", line))??;
    res.push(Scanner::new(depth, range));
  }
  res.sort_by(|a, b| a.depth.cmp(&b.depth));
  Ok(res)
}

fn find_max_depth(scanners: &Vec<Scanner>) -> i64 {
  let mut max = i64::MIN;
  for scanner in scanners {
    max = i64::max(max, scanner.depth);
  }
  max
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let scanners = read_data_file(input)?;
  let max_depth = find_max_depth(&scanners) + 1;
  let mut start = 0;
  loop {
    let mut caught = false;
    let mut idx = 0;
    for i in start..(start + max_depth) {
      if scanners[idx].depth != (i - start) {
        continue;
      }
      let j = idx;
      idx += 1;
      if scanners[j].is_at0(i) {
        caught = true;
        start += 1;
        break;
      }
    }
    if !caught {
      break;
    }
  }
  Ok(format!("{}", start))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
