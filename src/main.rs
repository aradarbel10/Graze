mod core;
mod raw;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);

fn main() {
  let expr = parser::ExprParser::new();

  println!("Hello, world!");
  println!("Debug: {:?}", expr.parse("foo[i32](x, ~a || b, ()) * 7 + false"));
  println!("Debug: {:?}", expr.parse("point {x = 1, y = 2}"));
}