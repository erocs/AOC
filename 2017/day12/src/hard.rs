#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from: {}", s)),
  }
}

#[derive(Debug, Default)]
struct Chan {
  id: i64,
  directs: Vec<i64>,
}

impl Chan {
  fn new(id: i64, directs: &[i64]) -> Chan {
    Chan {
      id: id,
      directs: directs.iter().map(|n| *n).collect()
    }
  }
}

fn read_data_file<P>(filename: P) -> Result<HashMap<i64, Chan>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = HashMap::new();
  for line in data.lines() {
    let toks =
        line.split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
            .filter(|s| !s.is_empty())
            .map(|s| to_i64(&s))
            .collect::<Result<Vec<i64>>>()?;
    if let Some((id, directs)) = toks.split_first() {
      res.insert(*id, Chan::new(*id, directs));
    }
  }
  Ok(res)
}

fn rc_to_usize<T>(rc: &Rc<T>) -> usize {
  let rc_clone = Rc::clone(rc);
  Rc::into_raw(rc_clone) as usize
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let chans = read_data_file(input)?;
  let mut buckets: HashMap<i64, Rc<RefCell<HashSet<i64>>>> = HashMap::new();
  for n in chans.keys() {
    let chan = chans.get(n).unwrap();
    let mut new_set: HashSet<i64> = HashSet::new();
    new_set.extend(&chan.directs);
    new_set.insert(*n);
    for p in &chan.directs {
      if n == p {
        continue;
      }
      if let Some(p_group_ref) = buckets.get(p) {
        let p_group = p_group_ref.borrow();
        new_set.extend(p_group.iter());
      }
    }
    let new_set_ref = Rc::new(RefCell::new(new_set));
    let new_set = new_set_ref.borrow();
    for q in new_set.iter() {
      buckets.insert(*q, Rc::clone(&new_set_ref));
    }
  }
  let mut group_ids = HashSet::new();
  for n in chans.keys() {
    if let Some(group_ref) = buckets.get(n) {
      let val = rc_to_usize(&group_ref);
      group_ids.insert(val);
    }
  }
  Ok(format!("{}", group_ids.len()))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
