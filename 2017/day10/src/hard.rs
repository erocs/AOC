#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Write};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::ops::Deref;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

const LIST_SIZE: usize = 256;

fn read_data_file<P>(filename: P) -> Result<Vec<u8>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let line = data.lines()
                 .next()
                 .ok_or(format_err!("No input line"))?;
  let mut res = line.as_bytes()
                    .iter()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<u8>>();
  res.push(17);
  res.push(31);
  res.push(73);
  res.push(47);
  res.push(23);
  Ok(res)
}

fn xor_combine(xs: &Vec<u8>, start: usize, len: usize) -> u8 {
  let mut n = xs[start];
  for i in (start+1)..(start+len) {
    n = n ^ xs[i];
  }
  n
}

struct Circular {
  xs_ref: Rc<RefCell<Vec<u8>>>,
}

impl Circular {
  fn new() -> Circular {
    let mut xs = Vec::with_capacity(LIST_SIZE);
    for i in 0..LIST_SIZE {
      xs.push(i as u8);
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

  fn densify(&mut self) {
    let mut res = Vec::new();
    {
      let xs = self.xs_ref.borrow();
      let mut i = 0;
      while i < LIST_SIZE {
        res.push(xor_combine(xs.deref(), i, 16));
        i += 16;
      }
    }
    self.xs_ref = Rc::new(RefCell::new(res));
  }

  fn hex_digest(&self) -> Result<String> {
    let mut s = String::new();
    let xs = self.xs_ref.borrow();
    for &b in xs.deref() {
      write!(&mut s, "{:02X}", b)?;
    }
    Ok(s.to_lowercase())
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
  let mut rope = Circular::new();
  let mut idx = 0;
  let mut skip = 0;
  for _ in 0..64 {
    for n in data.iter() {
      rope.reverse_range(idx, *n as usize);
      idx = advance(idx, *n as usize + skip);
      skip += 1;
    }
  }
  rope.densify();
  Ok(rope.hex_digest()?)
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
