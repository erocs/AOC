// vim: et:sts=2:ts=2:sw=2

extern crate bit_vec;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{Captures, Regex};
use std::env;
use std::fs::File;
use std::fmt;
use std::i32;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::result;
use std::usize;

type Result<T> = result::Result<T, failure::Error>;

lazy_static! {
  static ref RECT_RE: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
  static ref ROTATE_ROW_RE: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
  static ref ROTATE_COL_RE: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
}

fn to_usize(s: &str) -> Result<usize> {
  match s.parse::<usize>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse usize from '{}'", s)),
  }
}

trait Op {
  fn run(&self, grid: &mut Grid) -> Result<()>;
}

#[derive(Debug)]
struct Rect {
  w: usize,
  h: usize,
}

impl Rect {
  fn new(w: usize, h: usize) -> Self {
    Rect { w, h }
  }

  fn from<'t>(item: Option<Captures<'t>>) -> Result<Self> {
    if item.is_none() {
      return Err(format_err!("No captures found"));
    }
    let caps = item.unwrap();
    let w = to_usize(caps.get(1).ok_or(format_err!("Missing W for Rect"))?.as_str())?;
    let h = to_usize(caps.get(2).ok_or(format_err!("Missing H for Rect"))?.as_str())?;
    Ok(Self::new(w, h))
  }
}

impl Op for Rect {
  fn run(&self, grid: &mut Grid) -> Result<()> {
    if self.w > 0 && self.h > 0 {
      grid.fill_box(0, 0, self.w - 1, self.h - 1)
    } else {
      Ok(())
    }
  }
}

#[derive(Debug)]
struct RotateRow {
  y: usize,
  n: usize,
}

impl RotateRow {
  fn new(y: usize, n: usize) -> Self {
    RotateRow { y, n }
  }

  fn from<'t>(item: Option<Captures<'t>>) -> Result<Self> {
    if item.is_none() {
      return Err(format_err!("No captures found"));
    }
    let caps = item.unwrap();
    let y = to_usize(caps.get(1).ok_or(format_err!("Missing Y for RotateRow"))?.as_str())?;
    let n = to_usize(caps.get(2).ok_or(format_err!("Missing N for RotateRow"))?.as_str())?;
    Ok(Self::new(y, n))
  }
}

impl Op for RotateRow {
  fn run(&self, grid: &mut Grid) -> Result<()>  {
    grid.rotate_row(self.y, self.n)?;
    Ok(())
  }
}

#[derive(Debug)]
struct RotateCol {
  x: usize,
  n: usize,
}

impl RotateCol {
  fn new(x: usize, n: usize) -> Self {
    RotateCol { x, n }
  }

  fn from<'t>(item: Option<Captures<'t>>) -> Result<Self> {
    if item.is_none() {
      return Err(format_err!("No captures found"));
    }
    let caps = item.unwrap();
    let x = to_usize(caps.get(1).ok_or(format_err!("Missing X for RotateCol"))?.as_str())?;
    let n = to_usize(caps.get(2).ok_or(format_err!("Missing N for RotateCol"))?.as_str())?;
    Ok(Self::new(x, n))
  }
}

impl Op for RotateCol {
  fn run(&self, grid: &mut Grid) -> Result<()>  {
    grid.rotate_col(self.x, self.n)?;
    Ok(())
  }
}

#[derive(Debug)]
struct Grid {
  grid: Vec<bit_vec::BitVec>,
  tmp_row: bit_vec::BitVec,
  tmp_col: bit_vec::BitVec,
  x: usize,
  y: usize,
}

impl Grid {
  fn new(x: usize, y: usize) -> Result<Self> {
    const MAX_VAL: usize = (i32::MAX / 1024) as usize;
    if x > MAX_VAL || y > MAX_VAL {
      Err(format_err!("Dimension too large, x:{} y:{}", x, y))
    } else {
      let mut grid = Vec::new();
      for _ in 0..y {
        grid.push(bit_vec::BitVec::from_elem(x, false));
      }
      let tmp_row = bit_vec::BitVec::from_elem(x, false);
      let tmp_col = bit_vec::BitVec::from_elem(y, false);
      Ok(Grid { grid, tmp_row, tmp_col, x, y })
    }
  }

  fn fill_box(&mut self, top_left_x: usize, top_left_y: usize,
              low_right_x: usize, low_right_y: usize) -> Result<()> {
    if top_left_x >= self.x || low_right_x >= self.x || top_left_x > low_right_x {
      return Err(format_err!("fill_box: X out of bounds: {} {}", top_left_x, low_right_x));
    }
    if top_left_y >= self.y || low_right_y >= self.y || top_left_y > low_right_y {
      return Err(format_err!("fill_box: Y out of bounds: {} {}", top_left_y, low_right_y));
    }
    for i in top_left_y..=low_right_y {
      for j in top_left_x..=low_right_x {
        self.grid[i].set(j, true);
      }
    }
    Ok(())
  }

  fn rotate_row(&mut self, y: usize, n: usize) -> Result<()> {
    if y >= self.y {
      return Err(format_err!("rotate_row: Y out of bounds: {}", y));
    }
    let n = n % self.x;
    if n == 0 {
      return Ok(());
    }
    let new_row = self.grid.get_mut(y)
                      .ok_or(format_err!("Row out of bounds: {}", y))?;
    let old_row = &mut self.tmp_row;
    old_row.clear();
    old_row.union(new_row);
    for i in 0..self.x {
      let idx = (i + n) % self.x;
      new_row.set(idx, old_row.get(i).unwrap());
    }
    Ok(())
  }

