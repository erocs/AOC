extern crate failure;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;
use std::usize;

type Result<T> = result::Result<T, failure::Error>;
type GridType = HashMap<Vector, char>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
  x: i64,
  y: i64,
}

impl Vector {
  fn new(x: i64, y: i64) -> Vector {
    Vector { x: x, y: y }
  }

  fn add(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
  }
}

fn read_data_file<P>(filename: P) -> Result<(GridType, usize, usize)>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = HashMap::new();
  let mut max_x = 0;
  let mut max_y = 0;
  for (y, line) in data.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      if ch == '#' {
        res.insert(Vector::new(x as i64, y as i64), ch);
      }
      max_x = usize::max(max_x, x);
    }
    max_y = usize::max(max_y, y);
  }
  Ok((res, max_x, max_y))
}

static DIRS: &[Vector] = &[Vector{x:1, y:0}, Vector{x:0, y: 1}, Vector{x:-1, y:0}, Vector{x:0, y:-1}];

fn to_left(dir: usize) -> usize {
  if dir <= 0 {
    DIRS.len() - 1
  } else {
     dir - 1
  }
}

fn to_right(dir: usize) -> usize {
  (dir + 1) % DIRS.len()
}

fn get_grid(grid: &GridType, pos: &Vector) -> char {
  if let Some(ch) = grid.get(&pos) {
    *ch
  } else {
    '.'
  }
}

fn set_grid(grid: &mut GridType, pos: &Vector, ch: char) {
  if grid.contains_key(&pos) {
    let v = grid.get_mut(&pos).unwrap();
    *v = ch;
  } else {
    grid.insert(pos.clone(), ch);
  }
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let (mut grid, max_x, max_y) = read_data_file(input)?;
  let mut infection = 0;
  let mut pos = Vector::new((max_x / 2) as i64, (max_y / 2) as i64);
  let mut dir: usize = 3;
  for _ in 0..10_000 {
    let ch = get_grid(&grid, &pos);
    if ch == '#' {
      // Infected -> Clean
      dir = to_right(dir);
      set_grid(&mut grid, &pos, '.');
    } else {
      // Clean -> Infected
      dir = to_left(dir);
      set_grid(&mut grid, &pos, '#');
      infection += 1;
    }
    pos.add(&DIRS[dir]);
  }
  Ok(format!("{}", infection))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
