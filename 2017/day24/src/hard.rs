#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from: {}", s)),
  }
}

#[derive(Debug)]
struct Tube {
  sides: (i64, i64),
}

type TubeRef = Rc<RefCell<Tube>>;

impl Tube {
  fn new(s1: i64, s2: i64) -> TubeRef {
    Rc::new(RefCell::new(Tube {
      sides: (s1, s2),
    }))
  }
}

fn insert_tube(m: &mut HashMap<i64, Vec<TubeRef>>, n: i64, tube: &TubeRef) {
  if m.contains_key(&n) {
    let v = m.get_mut(&n).unwrap();
    v.push(Rc::clone(tube));
  } else {
    let mut v = Vec::new();
    v.push(Rc::clone(tube));
    m.insert(n, v);
  }
}

fn get_next_size(prev: &TubeRef, cur: &TubeRef) -> i64 {
  let p = prev.borrow();
  let c = cur.borrow();
  if p.sides.0 == c.sides.0 {
    c.sides.1
  } else if p.sides.0 == c.sides.1 {
    c.sides.0
  } else if p.sides.1 == c.sides.0 {
    c.sides.1
  } else {
    c.sides.0
  }
}

fn is_already_used(base: &Vec<TubeRef>, potential: &TubeRef) -> bool {
  for t in base {
    if Rc::ptr_eq(potential, t) {
      return true;
    }
  }
  false
}

fn calc_value(v: &Vec<TubeRef>) -> i64 {
  v.iter()
   .fold(0, |acc, t_ref| {
     let t = t_ref.borrow();
     acc + t.sides.0 + t.sides.1
   })
}

fn read_data_file<P>(filename: P) -> Result<HashMap<i64, Vec<TubeRef>>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut hm = HashMap::new();
  for line in data.lines() {
    let toks =
        line.split("/")
            .map(|s| to_i64(s))
            .collect::<Result<Vec<i64>>>()?;
    if toks.len() != 2 {
      return Err(format_err!("Incorrect number of pipe outputs: {}", toks.len()));
    }
    let tube = Tube::new(toks[0], toks[1]);
    insert_tube(&mut hm, toks[0], &tube);
    insert_tube(&mut hm, toks[1], &tube);
  }
  Ok(hm)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let tube_map = read_data_file(input)?;
  let marker = Tube::new(0, 0);
  let mut q = VecDeque::new();
  let mut longest = 1;
  let mut strongest = 0;
  let mut total = 0;
  for t in tube_map.get(&0).unwrap() {
    let mut v = Vec::new();
    v.push(Rc::clone(&marker));
    v.push(Rc::clone(t));
    q.push_back(v);
    total += 1;
  }
  while !q.is_empty() {
    total += 1;
    if (total & 0x1FFFF) == 0x10000 {
      println!("Processed: {}", total);
    }
    let base = q.pop_front().unwrap();
    let next = {
      let prev = &base[base.len() - 2];
      let cur = &base[base.len() - 1];
      get_next_size(&prev, &cur)
    };
    if let Some(potentials) = tube_map.get(&next) {
      let mut new_potentials = false;
      for nx in potentials {
        if is_already_used(&base, nx) {
          continue;
        }
        let mut v = base.clone();
        v.push(Rc::clone(nx));
        q.push_back(v);
        new_potentials = true;
      }
      if !new_potentials {
        if base.len() == longest {
          let total = calc_value(&base);
          if total > strongest {
            longest = base.len();
            strongest = total;
          }
        } else if base.len() > longest {
          let total = calc_value(&base);
          longest = base.len();
          strongest = total;
        }
      }
    } else {
      if base.len() == longest {
        let total = calc_value(&base);
        if total > strongest {
          longest = base.len();
          strongest = total;
        }
      } else if base.len() > longest {
        let total = calc_value(&base);
        longest = base.len();
        strongest = total;
      }
    }
  }
  println!("Total processed: {}", total);
  println!("Length: {}", longest);
  Ok(format!("{}", strongest))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
