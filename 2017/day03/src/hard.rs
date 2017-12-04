#[macro_use] extern crate failure;

use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from: {}", s)),
  }
}

fn read_data_file<P>(filename: P) -> Result<i64>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let row = data.split_whitespace()
                .map(to_i64)
                .collect::<Result<Vec<_>>>()?;
  if row.is_empty() {
    Err(format_err!("Insufficient input data"))
  } else if row[0] < 1 {
    Err(format_err!("Value too small: {}", row[0]))
  } else {
    Ok(row[0])
  }
}

struct Vector {
  dx: i64,
  dy: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
  East,
  North,
  West,
  South,
}

impl Direction {
  fn to_vec(&self) -> Vector {
    match *self {
      Direction::East => Vector{dx: 1, dy: 0},
      Direction::North => Vector{dx: 0, dy: -1},
      Direction::West => Vector{dx: -1, dy: 0},
      Direction::South => Vector{dx: 0, dy: 1},
    }
  }

  fn counter_clockwise(&self) -> Direction {
    match *self {
      Direction::East => Direction::North,
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct SpiralWalker {
  x: i64,
  y: i64,
  center_x: i64,
  center_y: i64,
  side_size: i64,
  dir: Direction,
}

impl SpiralWalker {
  fn new(center_x: i64, center_y: i64) -> SpiralWalker {
    SpiralWalker {
      x: center_x,
      y: center_y,
      center_x: center_x,
      center_y: center_y,
      side_size: 1,
      dir: Direction::East,
    }
  }

  fn advance(&mut self) {
    let d = self.dir.to_vec();
    let x = self.x + d.dx;
    let y = self.y + d.dy;
    let half = self.side_size / 2;
    if x < self.center_x - half ||
       x > self.center_x + half ||
       y < self.center_y - half ||
       y > self.center_y + half {
      if self.dir == Direction::East {
        // Advancement beyond current square. Expand to next level.
        self.side_size += 2;
        self.dir = self.dir.counter_clockwise();
      } else {
        // At edge, make a turn.
        self.dir = self.dir.counter_clockwise();
        self.advance();
        return;
      }
    }
    self.x = x;
    self.y = y;
  }
}

fn idx(x: i64, y: i64, row_size: i64) -> usize {
  (x + y * row_size) as usize
}

fn sum_box(arr: &Vec<i64>, x: i64, y: i64, row_size: i64) -> i64 {
  let mut sum = 0;
  for i in -1..2 {
    for j in -1..2 {
      if i == 0 && j == 0 {
        continue;
      }
      sum += arr[idx(x + i, y + j, row_size)];
    }
  }
  sum
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let n = read_data_file(input)?;
  if n == 1 {
    return Ok("0".to_string());
  }
  let side = {
    let mut lvl = 0;
    let mut min: i64;
    let mut max = 1;
    let mut side: i64;
    loop {
      lvl += 1;
      side = lvl * 2;
      min = max + 1;
      max = max + side * 4;
      if min <= n && n <= max {
        break;
      }
    }
    if side % 2 != 0 {
      return Err(format_err!("Box size not odd: {}", side + 1));
    }
    side as i64 + 3
  };
  let mid = side / 2 + 1;
  let arr_area = side * side;
  let mut arr = Vec::with_capacity(arr_area as usize);
  for _ in 0..arr_area {
    arr.push(0);
  }
  let mut w = SpiralWalker::new(mid, mid);
  arr[idx(w.x, w.y, side)] = 1;
  let mut sum: i64 = 0;
  while sum < n {
    w.advance();
    sum = sum_box(&arr, w.x, w.y, side);
    arr[idx(w.x, w.y, side)] = sum;
  }
  Ok(format!("{}", sum))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
