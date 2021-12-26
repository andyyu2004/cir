#![feature(once_cell)]

use std::hash::Hash;
use std::ops::Index;

use codespan::Span;
use la_arena::{Arena, Idx};
use smallvec::{smallvec, SmallVec};
use smol_str::SmolStr;

pub use self::intern::{Intern, Interned};

mod db;
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

#[derive(Debug, Clone, Eq)]
pub struct Name {
    pub span: Span,
    pub symbol: SmolStr,
}

impl Hash for Name {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Name {
    pub fn new(span: Span, symbol: impl AsRef<str>) -> Self {
        Self { span, symbol: SmolStr::new(symbol) }
    }
}

pub type Expr = Idx<ExprData>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprData {
    Var(Binder),
    Lit(Lit),
    Lambda(Expr),
    App(Expr, Expr),
    Type(Ty),
}

pub type Binder = Idx<BinderData>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinderData {
    Val(Ty),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyData {
    /// The top-level expression
    pub expr: Expr,
    pub exprs: Arena<ExprData>,
    pub binders: Arena<BinderData>,
}

impl BodyData {
    pub fn new(expr: Expr, exprs: Arena<ExprData>, binders: Arena<BinderData>) -> Self {
        Self { expr, exprs, binders }
    }
}

impl Index<Expr> for BodyData {
    type Output = ExprData;

    fn index(&self, index: Expr) -> &Self::Output {
        &self.exprs[index]
    }
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

    pub fn kind(&self) -> &TyKind {
        &self.kind
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
