#![allow(dead_code)]

pub enum BinOp {
  OpAdd, OpSub, OpMul, OpDiv, OpMod,
  OpGT, OpGTE, OpLT, OpLTE, OpEq,
  OpAnd, OpOr, OpXor, Shl, Shr,
  OpIndex,
}

pub struct FieldVal<'a> {
  nam : &'a str,
  val : Box<Expr<'a>>,
}
pub enum Expr<'a> {
  IntLit(i32),
  BoolLit(bool),
  CharLit(char),
  StringLit(String),
  Unit,
  BinOp(BinOp, Box<Expr<'a>>, Box<Expr<'a>>),
  Not(Box<Expr<'a>>),
  Var(&'a str),
  FunCall(&'a str, Vec<Type<'a>>, Vec<Expr<'a>>),
  Record(&'a str, Vec<FieldVal<'a>>),
  Proj(Box<Expr<'a>>, &'a str),
  Tuple(Vec<Expr<'a>>),
  ListLit(Vec<Expr<'a>>),
  Deref(Box<Expr<'a>>),
  NewRegion,
  New(Box<Expr<'a>>),
  Allocate(Box<Expr<'a>>, Box<Expr<'a>>),
  Free(Box<Expr<'a>>),
}

type Meta<'a> = &'a mut Option<Type<'a>>;
pub enum Type<'a> {
  Int32,
  Boolean,
  Char,
  Unit,
  Mut(Box<Type<'a>>),
  Prod(Vec<Type<'a>>),
  List(Box<Type<'a>>, i32),
  TypVar(&'a str),
  TypApp(&'a str, Vec<Type<'a>>),
  Ptr(Box<Type<'a>>),
  Region,
}



pub enum Pattern<'a> {
  PatWildcard,
  PatVar(String),
  PatTuple(Vec<Pattern<'a>>),
  PatOr(Vec<Pattern<'a>>),
  PatList(Vec<Pattern<'a>>),
  PatCtor(&'a str, Box<Pattern<'a>>),
}
pub struct Branch<'a> {
  pat : Pattern<'a>,
  bod : Expr<'a>,
}
pub enum Stmt<'a> {
  IfThen(Expr<'a>, Box<Stmt<'a>>),
  IfThenElse(Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
  Match(Expr<'a>, Vec<Branch<'a>>),
  Block(Vec<Stmt<'a>>),
  ForLoop(Box<Stmt<'a>>, Expr<'a>, Box<Stmt<'a>>, Box<Stmt<'a>>),
  WhileLoop(Expr<'a>, Box<Stmt<'a>>),
  VarDecl(String, Type<'a>, Expr<'a>),
  VarAssgn(Expr<'a>, Expr<'a>),
  ExprStmt(Expr<'a>),
  Return(Expr<'a>),
}

pub struct Param<'a> {
  nam : String,
  typ : Type<'a>,
}
pub struct FunDefn<'a> {
  nam : String,
  tparams : Vec<TypParam>,
  params : Vec<Param<'a>>,
  ret : Type<'a>,
  bod : Stmt<'a>,
}
pub struct TypParam {
  nam : String,
  sups : Vec<String>,
}
pub struct Field<'a> {
  nam : String,
  typ : Type<'a>,
}
pub struct Sig<'a> {
  nam : String,
  tparams : Vec<TypParam>,
  params : Vec<(String, Type<'a>)>,
  ret : Type<'a>,
}
pub enum Decl<'a> {
  FunDecl(FunDefn<'a>),
  RecordDecl(String, Vec<TypParam>, Vec<Field<'a>>),
  VariantDecl(String, Vec<TypParam>, Vec<Field<'a>>),
  ClassDecl(String, TypParam, Vec<Sig<'a>>),
  ImplDecl(String, Type<'a>, Vec<FunDefn<'a>>),
}