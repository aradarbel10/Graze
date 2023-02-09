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

pub enum Pattern {
  PatWildcard,
  PatVar(String),
  PatTuple(Vec<Pattern>),
  PatOr(Vec<Pattern>),
  PatList(Vec<Pattern>),
  PatCtor(String, Box<Pattern>),
}
pub struct Branch {
  pat : Pattern,
  bod : Expr,
}
pub enum Stmt {
  IfThen(Expr, Box<Stmt>),
  IfThenElse(Expr, Box<Stmt>, Box<Stmt>),
  Match(Expr, Vec<Branch>),
  Block(Vec<Stmt>),
  ForLoop(Box<Stmt>, Expr, Box<Stmt>, Box<Stmt>),
  WhileLoop(Expr, Box<Stmt>),
  VarDecl(Pattern, Option<Type>, Expr),
  VarAssgn(Expr, Expr),
  ExprStmt(Expr),
  Return(Expr),
}

pub struct Param {
  nam : String,
  typ : Option<Type>,
}
pub struct FunDefn {
  nam : String,
  tparams : Vec<TypParam>,
  params : Vec<Param>,
  ret : Option<Type>,
  bod : Stmt,
}
pub struct TypParam {
  nam : String,
  sups : Vec<String>,
}
pub struct Field {
  nam : String,
  typ : Type,
}
pub struct Sig {
  nam : String,
  tparams : Vec<TypParam>,
  params : Vec<(String, Type)>,
  ret : Type,
}
pub enum Decl {
  FunDecl(FunDefn),
  RecordDecl(String, Vec<TypParam>, Vec<Field>),
  VariantDecl(String, Vec<TypParam>, Vec<Field>),
  ClassDecl(String, TypParam, Vec<Sig>),
  ImplDecl(String, Type, Vec<FunDefn>),
}