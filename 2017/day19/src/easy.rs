#[macro_use] extern crate failure;

use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn read_data_file<P>(filename: P) -> Result<Vec<Vec<char>>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data.lines().map(|s| s.chars().collect::<Vec<char>>()).collect())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vector {
  x: i64,
  y: i64,
}

impl Vector {
  fn new(x: i64, y: i64) -> Vector {
    Vector { x: x, y: y }
  }

  fn add(&self, other: &Vector) -> Vector {
    Vector::new(self.x + other.x, self.y + other.y)
  }

  fn neg(&self) -> Vector {
    Vector::new(-self.x, -self.y)
  }
}

fn find_start_loc(data: &Vec<Vec<char>>) -> Result<Vector> {
  for (i, ch) in data[0].iter().enumerate() {
    if *ch == '|' {
      return Ok(Vector::new(0, i as i64));
    }
  }
  Err(format_err!("Char | not found"))
}

static DIRS: &[Vector] = &[Vector{x:1, y:0}, Vector{x:-1, y:0}, Vector{x:0, y:1}, Vector{x:0, y:-1}, ];

fn redirect(data: &Vec<Vec<char>>, idx: &Vector, dir: &Vector) -> Result<Vector> {
  let nir = dir.neg();
  for new_dir in DIRS {
    let jdx = idx.add(new_dir);
    let ch = data[jdx.x as usize][jdx.y as usize];
    if (ch == '|' || ch == '-') && *new_dir != nir {
      return Ok(new_dir.clone());
    }
  }
  Err(format_err!("Unable to determine new direction: {:?} {:?}", idx, dir))
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let data = read_data_file(input)?;
  let mut idx = find_start_loc(&data)?;
  let mut dir = Vector::new(1, 0);
  let mut res = Vec::new();
  let mut retry = true;
  loop {
    idx = idx.add(&dir);
    if idx.x < 0 || idx.y < 0 || idx.x >= data.len() as i64 || idx.y >= data[0].len() as i64 {
      // I guess it's done?
      break;
    }
    let ch = data[idx.x as usize][idx.y as usize];
    if ch == ' ' {
      if !retry {
        // I guess it's done?
        break;
      } else {
         retry = false;
      } 
      continue;
    } else {
      retry = true;
    }

    if ch.is_alphabetic() {
      res.push(ch);
    } else if ch == '+' {
      dir = redirect(&data, &idx, &dir)?;
    }
  }
  Ok(format!("{}", res.iter().collect::<String>()))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
