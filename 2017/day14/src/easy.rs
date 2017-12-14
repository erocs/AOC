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

fn read_data_file<P>(filename: P) -> Result<String>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let line = data.lines()
                 .next()
                 .ok_or(format_err!("No input line"))?;
  Ok(line.to_owned())
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

fn knot_hash(input: &str) -> Result<String> {
  let mut data: Vec<u8> = input.as_bytes().to_vec();
  data.extend(vec![17, 31, 73, 47, 23]);
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

fn count_bits(hex_str: &str) -> Result<i64> {
  let mut total = 0;
  for i in 0..hex_str.len() {
    let n = i64::from_str_radix(&hex_str[i..(i+1)], 16)?;
    total += (n >> 3) & 1;
    total += (n >> 2) & 1;
    total += (n >> 1) & 1;
    total += n & 1;
  }
  Ok(total)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut total = 0;
  for i in 0..128 {
    let hash = knot_hash(&format!("{}-{}", data, i))?;
    total += count_bits(&hash)?;
  }
  Ok(format!("{}", total))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
