#[macro_use] extern crate failure;

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
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
  children: Vec<String>,
  parent: Option<String>,
}

type TowerMap = HashMap<String, Tower>;

impl Tower {
  fn new(name: &str) -> Tower {
    Tower {
      name: name.to_string(),
      weight: 0,
      total_weight: -1,
      children: Vec::new(),
      parent: None,
    }
  }

  fn set_weight(&mut self, weight: i64) {
    self.weight = weight;
  }

  fn set_children(&mut self, children: Vec<String>) {
    self.children = children;
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

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let towers = read_data_file(input)?;
  let root_name = find_root_tower(&towers)?;
  Ok(format!("{}", root_name))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
