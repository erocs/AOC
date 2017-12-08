#[macro_use] extern crate failure;

use std::ascii::AsciiExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: String) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from: {}", s)),
  }
}

#[derive(Debug)]
struct Tower {
  name: String,
  weight: i64,
  total_weight: i64,
  children: Rc<RefCell<Vec<String>>>,
  parent: Option<String>,
}

type TowerMap = HashMap<String, Tower>;

impl Tower {
  fn new(name: &str) -> Tower {
    Tower {
      name: name.to_string(),
      weight: 0,
      total_weight: -1,
      children: Rc::new(RefCell::new(Vec::new())),
      parent: None,
    }
  }

  fn set_weight(&mut self, weight: i64) {
    self.weight = weight;
  }

  fn set_children(&mut self, children: Vec<String>) {
    self.children = Rc::new(RefCell::new(children));
  }

  fn set_parent(&mut self, parent: &str) {
    self.parent = Some(parent.to_string());
  }
}

fn insert_tower(towers: &mut TowerMap, name: &str) {
  if !towers.contains_key(name) {
    towers.insert(name.to_string(), Tower::new(name));
  }
}

fn read_data_file<P>(filename: P) -> Result<TowerMap>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut towers = TowerMap::new();
  for line in data.lines() {
    // Convert: nafjxju (347) -> hptnh, zcyjg
    // To: ['nafjxju', '347', 'hptnh', 'zcyjg']
    let mut row = line.chars()
                      .filter(|c| c.is_ascii() && (c.is_whitespace() || c.is_alphanumeric()))
                      .collect::<String>()
                      .split_whitespace()
                      .map(ToOwned::to_owned)
                      .collect::<Vec<String>>();
    if row.len() < 2 {
      return Err(format_err!("Missing tower name or weight"));
    }
    let children = row.split_off(2);
    let weight = to_i64(row.pop().unwrap())?;
    let name = row.pop().unwrap();
    for ch_name in children.iter() {
      insert_tower(&mut towers, ch_name);
      if let Some(child) = towers.get_mut(ch_name.as_str()) {
        child.set_parent(name.as_str());
      }
    } 
    insert_tower(&mut towers, name.as_str());
    if let Some(tower) = towers.get_mut(&name) {
      tower.set_weight(weight);
      tower.set_children(children);
    }
  }
  Ok(towers)
}

fn find_root_tower(towers: &TowerMap) -> Result<String> {
  let mut root_name = towers.keys()
                            .next()
                            .ok_or(format_err!("No towers defined"))?
                            .clone();
  loop {
    if let Some(t) = towers.get(&root_name) {
      if let Some(ref p) = t.parent {
        root_name = p.clone();
      } else {
        return Ok(root_name);
      }
    }
  }
}

fn calc_tower_weights(lvl: i64, towers_ref: Rc<RefCell<TowerMap>>, name: &str) -> Result<i64> {
  let mut total = 0;
  let children_ref = {
    let towers = towers_ref.borrow();
    if let Some(tower) = towers.get(name) {
      total = tower.weight;
      Ok(Rc::clone(&tower.children))
    } else {
      Err(format_err!("Unknown tower: {}", name))
    }
  }?;
  let children = children_ref.borrow();
  for child in children.iter() {
    let weight = calc_tower_weights(lvl + 1, Rc::clone(&towers_ref), child)?;
    total += weight;
  }
  let mut towers = towers_ref.borrow_mut();
  if let Some(tower) = towers.get_mut(name) {
    tower.weight = total;
println!("{} {}: {}", lvl, name, total);
  }
  Ok(total)
}

//fn find_imbalance_correction(towers: &TowerMap) -> Result<i64> {
//
//}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut towers = read_data_file(input)?;
  let root_name = { find_root_tower(&mut towers)? };
// TODO: FINISH, NOT COMPLETELY AUTOMATIC
//  calc_tower_weights(0, Rc::new(RefCell::new(towers)), root_name.as_str())?;
  calc_tower_weights(0, Rc::new(RefCell::new(towers)), "orflty")?;
//  let correction = find_imbalance_correction(towers);
  Ok(format!("{}", 0))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
