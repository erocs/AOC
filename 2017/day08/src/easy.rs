#[macro_use] extern crate failure;

use std::collections::HashMap;
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

fn get_op(name: &str, line: &str) -> Result<fn(i64, i64) -> i64> {
  match name {
    "inc" => Ok((|x, n| x + n) as fn(i64, i64) -> i64),
    "dec" => Ok((|x, n| x - n) as fn(i64, i64) -> i64),
    _ => Err(format_err!("Invalid op: {}", line)),
  }
}

fn get_cmp(name: &str, line: &str) -> Result<fn(i64, i64) -> bool> {
  match name {
    ">" => Ok((|m, n| m > n) as fn(i64, i64) -> bool),
    ">=" => Ok((|m, n| m >= n) as fn(i64, i64) -> bool),
    "<" => Ok((|m, n| m < n) as fn(i64, i64) -> bool),
    "<=" => Ok((|m, n| m <= n) as fn(i64, i64) -> bool),
    "==" => Ok((|m, n| m == n) as fn(i64, i64) -> bool),
    "!=" => Ok((|m, n| m != n) as fn(i64, i64) -> bool),
    _ => Err(format_err!("Invalid cmp: {}", line)),
  }
}

struct Instruction {
  reg: String,
  op_fn: fn(i64, i64) -> i64,
  adj: i64,
  cmp_reg: String,
  cmp_fn: fn(i64, i64) -> bool,
  cmp_val: i64,
}

impl Instruction {
  fn parse(line: &str) -> Result<Instruction> {
    // 0   1         2     3  4   5                 6
    // \w+ (inc|dec) -?\d+ if \w+ (<|>|==|!=|>=|<=) -?\d+
    let row = line.split_whitespace()
                  .collect::<Vec<_>>();
    if row.len() != 7 {
      return Err(format_err!("Missing instruction parts, have {}: {}", row.len(), line));
    }
    if row[3] != "if" {
      return Err(format_err!("Invalid instruction, no if: {}", line));
    }
    Ok(Instruction {
      reg: row[0].to_string(),
      op_fn: get_op(row[1], line)?,
      adj: to_i64(row[2])?,
      cmp_reg: row[4].to_string(),
      cmp_fn: get_cmp(row[5], line)?,
      cmp_val: to_i64(row[6])?,
    })
  }
}

fn get_reg_val(regs: &HashMap<String, i64>, name: &str) -> i64 {
  if let Some(n) = regs.get(name) {
    *n
  } else {
    0
  }
}

fn instr_passes(regs: &mut HashMap<String, i64>, instr: &Instruction) -> bool {
  let cmp_reg_val = get_reg_val(&regs, &instr.cmp_reg);
  (instr.cmp_fn)(cmp_reg_val, instr.cmp_val)
}

fn update_reg(regs: &mut HashMap<String, i64>, instr: &Instruction) {
  let reg_val = get_reg_val(&regs, &instr.reg);
  let result = (instr.op_fn)(reg_val, instr.adj);
  regs.insert(instr.reg.to_string(), result);
}

fn read_data_file<P>(filename: P) -> Result<String>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut regs: HashMap<String, i64> = HashMap::new();
  let data = read_data_file(input)?;
  for line in data.lines() {
    let instr = Instruction::parse(line)?;
    if instr_passes(&mut regs, &instr) {
      update_reg(&mut regs, &instr);
    }
  }
  let mut max = i64::MIN;
  for n in regs.values() {
    max = i64::max(max, *n);
  }
  Ok(format!("{}", max))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
