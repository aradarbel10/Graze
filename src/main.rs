mod core;
mod raw;

use std::fs;
use std::error::Error;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);

fn main() -> Result<(), Box<dyn Error>> {
  let str = fs::read_to_string("examples/test.grz")?;
  let prog = parser::ProgParser::new().parse(str.as_str());
  println!("Program: {:?}", prog);

  Ok(())
}