  fn rotate_col(&mut self, x: usize, n: usize) -> Result<()> {
    if x >= self.x {
      return Err(format_err!("rotate_col: X out of bounds: {}", x));
    }
    let n = n % self.y;
    if n == 0 {
      return Ok(());
    }
    let old_col = &mut self.tmp_col;
    old_col.clear();
    for i in 0..self.y {
      let row = self.grid.get_mut(i)
                         .ok_or(format_err!("Row out of bounds: {}", i))?;
      let val = row.get(x)
                   .ok_or(format_err!("Col out of bounds: {}", x))?;
      if val {
        old_col.set(i, true);
      }
    }
    for i in 0..self.y {
      let idx = (i + n) % self.y;
      self.grid.get_mut(idx).unwrap().set(x, old_col[i]);
    }
    Ok(())
  }

  fn true_pixel_count(&self) -> usize {
    self.grid.iter()
        .fold(0, |acc, row| acc + row.iter()
            .fold(0, |acc, b| acc + b as usize))
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in &self.grid {
      let s = row.iter().map(|b| if b { "X" } else { " " }).collect::<Vec<_>>().join("");
      writeln!(f, "{}", s)?;
    }
    Ok(())
  }
}

fn read_input<P>(path: P) -> Result<Vec<String>>
    where P: AsRef<Path> + fmt::Debug {
  let f = File::open(&path)?;
  let reader = io::BufReader::new(f);
  Ok(reader.lines().collect::<result::Result<Vec<_>, _>>()?)
}

fn to_op(s: &str) -> Result<Box<Op>> {
  if RECT_RE.is_match(&s) {
    let r = Rect::from(RECT_RE.captures(&s))?;
    //DBG println!("{:?}", r);
    Ok(Box::new(r))
  } else if ROTATE_ROW_RE.is_match(&s) {
    let r = RotateRow::from(ROTATE_ROW_RE.captures(&s))?;
    //DBG println!("{:?}", r);
    Ok(Box::new(r))
  } else if ROTATE_COL_RE.is_match(&s) {
    let r = RotateCol::from(ROTATE_COL_RE.captures(&s))?;
    //DBG println!("{:?}", r);
    Ok(Box::new(r))
  } else {
    Err(format_err!("Bad Op format: {}", s))
  }
}

fn solve<P>(path: P) -> Result<usize>
    where P: AsRef<Path> + fmt::Debug {
  let mut grid = Grid::new(50, 6)?;
  //DBG println!("{:?}", grid);
  for line in read_input(path)? {
    let op = to_op(&line)?;
    op.run(&mut grid)?;
    //DBG println!("{:?}", grid);
  }
  println!("\n{}", grid);
  Ok(grid.true_pixel_count())
}

fn main() -> Result<()> {
  let args = env::args().collect::<Vec<_>>();
  let path = PathBuf::from(args.get(1).map(|s| s.as_str()).unwrap_or("input.txt"));
  println!("Pixel count: {}", solve(path)?);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sm_grid_box_fill() {
    let mut grid = Grid::new(2, 4).unwrap();
    grid.fill_box(0, 0, 1, 3).unwrap();
    assert!(grid.grid.iter().all(|bv| bv.all()));
    assert_eq!(grid.true_pixel_count(), 8);
  }

  #[test]
  fn test_sm_grid_box_quarter_fill() {
    let mut grid = Grid::new(4, 6).unwrap();
    grid.fill_box(1, 1, 2, 4).unwrap();
    assert_eq!(grid.true_pixel_count(), 8);
  }

  #[test]
  fn test_sm_grid_rr() {
    let mut grid = Grid::new(2, 4).unwrap();
    grid.fill_box(0, 0, 0, 0).unwrap();
    grid.rotate_row(0, 3).unwrap();
    assert!(grid.grid[0].get(1).unwrap());
    assert_eq!(grid.true_pixel_count(), 1);
  }

  #[test]
  fn test_sm_grid_rc() {
    let mut grid = Grid::new(2, 4).unwrap();
    grid.fill_box(0, 0, 0, 0).unwrap();
    grid.rotate_col(0, 6).unwrap();
    assert!(grid.grid[2].get(0).unwrap());
    assert_eq!(grid.true_pixel_count(), 1);
  }

  #[test]
  fn test_sm_grid_rr_100() {
    let mut grid = Grid::new(13, 1).unwrap();
    grid.fill_box(0, 0, 0, 0).unwrap();
    for i in 0..100 {
      grid.rotate_row(0, 1).unwrap();
      assert!(grid.grid[0].get((i+1) % 13).unwrap());
      assert_eq!(grid.true_pixel_count(), 1);
    }
  }

  #[test]
  fn test_sm_grid_rc_100() {
    let mut grid = Grid::new(1, 13).unwrap();
    grid.fill_box(0, 0, 0, 0).unwrap();
    for i in 0..100 {
      grid.rotate_col(0, 1).unwrap();
      assert!(grid.grid[(i+1) % 13].get(0).unwrap());
      assert_eq!(grid.true_pixel_count(), 1);
    }
  }
}  // mod tests
