#[macro_use] extern crate failure;

use std::ascii::AsciiExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::Read;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from '{}'", s)),
  }
}

#[derive(Debug)]
enum Val {
  Reg(String),
  Num(i64),
  None,
}

impl Val {
  fn get(&self, bus: &RegBus) -> Result<i64> {
    match *self {
      Val::Reg(ref n) => {
        let reg_ref = bus.get(&n)?;
        let val = reg_ref.borrow().to_owned();
        Ok(val)
      },
      Val::Num(ref n) => {Ok(n.to_owned())},
      Val::None => Err(format_err!("Val::None")),
    }
  }
}

#[derive(Debug)]
enum OpResult {
  Ok,
  Rcv(i64),
  PcAdj(i64),
}

#[derive(Debug)]
enum Instr {
  Add(Val, Val),
  Jgz(Val, Val),
  Mod(Val, Val),
  Mul(Val, Val),
  Rcv(Val),
  Set(Val, Val),
  Snd(Val),
}

impl Instr {
  fn from(bus: &RegBus, s: &str, p1: &str, p2: &str) -> Result<Instr> {
    if s.len() != 3 {
      return Err(format_err!("Invalid instruction: {}", s));
    }
    if p1.is_empty() {
      return Err(format_err!("Empty instruction argument 1"));
    }
    if !bus.contains(p1) {
      if !RegBus::valid_name(p1) {
        return Err(format_err!("Unknown register: {}", p1));
      }
      bus.get(p1)?;
    }
    let p1 = Val::Reg(p1.to_owned());
    let p2 = if bus.contains(p2) {
      Val::Reg(p2.to_owned())
    } else if let Ok(n) = to_i64(p2) {
      Val::Num(n)
    } else {
      Val::None
    };
    match s.to_lowercase().as_str() {
      "add" => Ok(Instr::Add(p1, p2)),
      "jgz" => Ok(Instr::Jgz(p1, p2)),
      "mod" => Ok(Instr::Mod(p1, p2)),
      "mul" => Ok(Instr::Mul(p1, p2)),
      "rcv" => Ok(Instr::Rcv(p1)),
      "set" => Ok(Instr::Set(p1, p2)),
      "snd" => Ok(Instr::Snd(p1)),
      _ => Err(format_err!("Unknown instruction: {}", s)),
    }
  }

  fn run(&self, bus: &RegBus) -> Result<OpResult> {
    match *self {
      Instr::Add(ref v1, ref v2) => self.run_add(&bus, &v1, &v2),
      Instr::Jgz(ref v1, ref v2) => self.run_jgz(&bus, &v1, &v2),
      Instr::Mod(ref v1, ref v2) => self.run_mod(&bus, &v1, &v2),
      Instr::Mul(ref v1, ref v2) => self.run_mul(&bus, &v1, &v2),
      Instr::Rcv(ref v1) => self.run_rcv(&bus, &v1),
      Instr::Set(ref v1, ref v2) => self.run_set(&bus, &v1, &v2),
      Instr::Snd(ref v1) => self.run_snd(&bus, &v1),
    }
  }

  fn run_add(&self, bus: &RegBus, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(bus)?;
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let mut reg = reg_ref.borrow_mut();
    let r = reg.deref_mut();
    *r += val;
    Ok(OpResult::Ok)
  }

  fn run_jgz(&self, bus: &RegBus, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(bus)?;
    let cmp = match *v1 {
      Val::Reg(ref n) => {
        let reg_ref = bus.get(n)?;
        let reg = reg_ref.borrow();
        Ok(*reg)
      },
      Val::Num(ref n) => Ok(*n),
      Val::None => Err(format_err!("Val::None encountered")),
    }?;
    if cmp > 0 {
       Ok(OpResult::PcAdj(val))
    } else {
       Ok(OpResult::Ok)
    }
  }

  fn run_mod(&self, bus: &RegBus, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(bus)?;
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let mut reg = reg_ref.borrow_mut();
    let r = reg.deref_mut();
    *r = *r % val;
    Ok(OpResult::Ok)
  }

  fn run_mul(&self, bus: &RegBus, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(bus)?;
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let mut reg = reg_ref.borrow_mut();
    let r = reg.deref_mut();
    *r = *r * val;
    Ok(OpResult::Ok)
  }

  fn run_rcv(&self, bus: &RegBus, v1: &Val) -> Result<OpResult> {
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let reg = reg_ref.borrow();
    if *reg > 0 {
      Ok(OpResult::Rcv(bus.get_snd()))
    } else {
      Ok(OpResult::Ok)
    }
  }

  fn run_set(&self, bus: &RegBus, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(bus)?;
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let mut reg = reg_ref.borrow_mut();
    let r = reg.deref_mut();
    *r = val;
    Ok(OpResult::Ok)
  }

  fn run_snd(&self, bus: &RegBus, v1: &Val) -> Result<OpResult> {
    let reg_name = if let Val::Reg(ref n) = *v1 {&n} else {""};
    let reg_ref = bus.get(reg_name)?;
    let reg = reg_ref.borrow();
    bus.set_snd(*reg);
    Ok(OpResult::Ok)
  }
}

#[derive(Debug)]
struct RegBus {
  rs_ref: Rc<RefCell<HashMap<String, Rc<RefCell<i64>>>>>,
  snd_ref: Rc<RefCell<i64>>,
}

impl RegBus {
  fn valid_name(s: &str) -> bool {
    s.len() == 1 && s.chars().all(|c| char::is_ascii(&c))
  }

  fn new() -> RegBus {
    RegBus {
      rs_ref: Rc::new(RefCell::new(HashMap::new())),
      snd_ref: Rc::new(RefCell::new(0)),
    }
  }

  fn get(&self, s: &str) -> Result<Rc<RefCell<i64>>> {
    if s.len() != 1 {
      return Err(format_err!("Bad register name: {}", s));
    }
    let exists = { self.rs_ref.borrow().contains_key(s) };
    if !exists {
      let mut rs = self.rs_ref.borrow_mut();
      rs.insert(s.to_owned(), Rc::new(RefCell::new(0)));
    }
    let rs = self.rs_ref.borrow();
    Ok(Rc::clone(rs.get(s).unwrap()))
  }

  fn contains(&self, s: &str) -> bool {
    self.rs_ref.borrow().contains_key(s)
  }

  fn set_snd(&self, n: i64) {
    let mut snd = self.snd_ref.borrow_mut();
    *snd = n;
  }

  fn get_snd(&self) -> i64 {
    let snd = self.snd_ref.borrow();
    *snd
  }
}

fn read_data_file<P>(bus: &RegBus, filename: P) -> Result<Vec<Instr>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  for line in data.lines() {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let instr = if parts.len() == 2 {
      Instr::from(&bus, parts[0], parts[1], "")
    } else if parts.len() == 3 {
      Instr::from(&bus, parts[0], parts[1], parts[2])
    } else {
      Err(format_err!("Bad instruction: {}", line))
    }?;
    res.push(instr);
  }
  Ok(res)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let bus = RegBus::new(); 
  let program = read_data_file(&bus, input)?;
  let mut res = 0;
  let mut pc = 0;
  loop {
    let instr = &program[pc];
    let instr_res = instr.run(&bus)?;
    match instr_res {
      OpResult::Ok => { pc += 1; },
      OpResult::PcAdj(pc_adj) => {
        pc = (pc as i64 + pc_adj) as usize;
      },
      OpResult::Rcv(n) => {
         res = n;
         break;
      },
    };
  }
  Ok(format!("{}", res))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
