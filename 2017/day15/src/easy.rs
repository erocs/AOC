#[macro_use] extern crate failure;

use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Write};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::ops::Deref;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, failure::Error>;

fn solve() -> Result<String> {
  let mut a: i64 = 591;
  let af: i64 = 16807;
  let mut b: i64 = 393;
  let bf: i64 = 48271;
  let mut count = 0;
  for _ in 0..40_000_001 {
    a = (a * af) % 2147483647;
    b = (b * bf) % 2147483647;
    if (a & 0xFFFF) == (b & 0xFFFF) {
      count += 1;
    }
  }
  Ok(format!("{}", count))
}

fn main() {
  match solve() {
    Ok(msg) => println!("Result: {}", msg),
    Err(err) => println!("Error: {}", err),
  }
}
