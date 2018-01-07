#[macro_use] extern crate failure;

use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::u16;

type Result<T> = result::Result<T, failure::Error>;

trait BitMagic {
  type Me;
  fn rotate_ccw_1(&self, dim: usize) -> Result<Self::Me>;
  fn rotate_ccw(&self, dim: usize) -> Result<Self::Me>;
  fn rotate_cw_1(&self, dim: usize) -> Result<Self::Me>;
  fn rotate_cw(&self, dim: usize) -> Result<Self::Me>;
  fn flip_horiz(&self, dim: usize) -> Result<Self::Me>;
  fn flip_vert(&self, dim: usize) -> Result<Self::Me>;
  fn normalize(&self, dim: usize) -> Result<Self::Me>;
}

fn bit_index(dim: usize, x: usize, y: usize) -> usize {
  x + y * dim
}

fn bit_coord(dim: usize, idx: usize) -> (usize, usize) {
  (idx % dim, idx / dim)
}


// Positive iteration is counter-clockwise.
const ROTATE2X2: &[(usize, usize)] = &[(0, 0), (1, 0), (1, 1), (0, 1)];
const ROTATE3X3: &[(usize, usize)] = &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)];

impl BitMagic for u16 {
  type Me = u16;

  fn rotate_ccw_1(&self, dim: usize) -> Result<Self::Me> {
    let n = *self;
    let mut new_n = n;
    if dim < 2 || dim > 3 {
      return Err(format_err!("Unsupported dim: {}", dim));
    }
    let rotations = match dim {
      2 => ROTATE2X2,
      3 => ROTATE3X3,
      _ => unreachable!(),
    };
    let mut next_val = {
      let tmp = rotations[rotations.len() - 1];
      (n >> bit_index(dim, tmp.0, tmp.1)) & 0x1
    };
    for i in 0..rotations.len() {
      let bit_idx = bit_index(dim, rotations[i].0, rotations[i].1);
      let cur_val = next_val;
      next_val = (n >> bit_idx) & 0x1;
      if cur_val == 1 {
        new_n = new_n | (cur_val << bit_idx);
      } else {
        new_n = new_n & !(1 << bit_idx);
      }
    }
    Ok(new_n)
  }

  fn rotate_ccw(&self, dim: usize) -> Result<Self::Me> {
    match dim {
      2 => self.rotate_ccw_1(dim),
      3 => self.rotate_ccw_1(dim)?.rotate_ccw_1(dim),
      _ => Err(format_err!("Unsupported dim: {}", dim)),
    }
  }

  fn rotate_cw_1(&self, dim: usize) -> Result<Self::Me> {
    let n = *self;
    let mut new_n = n;
    if dim < 2 || dim > 3 {
      return Err(format_err!("Unsupported dim: {}", dim));
    }
    let rotations = match dim {
      2 => ROTATE2X2,
      3 => ROTATE3X3,
      _ => unreachable!(),
    };
    let mut next_val = {
      let tmp = rotations[0];
      (n >> bit_index(dim, tmp.0, tmp.1)) & 0x1
    };
    let mut i = rotations.len();
    while i > 0 {
      i -= 1;
      let bit_idx = bit_index(dim, rotations[i].0, rotations[i].1);
      let cur_val = next_val;
      next_val = (n >> bit_idx) & 0x1;
      if cur_val == 1 {
        new_n = new_n | (cur_val << bit_idx);
      } else {
        new_n = new_n & !(1 << bit_idx);
      }
    }
    Ok(new_n)
  }

  fn rotate_cw(&self, dim: usize) -> Result<Self::Me> {
    match dim {
      2 => self.rotate_cw_1(dim),
      3 => self.rotate_cw_1(dim)?.rotate_cw_1(dim),
      _ => Err(format_err!("Unsupported dim: {}", dim)),
    }
  }

