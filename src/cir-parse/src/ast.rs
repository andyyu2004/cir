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
    Lambda(Name, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
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
pub struct Var {
    pub name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TyVar {
    pub name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ty {
    Var(TyVar),
    Scalar(cir::Scalar),
    Fn(Box<Ty>, Box<Ty>),
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
    pub ty: Ty,
    pub expr: Expr,
}
