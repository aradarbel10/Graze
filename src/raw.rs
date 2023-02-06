#![allow(dead_code)]

pub enum BinOp {
  OpAdd, OpSub, OpMul, OpDiv, OpMod,
  OpGT, OpGTE, OpLT, OpLTE, OpEq,
  OpAnd, OpOr, OpXor,
  OpIndex,
}

pub struct FieldVal {
  nam : String,
  val : Box<Expr>,
}
pub enum Expr {
  IntLit(i32),
  BoolLit(bool),
  CharLit(char),
  StringLit(String),
  BinOp(BinOp, Box<Expr>, Box<Expr>),
  Not(Box<Expr>),
  Var(String),
  FunCall(String, Option<Vec<Type>>, Vec<Expr>),
  Record(String, Vec<FieldVal>),
  Tuple(Vec<Expr>),
  ListLit(Vec<Expr>),
  Ref(Box<Expr>),
  Deref(Box<Expr>),
}

pub enum PreType {
  Int32,
  Boolean,
  Char,
  Prod(Vec<Type>),
  List(Type, Expr),
  TypVar(String),
  Ptr(Type),
}
pub enum TypeModifier {Mut, Immut, Pure}
pub struct Type {
  pretype : Box<PreType>,
  modifier : TypeModifier,
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