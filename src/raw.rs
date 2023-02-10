#![allow(dead_code)]

#[derive(Debug)]
pub enum BinOp {
  Add, Sub, Mul, Div, Mod,
  GT, GTE, LT, LTE, Eq,
  And, Or, Xor, Shl, Shr,
  Index,
}

#[derive(Debug)]
pub struct FieldVal {
  pub nam : String,
  pub val : Box<Expr>,
}
#[derive(Debug)]
pub enum Expr {
  IntLit(i32),
  BoolLit(bool),
  CharLit(char),
  StringLit(String),
  Unit,
  BinOp(BinOp, Box<Expr>, Box<Expr>),
  Not(Box<Expr>),
  Var(String),
  FunCall(String, Option<Vec<Type>>, Vec<Expr>),
  Record(String, Vec<FieldVal>),
  Proj(Box<Expr>, String),
  Tuple(Vec<Expr>),
  ListLit(Vec<Expr>),
  Deref(Box<Expr>),
  NewRegion,
  New(Box<Expr>),
  Allocate(Box<Expr>, Box<Expr>),
  Free(Box<Expr>),
}

#[derive(Debug)]
pub enum Type {
  Int32,
  Boolean,
  Char,
  Unit,
  Mut(Box<Type>),
  Prod(Vec<Type>),
  List(Box<Type>, i32),
  TypVar(String),
  TypApp(String, Vec<Type>),
  Ptr(Box<Type>),
  Region,
}

#[derive(Debug)]
pub enum Pattern {
  Wildcard,
  Var(String),
  Tuple(Vec<Pattern>),
  Or(Vec<Pattern>),
  List(Vec<Pattern>),
  Ctor(String, Vec<Pattern>),
}
#[derive(Debug)]
pub struct Branch {
  pub pat : Pattern,
  pub bod : Vec<Stmt>,
}
#[derive(Debug)]
pub enum Stmt {
  IfThen(Expr, Vec<Stmt>),
  IfThenElse(Expr, Vec<Stmt>, Vec<Stmt>),
  Match(Expr, Vec<Branch>),
  Block(Vec<Stmt>),
  ForEach(String, Expr, Vec<Stmt>),
  WhileLoop(Expr, Vec<Stmt>),
  VarDecl(Pattern, Option<Type>, Expr),
  VarAssgn(Expr, Expr),
  ExprStmt(Expr),
  Return(Expr),
}

#[derive(Debug)]
pub struct Param {
  pub nam : String,
  pub typ : Option<Type>,
}
#[derive(Debug)]
pub struct FunDefn {
  pub nam : String,
  pub tparams : Vec<TypParam>,
  pub params : Vec<Param>,
  pub ret : Option<Type>,
  pub bod : Vec<Stmt>,
}
#[derive(Debug)]
pub struct TypParam {
  pub nam : String,
  pub sups : Vec<String>,
}
#[derive(Debug)]
pub struct Field {
  pub nam : String,
  pub typ : Type,
}
#[derive(Debug)]
pub struct Sig {
  pub nam : String,
  pub tparams : Vec<TypParam>,
  pub params : Vec<Field>,
  pub ret : Type,
}
#[derive(Debug)]
pub enum Decl {
  Fun(FunDefn),
  Record(String, Vec<TypParam>, Vec<Field>),
  Variant(String, Vec<TypParam>, Vec<Field>),
  Class(String, TypParam, Vec<Sig>),
  Impl(String, Type, Vec<FunDefn>),
  Type(String, Vec<TypParam>, Type),
}