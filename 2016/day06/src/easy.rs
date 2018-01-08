extern crate failure;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;
use std::usize;

type Result<T> = result::Result<T, failure::Error>;

fn read_data_file<P>(filename: P) -> Result<Vec<String>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data.lines().map(ToOwned::to_owned).collect())
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut counters: Vec<HashMap<char, usize>>  = Vec::new();
  for s in data {
    for (i, ch) in s.chars().enumerate() {
      if counters.len() <= i {
        counters.push(HashMap::new());
      }
      let hm = &mut counters[i];
      if hm.contains_key(&ch) {
        let n = hm.get_mut(&ch).unwrap();
        *n = *n + 1;
      } else {
        hm.insert(ch, 1);
      }
    }
  }
  let mut s = String::new();
  for hm in counters {
    let max = hm.iter()
                .fold((&'~', &usize::MIN),
                      |acc, kv| if kv.1 > &acc.1 { kv } else { acc });
    s.push(*max.0);
  }
  Ok(format!("{}", s))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
