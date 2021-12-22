#![feature(once_cell)]

use codespan::Span;
use la_arena::Idx;
use smallvec::{smallvec, SmallVec};
use smol_str::SmolStr;

pub use self::intern::{Intern, Interned};

mod intern;

#[derive(Debug, PartialEq, Eq)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    ValueDef(Idx<ValueDef>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueDef {
    pub name: Name,
    pub ty: Ty,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub span: Span,
    pub symbol: SmolStr,
}

impl Name {
    pub fn new(span: Span, symbol: impl AsRef<str>) -> Self {
        Self { span, symbol: SmolStr::new(symbol) }
    }
}

pub type Expr = Idx<ExprData>;

#[derive(Debug, PartialEq, Eq)]
pub enum ExprData {
    Var(Debruijn),
    Lit(Lit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Debruijn {
    depth: usize,
}

impl Debruijn {
    pub const fn new(depth: usize) -> Self {
        Self { depth }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lit {
    Bool(bool),
    Int(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub segments: SmallVec<[PathSegment; 1]>,
}

impl Path {
    pub fn single(name: Name) -> Self {
        Self { segments: smallvec![PathSegment { name }] }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PathSegment {
    name: Name,
}

pub type Body = Idx<BodyData>;

#[derive(Debug, PartialEq, Eq)]
pub struct BodyData {
    pub expr: Expr,
}

pub type Ty = Interned<TyData>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TyData {
    // TODO flags
    kind: TyKind,
}

impl TyData {
    pub fn new(kind: TyKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TyKind {
    Scalar(Scalar),
    Fn(Ty, Ty),
    Var(TyVar),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TyVar {}

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty::intern(TyData::new(self))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scalar {
    Bool,
    Int,
    Float,
}
