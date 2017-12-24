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
  fn get(&self, cpu: &CPU) -> Result<i64> {
    match *self {
      Val::Reg(ref n) => {
        let reg_ref = cpu.get_reg(&n)?;
        let val = reg_ref.borrow().to_owned();
        Ok(val)
      },
      Val::Num(ref n) => {Ok(n.to_owned())},
      Val::None => Err(format_err!("Val::None")),
    }
  }

  fn get_reg(&self, cpu: &CPU) -> Result<i64> {
    match *self {
      Val::Reg(ref name) => {
        Ok(cpu.get_reg_val(&name)?)
      },
      _ => Err(format_err!("Non-register value")),
    }
  }

  fn get_reg_name(&self) -> Result<String> {
    if let Val::Reg(ref name) = *self {
      Ok(name.clone())
    } else {
      Err(format_err!("Value not a register"))
    }
  }
}

#[derive(Debug)]
enum OpResult {
  Ok,
  PcAdj(i64),
  NoMoreInstructions,
}

#[derive(Debug)]
enum Instr {
  Jnz(Val, Val),
  Mul(Val, Val),
  Set(Val, Val),
  Sub(Val, Val),
}

impl Instr {
  fn from(cpu: &CPU, s: &str, p1: &str, p2: &str) -> Result<Instr> {
    if s.len() != 3 {
      return Err(format_err!("Invalid instruction: {}", s));
    }
    if p1.is_empty() {
      return Err(format_err!("Empty instruction argument 1"));
    }
    let p1 = {
      let mut tmp_p1 = Val::None;
      if cpu.contains_reg(p1) {
        tmp_p1 = Val::Reg(p1.to_owned());
      } else {
        if CPU::valid_name(p1) {
          // Register the register
          cpu.get_reg(p1)?;
          tmp_p1 = Val::Reg(p1.to_owned());
        } else if let Ok(n) = to_i64(p1) {
          tmp_p1 = Val::Num(n);
        } else {
          return Err(format_err!("Invalid first argument: {}", p1));
        }
      }
      tmp_p1
    };
    let p2 = if cpu.contains_reg(p2) {
      Val::Reg(p2.to_owned())
    } else if let Ok(n) = to_i64(p2) {
      Val::Num(n)
    } else {
      Val::None
    };
    match s.to_lowercase().as_str() {
      "jnz" => Ok(Instr::Jnz(p1, p2)),
      "mul" => Ok(Instr::Mul(p1, p2)),
      "set" => Ok(Instr::Set(p1, p2)),
      "sub" => Ok(Instr::Sub(p1, p2)),
      _ => Err(format_err!("Unknown instruction: {}", s)),
    }
  }

  fn run(&self, cpu: &CPU) -> Result<OpResult> {
    cpu.profile(self);
    match *self {
      Instr::Jnz(ref v1, ref v2) => self.run_jnz(&cpu, &v1, &v2),
      Instr::Mul(ref v1, ref v2) => self.run_mul(&cpu, &v1, &v2),
      Instr::Set(ref v1, ref v2) => self.run_set(&cpu, &v1, &v2),
      Instr::Sub(ref v1, ref v2) => self.run_sub(&cpu, &v1, &v2),
    }
  }

  fn run_jnz(&self, cpu: &CPU, v1: &Val, v2: &Val) -> Result<OpResult> {
    let cmp = v1.get(&cpu)?;
    let val = v2.get(&cpu)?;
    if cmp != 0 {
       Ok(OpResult::PcAdj(val))
    } else {
       Ok(OpResult::Ok)
    }
  }

  fn run_mul(&self, cpu: &CPU, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(cpu)?;
    let reg_name = v1.get_reg_name()?;
    let reg = cpu.get_reg_val(&reg_name)?;
    cpu.set_reg(&reg_name, reg * val)?;
    Ok(OpResult::Ok)
  }

  fn run_set(&self, cpu: &CPU, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(cpu)?;
    let reg_name = v1.get_reg_name()?;
    cpu.set_reg(&reg_name, val)?;
    Ok(OpResult::Ok)
  }

  fn run_sub(&self, cpu: &CPU, v1: &Val, v2: &Val) -> Result<OpResult> {
    let val: i64 = v2.get(cpu)?;
    let reg_name = v1.get_reg_name()?;
    let reg = cpu.get_reg_val(&reg_name)?;
    cpu.set_reg(&reg_name, reg - val)?;
    Ok(OpResult::Ok)
  }
}

