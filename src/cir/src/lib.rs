#![feature(once_cell)]

use std::fmt;
use std::hash::{Hash, Hasher};
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

pub type ValueDef = Idx<ValueDefData>;
pub type DataDef = Idx<DataDefData>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    ValueDef(ValueDef),
    DataDef(DataDef),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataDefData {}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueDefData {
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
    fn hash<H: Hasher>(&self, state: &mut H) {
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
    Lambda(Binder, Expr),
    App(Expr, Expr),
    Type(Ty),
}

pub type Binder = Idx<BinderData>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinderData {
    Val(Ty),
    Ty,
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TyData {
    // TODO flags
    kind: TyKind,
}

impl fmt::Debug for TyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl TyData {
    pub fn new(kind: TyKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &TyKind {
        &self.kind
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TyKind {
    Scalar(Scalar),
    Fn(Ty, Ty),
    Var(Debruijn),
    ForAll(Ty),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Debruijn(u32);

impl Debruijn {
    pub const INNER: Self = Self(0);

    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn within(self, other: Self) -> bool {
        self <= other
    }

    #[must_use]
    pub fn shifted_in(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Debug for TyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Scalar(scalar) => write!(f, "{:?}", scalar),
            TyKind::Fn(l, r) => write!(f, "({:?} -> {:?})", l, r),
            TyKind::Var(var) => write!(f, "{:?}", var),
            TyKind::ForAll(ty) => write!(f, "∀{:?}", ty),
        }
    }
}

impl TyKind {
    pub fn intern(self) -> Ty {
        Ty::intern(TyData::new(self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scalar {
    Bool,
    Int,
    Float,
}
