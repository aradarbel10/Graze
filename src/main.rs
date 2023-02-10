mod core;
mod raw;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);

fn main() {
  let expr = parser::ExprParser::new();
  let typ = parser::TypeParser::new();
  let block = parser::BlockParser::new();

  println!("Hello, world!");
  println!("Debug: {:?}", expr.parse("foo[i32](x, ~a || b, ()) * 7 + false"));
  println!("Debug: {:?}", expr.parse("point {x = 1, y = 2}"));
  println!("Debug: {:?}", typ.parse("(i32, bool # 8)"));
  println!("Debug: {:?}", typ.parse("mut &(mut myTyp)"));
  println!("Debug: {:?}", block.parse("if true then ret x; else ret y; end"));
  println!("Debug: {:?}", block.parse("let x : i32 = 42; set y = x / 7; ret y;"));
}