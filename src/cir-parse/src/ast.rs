use cir::{Name, TyVar};
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
    Type(Type),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Binder {
    Val(Name, Type),
    Ty(TyVar),
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
    ForAll(Name, Box<Type>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub span: Span,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemKind {
    ValueDef(ValueDef),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueDef {
    pub name: Name,
    pub ty: Type,
    pub expr: Expr,
}
