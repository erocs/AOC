#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Write};
use std::fs::File;
use std::i64;
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

fn to_bit_vec(hex_str: &str) -> Result<Vec<i64>> {
  let mut res = Vec::with_capacity(hex_str.len() * 4);
  for i in 0..hex_str.len() {
    let n = i64::from_str_radix(&hex_str[i..(i+1)], 16)?;
    res.push(i64::MAX * ((n >> 3) & 1));
    res.push(i64::MAX * ((n >> 2) & 1));
    res.push(i64::MAX * ((n >> 1) & 1));
    res.push(i64::MAX * (n & 1));
  }
  Ok(res)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
  x: i64,
  y: i64,
}

impl Coord {
  fn new(x: i64, y: i64) -> Coord {
    Coord {
      x: x,
      y: y,
    }
  }

  fn oob(c: &Coord) -> bool {
    c.x < 0 || c.y < 0 || c.x >= 128 || c.y >= 128
  }

  fn add(&self, dx: i64, dy: i64) -> Option<Coord> {
    let new_coord = Coord::new(self.x + dx, self.y + dy);
    if Coord::oob(&new_coord) {
      None
    } else {
      Some(new_coord)
    }
  }

  fn ux(&self) -> usize {
    self.x as usize
  }

  fn uy(&self) -> usize {
    self.y as usize
  }
}

const DIRS: &[(i64, i64)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn mark_group(bit_arr: &Vec<RefCell<Vec<i64>>>, i: i64, j: i64, id: i64) {
  let mut q = VecDeque::new();
  let mut hs = HashSet::new();
  {
    let coord = Coord::new(i, j);
    hs.insert(coord.clone());
    q.push_back(coord);
  }
  while !q.is_empty() {
    let coord = q.pop_back().unwrap();
    {
      let mut row = bit_arr[coord.ux()].borrow_mut();
      if row[coord.uy()] != i64::MAX {
        continue;
      }
      row[coord.uy()] = id;
    }
    for dir in DIRS {
      if let Some(new_coord) = coord.add(dir.0, dir.1) {
        if !hs.contains(&new_coord) {
          hs.insert(new_coord.clone());
          q.push_back(new_coord);
        }
      }
    }
  }
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut bit_arr: Vec<RefCell<Vec<i64>>> = Vec::with_capacity(128);
  for i in 0..128 {
    let hash = knot_hash(&format!("{}-{}", data, i))?;
    bit_arr.push(RefCell::new(to_bit_vec(&hash)?));
  }
  let mut next_id = 1;
  for i in 0..128 {
    for j in 0..128 {
      let cell_val = { bit_arr[i].borrow()[j] };
      if cell_val == i64::MAX {
        mark_group(&bit_arr, i as i64, j as i64, next_id);
        next_id += 1;
      }
    }
  }
  Ok(format!("{}", next_id - 1))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
