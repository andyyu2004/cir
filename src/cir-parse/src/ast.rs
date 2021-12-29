use cir::Name;
use codespan::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Var(Var),
    Lit(Literal),
    Lambda(Binder, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Case(Box<Expr>, Alts),
    Type(Type),
}

pub type Alts = Vec<Alt>;

#[derive(Debug, PartialEq, Eq)]
pub struct Alt {
    pub pat: Pat,
    pub body: Expr,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pat {
    Variant(Path),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Binder {
    Val(Name, Type),
    Ty(TyVar),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct TyVar {
    pub name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Literal {
    pub span: Span,
    pub kind: LiteralKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int(i64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Var {
    /// Value level variable
    Val { name: Name },
    /// Type level variable
    Ty(TyVar),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Var(TyVar),
    Scalar(cir::Scalar),
    Fn(Box<Type>, Box<Type>),
    ForAll(TyVar, Box<Type>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub span: Span,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemKind {
    ValueDef(ValueDef),
    DataDef(DataDef),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueDef {
    pub name: Name,
    pub ty: Type,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    // TODO paths are just a name for now
    pub name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataDef {
    pub name: Name,
    pub binders: Vec<TyVar>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: Name,
    pub params: Vec<Type>,
}