  fn flip_horiz(&self, dim: usize) -> Result<Self::Me> {
    let n = *self;
    if dim == 2 {
      Ok(((n & 0b0101) << 1) | ((n & 0b1010) >> 1))
    } else if dim == 3 {
      Ok(((n & 0b100100100) >> 2) | (n & 0b010010010) | ((n & 0b001001001) << 2))
    } else {
      Err(format_err!("Undefined dimention: {}", dim))
    }
  }

  fn flip_vert(&self, dim: usize) -> Result<Self::Me> {
    let n = *self;
    if dim == 2 {
      Ok(((n & 0b11) << 2) | ((n >> 2) & 0b11))
    } else if dim == 3 {
      Ok(((n & 0b111) << 6) | (n & 0b111000) | ((n >> 6) & 0b111))
    } else {
      Err(format_err!("Undefined dimention: {}", dim))
    }
  }

  fn normalize(&self, dim: usize) -> Result<Self::Me> {
    let n = *self;
    let mut min = n;
    let mut rot = n;
    if dim == 2 {
      rot = rot.rotate_cw(2)?;
      min = u16::min(min, rot);
      rot = rot.rotate_cw(2)?;
      min = u16::min(min, rot);
      rot = rot.rotate_cw(2)?;
      min = u16::min(min, rot);
    } else if dim == 3 {
      for _ in 0..4 {
        min = u16::min(min, rot);
        min = u16::min(min, rot.flip_horiz(3)?);
        min = u16::min(min, rot.flip_vert(3)?);
        rot = rot.rotate_cw(3)?;
      }
    } else {
      return Err(format_err!("Undefined dimention: {}", dim));
    }
    Ok(min)
  }
}

#[derive(Clone, Debug)]
struct Grid {
  dim: usize,
  data: Vec<bool>,
}

impl Grid {
  fn new(dim: usize) -> Grid {
    let len = dim.pow(2);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
      vec.push(false);
    }
    Grid {
      dim: dim,
      data: vec,
    }
  }

  fn from_subgrids(subs: Vec<Grid>) -> Result<Grid> {
    if subs.is_empty() {
      return Err(format_err!("Insufficient sub-grids: {}", subs.len()));
    }
    let dim = (subs.len() as f64).sqrt() as usize;
    if dim.pow(2) != subs.len() {
      return Err(format_err!("Insufficient sub-grids: {}", subs.len()));
    }
    let sub_dim = subs[0].dim;
    if subs.iter().any(|sg| sg.dim != sub_dim) {
      return Err(format_err!("Mismatched sub-grid DIM"));
    }
    let mut grid = Grid::new(dim * sub_dim);
    for (sg_idx, sg) in subs.iter().enumerate() {
      let (gx, gy) = bit_coord(dim, sg_idx);
      for x in 0..sub_dim {
        for y in 0..sub_dim {
          let idx = bit_index(grid.dim, gx * sub_dim + x, gy * sub_dim + y);
          let sub_idx = bit_index(sg.dim, x, y);
          grid.data[idx] = sg.data[sub_idx];
        }
      }
    }
    Ok(grid)
  }

  fn iter(&self, slice_dim: usize) -> Result<GridIterator> {
    if self.dim % slice_dim != 0 {
      return Err(format_err!("Grid dimention not a multiple of the slice size"));
    }
    Ok(GridIterator {
      grid: self,
      slice_dim: slice_dim,
      max_d: self.dim / slice_dim,
      dx: 0,
      dy: 0,
    })
  }

  fn to_n(&self) -> Result<u16> {
    if self.dim < 2 || self.dim > 4 {
      return Err(format_err!("Unable to translate grid of size {} to N", self.dim));
    }
    let mut n = 0;
    for (i, b) in self.data.iter().enumerate() {
      if *b {
        n = n | (1 << i);
      }
      if i >= 16 {
        return Err(format_err!("Too many elements: {}", i));
      }
    }
    Ok(n.normalize(self.dim)?)
  }

  fn to_n_unsafe(&self) -> u16 {
    match self.to_n() {
      Ok(n) => return n,
      Err(err) => panic!("to_n_unsafe error: {}", err),
    }
  }

  fn from_line(s: &str) -> Result<(Self, Self)> {
    let parts = s.trim().split(" => ").map(ToOwned::to_owned).collect::<Vec<String>>();
    if parts.len() != 2 {
      return Err(format_err!("Invalid pattern line: {}", s));
    }
    let grid_from = Grid::from_str(&parts[0])?;
    let grid_to = Grid::from_str(&parts[1])?;
    Ok((grid_from, grid_to))
  }
}

