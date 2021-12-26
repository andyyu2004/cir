use std::collections::{HashMap, VecDeque};

use crate::ast;

use la_arena::{Arena, Idx};

#[derive(Debug, Default)]
pub(crate) struct LowerCtxt {
    pub(crate) bodies: Arena<cir::BodyData>,
    value_defs: Arena<cir::ValueDef>,
    exprs: Arena<cir::ExprData>,
}

impl LowerCtxt {
    fn lower_source_file(&mut self, file: &ast::SourceFile) -> cir::Items {
        let items = file.items.iter().map(|item| self.lower_item(item)).collect();
        cir::Items { items }
    }

    fn lower_item(&mut self, item: &ast::Item) -> cir::Item {
        match &item.kind {
            ast::ItemKind::ValueDef(def) => cir::Item::ValueDef(self.lower_value_def(def)),
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
            ast::Type::Var(_) => todo!(),
            ast::Type::Scalar(scalar) => cir::TyKind::Scalar(*scalar),
            ast::Type::Fn(l, r) => cir::TyKind::Fn(self.lower_ty(l), self.lower_ty(r)),
            ast::Type::ForAll(_, _) => todo!(),
        };
        kind.intern()
    }

    fn lower_ty_var(&mut self, var: &ast::TyVar) -> cir::Ty {
        todo!()
    }

    fn lower_ty_fn(&mut self, l: &ast::Type, r: &ast::Type) -> cir::Ty {
        todo!()
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
    binder_map: HashMap<cir::Name, Vec<cir::Binder>>,
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
            ast::Expr::Lambda(binder, expr) =>
                self.in_binder(binder, |lcx, binder| cir::ExprData::Lambda(binder,lcx.lower_expr(expr))),
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
        let name = match name {
            ast::Var::Val { name } => name,
            ast::Var::Ty(_) => todo!(),
        };
        self.binder_map.get(name).and_then(|binders| binders.last().copied())
    }

    fn in_binder<R>(
        &mut self,
        binder: &ast::Binder,
        f: impl FnOnce(&mut Self, cir::Binder) -> R,
    ) -> R {
        let (name, binder_data) = match binder {
            ast::Binder::Val(name, ty) => (name, cir::BinderData::Val(self.lcx.lower_ty(ty))),
            ast::Binder::Ty(_) => todo!(),
        };
        let binder = self.binders.alloc(binder_data);
        self.binder_map.entry(name.clone()).or_default().push(binder);
        let r = f(self, binder);
        assert_eq!(self.binder_map.get_mut(name).unwrap().pop(), Some(binder));
        r
    }
}

#[cfg(test)]
mod tests;
