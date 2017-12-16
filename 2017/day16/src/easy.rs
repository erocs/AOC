#[macro_use] extern crate failure;

use std::collections::VecDeque;
use std::fs::File;
use std::i64;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

type Result<T> = result::Result<T, failure::Error>;

const MAX_CHAR_IDX: u8 = 16;

fn to_i64(s: &str) -> Result<i64> {
  match s.parse::<i64>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format_err!("Unable to parse i64 from '{}'", s)),
  }
}

#[derive(Debug)]
enum Move {
  Spin{n: i64},
  Exchange{a: i64, b: i64},
  Partner{a: char, b: char},
}

impl Move {
  fn parse(s: &str) -> Result<Move> {
    match &s[0..1] {
      "s" => Move::parse_spin(s),
      "x" => Move::parse_exchange(s),
      "p" => Move::parse_partner(s),
      _ => Err(format_err!("Unknown Move: {}", s)),
    }
  }

  fn parse_spin(s: &str) -> Result<Move> {
    let n = to_i64(&s[1..s.len()])?;
    if n < 0 || n > MAX_CHAR_IDX as i64 {
      Err(format_err!("Spin out of range: {}", s))
    } else {
      Ok(Move::Spin {
        n: n,
      })
    }
  }

  fn parse_exchange(s: &str) -> Result<Move> {
    let mut it = s[1..s.len()].split("/").map(to_i64);
    Ok(Move::Exchange{
      a: it.next().ok_or(format_err!("Missing Exchange A: {}", s))??,
      b: it.next().ok_or(format_err!("Missing Exchange B: {}", s))??,
    })
  }

  fn parse_partner(s: &str) -> Result<Move> {
    let mut it = s[1..s.len()].split("/");
    Ok(Move::Partner{
      a: it.next().ok_or(format_err!("Missing Partner A: {}", s))?
           .chars()
           .next().ok_or(format_err!("Zero length Partner A: {}", s))?,
      b: it.next().ok_or(format_err!("Missing Partner B: {}", s))?
           .chars()
           .next().ok_or(format_err!("Zero length Partner B: {}", s))?,
    })
  }

  fn run(&self, programs: &mut VecDeque<char>) -> Result<()> {
    match *self {
      Move::Spin{n} => self.run_spin(programs, n),
      Move::Exchange{a, b} => self.run_exchange(programs, a, b),
      Move::Partner{a, b} => self.run_partner(programs, a, b),
    }
  }

  fn run_spin(&self, programs: &mut VecDeque<char>, n: i64) -> Result<()> {
    for _ in 0..n {
      let m = programs.pop_back().ok_or(format_err!("Empty programs"))?;
      programs.push_front(m);
    }
    Ok(())
  }

  fn run_exchange(&self, programs: &mut VecDeque<char>, a: i64, b: i64) -> Result<()> {
    programs.swap(a as usize, b as usize);
    Ok(())
  }

  fn run_partner(&self, programs: &mut VecDeque<char>, a: char, b: char) -> Result<()> {
    let ai = programs.iter().take_while(|c| **c != a).fold(0, |acc, &_c| acc + 1);
    let bi = programs.iter().take_while(|c| **c != b).fold(0, |acc, &_c| acc + 1);
    programs.swap(ai, bi);
    Ok(())
  }
}

fn read_data_file<P>(filename: P) -> Result<Vec<Move>>
    where P: AsRef<Path> {
  let mut f = File::open(&filename)?;
  let mut data = String::new();
  f.read_to_string(&mut data)?;
  Ok(data.lines()
         .next().ok_or(format_err!("No input lines found"))?
         .split(',')
         .map(Move::parse)
         .collect::<Result<Vec<_>>>()?)
}

fn solve<P>(input: P) -> Result<String>
    where P: AsRef<Path> {
  let mut programs = (0..MAX_CHAR_IDX).map(|n| (n + 'a' as u8) as char)
                            .collect::<VecDeque<char>>();
  let instructions = read_data_file(input)?;
  for instruction in instructions.iter() {
    instruction.run(&mut programs)?;
  }
  Ok(format!("{}", programs.iter().collect::<String>()))
}

fn main() {
  match solve(PathBuf::from("input.txt")) {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