impl FromStr for Grid {
  type Err = failure::Error;

  fn from_str(s: &str) -> Result<Self> {
    let n_from = s.chars().filter(|ch| *ch == '#' || *ch == '.').count();
    let div_from = (n_from as f64).sqrt().round() as usize;
    if div_from.pow(2) != n_from {
      return Err(format_err!("Grid size not a square dim: {}", s));
    }
    let mut grid = Grid::new(div_from);
    for (i, ch) in s.chars().filter(|ch| *ch == '#' || *ch == '.').enumerate() {
      if ch == '#' {
        grid.data[i] = true;
      }
    }
    Ok(grid)
  }
}

impl Hash for Grid {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let hstr = format!("{}-{}", self.dim, self.to_n_unsafe());
    hstr.hash(state);
  }
}

impl PartialEq for Grid {
  fn eq(&self, other: &Grid) -> bool {
    let a = format!("{}-{}", self.dim, self.to_n_unsafe());
    let b = format!("{}-{}", other.dim, other.to_n_unsafe());
    a == b
  }
}

impl Eq for Grid {}

#[derive(Debug)]
struct GridIterator<'a> {
  grid: &'a Grid,
  slice_dim: usize,
  max_d: usize,
  dx: usize,
  dy: usize,
}

impl<'a> Iterator for GridIterator<'a> {
  type Item = Grid;

  fn next(&mut self) -> Option<Self::Item> {
    if self.dy >= self.max_d {
      return None;
    }
    let nx = self.dx * self.slice_dim;
    let ny = self.dy * self.slice_dim;
    let mut grid = Grid::new(self.slice_dim);
    for x in 0..self.slice_dim {
      for y in 0..self.slice_dim {
        let idx = bit_index(self.grid.dim, nx + x, ny + y);
        let sub_idx = bit_index(grid.dim, x, y);
        let n = self.grid.data[idx];
        grid.data[sub_idx] = n;
      }
    }
    self.dx += 1;
    if self.dx >= self.max_d {
      self.dx = 0;
      self.dy += 1;
    }
    Some(grid)
  }
}

type PatternMap = HashMap<Grid, Grid>;

fn read_data_file<P>(filename: P) -> Result<PatternMap>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = HashMap::new();
  for line in data.lines() {
    let (grid_from, grid_to) = Grid::from_line(line)?;
    if res.contains_key(&grid_from) {
      return Err(format_err!("Duplicate from Grid in input: {}", line));
    }
    res.insert(grid_from, grid_to);
  }
  Ok(res)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let translations = read_data_file(input)?;
  let mut grid = Grid::from_str(".#./..#/###")?;
  for _ in 0..5 {
    let mut vec = Vec::new();
    let n = if grid.dim % 2 == 0 {
      Ok(2)
    } else if grid.dim % 3 == 0 {
      Ok(3)
    } else {
      Err(format_err!("Bad multiple: {}", grid.dim))
    }?;
    for sgr in grid.iter(n)? {
      if !translations.contains_key(&sgr) {
        return Err(format_err!("No translation found: {}-{} {:?}", sgr.dim, sgr.to_n_unsafe(), sgr));
      }
      vec.push(translations.get(&sgr).unwrap().clone());
    }
    grid = Grid::from_subgrids(vec)?;
  }
  let count = grid.data.iter().filter(|b| **b).count();
  Ok(format!("{:?}", count))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
