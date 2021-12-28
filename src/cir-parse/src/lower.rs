use cir::Debruijn;
use enum_map::{Enum, EnumMap};
use std::collections::HashMap;

use crate::ast;

use la_arena::{Arena, Idx};

#[derive(Debug, Default)]
pub(crate) struct LowerCtxt {
    pub(crate) bodies: Arena<cir::BodyData>,
    value_defs: Arena<cir::ValueDef>,
    exprs: Arena<cir::ExprData>,
    foralls: Vec<cir::Name>,
}

impl LowerCtxt {
    fn lower_source_file(&mut self, file: &ast::SourceFile) -> cir::Items {
        let items = file.items.iter().map(|item| self.lower_item(item)).collect();
        cir::Items { items }
    }

    fn lower_item(&mut self, item: &ast::Item) -> cir::Item {
        match &item.kind {
            ast::ItemKind::ValueDef(def) => cir::Item::ValueDef(self.lower_value_def(def)),
            ast::ItemKind::DataDef(_) => todo!(),
        }
    }

    fn lower_value_def(&mut self, value_def: &ast::ValueDef) -> Idx<cir::ValueDef> {
        let ast::ValueDef { name, ty, expr } = value_def;
        let value_def = cir::ValueDef {
            name: name.clone(),
            ty: self.lower_ty(ty),
            body: self.lower_body(expr),
        };
        self.value_defs.alloc(value_def)
    }

    pub(crate) fn lower_ty(&mut self, ty: &ast::Type) -> cir::Ty {
        let kind = match &ty {
            ast::Type::Var(var) => cir::TyKind::Var(self.lower_ty_var(var)),
            ast::Type::Scalar(scalar) => cir::TyKind::Scalar(*scalar),
            ast::Type::Fn(l, r) => cir::TyKind::Fn(self.lower_ty(l), self.lower_ty(r)),
            // TODO not sure how to deal with var
            ast::Type::ForAll(var, ty) =>
                self.in_forall(var, |lcx| cir::TyKind::ForAll(lcx.lower_ty(ty))),
        };
        kind.intern()
    }

    fn lower_ty_var(&mut self, var: &ast::TyVar) -> Debruijn {
        let index = self.foralls.iter().rev().position(|name| name == &var.name).unwrap();
        Debruijn::new(index as u32)
    }

    fn in_forall<R>(&mut self, var: &ast::TyVar, f: impl FnOnce(&mut Self) -> R) -> R {
        self.foralls.push(var.name.clone());
        let r = f(self);
        assert_eq!(self.foralls.pop().unwrap(), var.name);
        r
    }

    fn lower_body(&mut self, expr: &ast::Expr) -> cir::Body {
        BodyLowerCtxt::new(self).lower(expr)
    }
}

#[derive(Debug)]
pub(crate) struct BodyLowerCtxt<'lcx> {
    lcx: &'lcx mut LowerCtxt,
    exprs: Arena<cir::ExprData>,
    binders: Arena<cir::BinderData>,
    binder_map: Namespaced<HashMap<cir::Name, Vec<cir::Binder>>>,
}

type Namespaced<T> = EnumMap<Ns, T>;

#[derive(Debug, Hash, Enum, Clone, Copy, PartialEq, Eq)]
enum Ns {
    Val,
    Ty,
}

impl<'lcx> BodyLowerCtxt<'lcx> {
    pub fn new(lcx: &'lcx mut LowerCtxt) -> Self {
        Self {
            lcx,
            exprs: Default::default(),
            binders: Default::default(),
            binder_map: Default::default(),
        }
    }

    pub(crate) fn lower(mut self, expr: &ast::Expr) -> cir::Body {
        let expr = self.lower_expr(expr);
        let Self { exprs, binders, .. } = self;
        self.lcx.bodies.alloc(cir::BodyData::new(expr, exprs, binders))
    }

    fn lower_expr(&mut self, expr: &ast::Expr) -> cir::Expr {
        let expr = match &expr {
            ast::Expr::Var(var) => self.lower_var_expr(var),
            ast::Expr::Lit(lit) => cir::ExprData::Lit(match lit.kind {
                ast::LiteralKind::Int(i) => cir::Lit::Int(i),
                ast::LiteralKind::Bool(b) => cir::Lit::Bool(b),
            }),
            ast::Expr::Lambda(binder, expr) => self.in_binder(binder, |bcx, binder| {
                cir::ExprData::Lambda(binder, bcx.lower_expr(expr))
            }),
            ast::Expr::App(f, x) => cir::ExprData::App(self.lower_expr(f), self.lower_expr(x)),
            ast::Expr::Type(ty) => cir::ExprData::Type(self.lcx.lower_ty(ty)),
        };
        self.exprs.alloc(expr)
    }

    fn lower_var_expr(&self, var: &ast::Var) -> cir::ExprData {
        let binder = match self.lookup_var(&var) {
            Some(binder) => binder,
            None => todo!("unbound var: {:?}", var),
        };
        cir::ExprData::Var(binder)
    }

    fn lookup_var(&self, name: &ast::Var) -> Option<cir::Binder> {
        let (name, ns) = match name {
            ast::Var::Val { name } => (name, Ns::Val),
            ast::Var::Ty(_) => todo!(),
        };

        self.binder_map[ns].get(name).and_then(|binders| binders.last().copied())
    }

    fn in_binder<R>(
        &mut self,
        binder: &ast::Binder,
        f: impl FnOnce(&mut Self, cir::Binder) -> R,
    ) -> R {
        match binder {
            ast::Binder::Val(name, ty) => {
                // (name, Ns::Val, cir::BinderData::Val(self.lcx.lower_ty(ty)));
                let ns = Ns::Val;
                let binder_data = cir::BinderData::Val(self.lcx.lower_ty(ty));
                let binder = self.binders.alloc(binder_data);
                self.binder_map[ns].entry(name.clone()).or_default().push(binder);
                let r = f(self, binder);
                assert_eq!(self.binder_map[ns].get_mut(name).unwrap().pop(), Some(binder));
                r
            }
            ast::Binder::Ty(var) => {
                // (&var.name, Ns::Ty, cir::BinderData::Ty),
                // FIXME hack (copying `in_forall` impl for now)
                self.lcx.foralls.push(var.name.clone());

                // FIXME do we need this binder
                let binder_data = cir::BinderData::Ty;
                let binder = self.binders.alloc(binder_data);
                let r = f(self, binder);
                assert_eq!(self.lcx.foralls.pop().unwrap(), var.name);
                r
            }
        }
    }
}

#[cfg(test)]
mod tests;