#[derive(Debug)]
struct CPU {
  id: usize,
  pc_ref: RefCell<usize>,
  rs_ref: Rc<RefCell<HashMap<String, Rc<RefCell<i64>>>>>,
  prog: Vec<Instr>,
  prof_ref: RefCell<HashMap<String, usize>>,
}

impl CPU {
  fn valid_name(s: &str) -> bool {
    s.len() == 1 && s.chars().all(|c| char::is_ascii(&c) && c >= 'a' && c <= 'h')
  }

  fn new(id: i64) -> Result<CPU> {
    let cpu = CPU {
      id: id as usize,
      pc_ref: RefCell::new(0),
      rs_ref: Rc::new(RefCell::new(HashMap::new())),
      prog: Vec::new(),
      prof_ref: RefCell::new(HashMap::new()),
    };
    let reg_ref = cpu.get_reg("p")?;
    let mut reg = reg_ref.borrow_mut();
    *reg = id;
    Ok(cpu)
  }

  fn get_reg(&self, s: &str) -> Result<Rc<RefCell<i64>>> {
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

  fn get_reg_val(&self, s: &str) -> Result<i64> {
    let reg_ref = self.get_reg(s)?;
    let val = *reg_ref.borrow();
    Ok(val)
  }

  fn set_reg(&self, name: &str, val: i64) -> Result<()> {
    let reg_ref = self.get_reg(name)?;
    let mut reg = reg_ref.borrow_mut();
    let r = reg.deref_mut();
    *r = val;
    Ok(())
  }

  fn contains_reg(&self, s: &str) -> bool {
    self.rs_ref.borrow().contains_key(s)
  }

  //fn print_reg_debug(&self) {
  //  let reg = self.rs_ref.borrow();
  //  let mut sorted = reg.keys().cloned().collect::<Vec<String>>();
  //  sorted.sort();
  //  let sorted = sorted;
  //  for k in sorted {
  //    let v_ref = reg.get(&k).unwrap();
  //    let v = v_ref.borrow();
  //    print!("REG {} = {}  ", k, v);
  //  }
  //  println!("");
  //}

  fn pc(&self) -> usize {
    *self.pc_ref.borrow()
  }

  fn adj_pc(&self, adj: i64) {
    let mut pc = self.pc_ref.borrow_mut();
    *pc = (*pc as i64 + adj) as usize;
  }

  fn set_prog(&mut self, new_prog: Vec<Instr>) {
    self.prog = new_prog;
  }

  fn step(&self) -> Result<OpResult> {
    let pc = self.pc();
    if pc >= self.prog.len() {
      return Ok(OpResult::NoMoreInstructions);
    }
    let instr = &self.prog[pc];
    let instr_res = instr.run(&self)?;
    match instr_res {
      OpResult::PcAdj(pc_adj) => self.adj_pc(pc_adj),
      _ => self.adj_pc(1),
    };
    Ok(OpResult::Ok)
  }

  fn profile(&self, instr: &Instr) {
    let reg_name = match *instr {
      Instr::Jnz(_, _) => "jnz",
      Instr::Mul(_, _) => "mul",
      Instr::Set(_, _) => "set",
      Instr::Sub(_, _) => "sub",
    };
    let mut prof = self.prof_ref.borrow_mut();
    if prof.contains_key(reg_name) {
      let p = prof.get_mut(reg_name).unwrap();
      *p = *p + 1;
    } else {
      prof.insert(reg_name.to_owned(), 1);
    }
  }

  fn get_profile(&self, reg_name: &str) -> usize {
    let prof = self.prof_ref.borrow();
    *prof.get(reg_name).unwrap_or(&0)
  }
}

fn read_data_file<P>(cpu: &mut CPU, filename: &P) -> Result<()>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  let mut res = Vec::new();
  for line in data.lines() {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let instr = if parts.len() == 2 {
      Instr::from(&cpu, parts[0], parts[1], "")
    } else if parts.len() == 3 {
      Instr::from(&cpu, parts[0], parts[1], parts[2])
    } else {
      Err(format_err!("Bad instruction: {}", line))
    }?;
    res.push(instr);
  }
  cpu.set_prog(res);
  Ok(())
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut cpu = CPU::new(0)?;
  read_data_file(&mut cpu, &input)?;
  let mut running = true;
  while running {
    let cpu_res = cpu.step()?;
    match cpu_res {
      OpResult::NoMoreInstructions => {
        running = false;
        break;
      },
      _ => {},
    };
  }
  Ok(format!("{}", cpu.get_profile("mul")))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
