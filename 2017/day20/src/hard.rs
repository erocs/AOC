#[macro_use] extern crate failure;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

const SETTLING_COUNT: i64 = 10000;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from '{}'", s)),
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
  x: i64,
  y: i64,
  z: i64,
}

impl Vector {
  fn new(x: i64, y: i64, z: i64) -> Vector {
    Vector { x: x, y: y, z: z }
  }

  fn dist(&self) -> i64 {
    i64::abs(self.x) + i64::abs(self.y) + i64::abs(self.z)
  }

  fn add(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

#[derive(Debug)]
struct Point {
  loc: Vector,
  vel: Vector,
  acc: Vector,
  deleted: bool,
}

impl Point {
  fn new(loc: Vector, vel: Vector, acc: Vector) -> Point {
    Point { loc: loc, vel: vel, acc: acc, deleted: false }
  }

  fn from_captures(captures: &regex::Captures) -> Result<Point> {
    if captures.len() < 10 {
      return Err(format_err!("Insufficient captures: {}", captures.len()));
    }
    Ok(Point::new(
      Vector::new(
        to_i64(captures.get(1).map_or("bad1", |m| m.as_str()))?,
        to_i64(captures.get(2).map_or("bad2", |m| m.as_str()))?,
        to_i64(captures.get(3).map_or("bad3", |m| m.as_str()))?),
      Vector::new(
        to_i64(captures.get(4).map_or("bad4", |m| m.as_str()))?,
        to_i64(captures.get(5).map_or("bad5", |m| m.as_str()))?,
        to_i64(captures.get(6).map_or("bad6", |m| m.as_str()))?),
      Vector::new(
        to_i64(captures.get(7).map_or("bad7", |m| m.as_str()))?,
        to_i64(captures.get(8).map_or("bad8", |m| m.as_str()))?,
        to_i64(captures.get(9).map_or("bad9", |m| m.as_str()))?)))
  }

  fn advance(&mut self) {
    self.vel.add(&self.acc);
    self.loc.add(&self.vel);
  }

  fn is_deleted(&self) -> bool {
    self.deleted
  }

  fn set_deleted(&mut self) {
    self.deleted = true;
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<Point>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
  for cap in re.captures_iter(&data) {
    res.push(Point::from_captures(&cap)?);
  }
  Ok(res)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut pts = read_data_file(input)?;
  let mut closest_id = i64::MAX;
  let mut count = SETTLING_COUNT;
  loop {
    let mut collissions: HashMap<Vector, Vec<usize>> = HashMap::new();
    let mut closest_dist = i64::MAX;
    let mut now_closest_id = i64::MAX;
    for i in 0..pts.len() {
      let pt = &mut pts[i];
      if pt.is_deleted() {
        continue;
      }
      pt.advance();
      if collissions.contains_key(&pt.loc) {
        // https://github.com/rust-lang/rust/issues/6393
        if let Some(v) = collissions.get_mut(&pt.loc) {
          v.push(i);
        }
      } else {
        let mut v = Vec::new();
        v.push(i);
        collissions.insert(pt.loc.clone(), v);
      }
      let dist = pt.loc.dist();
      if dist < closest_dist {
        closest_dist = dist;
        now_closest_id = i as i64;
      }
    }
    if closest_id == now_closest_id {
      count -= 1;
      if count <= 0 {
        break;
      }
    } else {
      count = SETTLING_COUNT;
      closest_id = now_closest_id;
    }
    for vs in collissions.values().filter(|v| v.len() > 1) {
      for idx in vs {
        pts[*idx].set_deleted();
      }
    }
  }
  let count = pts.iter().filter(|pt| !pt.is_deleted()).count();
  Ok(format!("{}", count))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
