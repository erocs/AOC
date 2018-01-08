extern crate failure;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn read_data_file<P>(filename: P) -> Result<Vec<Vec<char>>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data.lines().map(|s| s.chars().collect::<Vec<char>>()).collect())
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut count = 0;
  for s in &data {
    if s.len() < 8 {
      continue;
    }
    let mut aba_seq = HashSet::new();
    let mut bab_seq = HashSet::new();
    let mut in_hynseq = false;
    for chx in s[..].windows(3) {
      if chx[0] == '[' || chx[0] == ']' {
        if chx[0] == '[' {
          in_hynseq = true;
        } else if chx[0] == ']' {
          in_hynseq = false;
        }
      } else if chx[0] == chx[2] && chx[0] != chx[1] {
        if in_hynseq {
          bab_seq.insert(format!("{1}{0}{1}", chx[0], chx[1]));
        } else {
          aba_seq.insert(format!("{0}{1}{0}", chx[0], chx[1]));
        }
      }
    }
    if aba_seq.intersection(&bab_seq).count() > 0 {
      count += 1;
    }
  }
  Ok(format!("{}", count))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
