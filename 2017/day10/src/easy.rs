#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

const LIST_SIZE: usize = 256;

fn to_usize(s: &str) -> Result<usize> {
  match s.parse::<usize>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse usize from: {}", s)),
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<usize>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  for s in data.split_whitespace() {
    let mut ss = s.split(',')
                  .map(to_usize)
                  .collect::<Result<Vec<usize>>>()?;
    res.append(&mut ss);
  }
  Ok(res)
}

struct Circular {
  xs_ref: Rc<RefCell<Vec<usize>>>,
}

impl Circular {
  fn new() -> Circular {
    let mut xs = Vec::with_capacity(LIST_SIZE);
    for i in 0..LIST_SIZE {
      xs.push(i);
    }
    Circular {
      xs_ref: Rc::new(RefCell::new(xs)),
    }
  }

  fn reverse_range(&self, start: usize, len: usize) {
    let mut xs = self.xs_ref.borrow_mut();
    let mut indexes = Vec::new();
    for i in start..(start + len) {
      indexes.push(i % LIST_SIZE);
    }
    for i in 0..indexes.len() / 2 {
      let j = indexes.len() - i - 1;
      xs.swap(indexes[i], indexes[j]);
    }
  }

  fn get(&self, idx: usize) -> usize {
    let xs = self.xs_ref.borrow();
    xs[idx]
  }
}

impl Display for Circular {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let xs = self.xs_ref.borrow();
    write!(f, "{:?}", xs)
  }
}

fn advance(cur: usize, len: usize) -> usize {
  (cur + (len % LIST_SIZE)) % LIST_SIZE
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let rope = Circular::new();
  let mut idx = 0;
  let mut skip = 0;
  for n in data {
    rope.reverse_range(idx, n);
    idx = advance(idx, n + skip);
    skip += 1;
  }
  Ok(format!("{}", rope.get(0) * rope.get(1)))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
