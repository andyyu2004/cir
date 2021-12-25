use cir::{Debruijn, Path};
use la_arena::{Arena, Idx};

use crate::ast::{self};

#[derive(Debug, Default)]
struct LowerCtxt {
    value_defs: Arena<cir::ValueDef>,
    bodies: Arena<cir::BodyData>,
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

    fn lower_ty(&self, ty: &ast::Ty) -> cir::Ty {
        let kind = match &ty {
            ast::Ty::Var(_) => todo!(),
            ast::Ty::Scalar(scalar) => cir::TyKind::Scalar(*scalar),
            ast::Ty::Fn(l, r) => cir::TyKind::Fn(self.lower_ty(l), self.lower_ty(r)),
            ast::Ty::ForAll(_, _) => todo!(),
        };
        kind.intern()
    }

    fn lower_ty_var(&mut self, var: &ast::TyVar) -> cir::Ty {
        todo!()
    }

    fn lower_ty_fn(&mut self, l: &ast::Ty, r: &ast::Ty) -> cir::Ty {
        todo!()
    }

    fn lower_expr(&mut self, expr: &ast::Expr) -> cir::Expr {
        let expr = match &expr {
            ast::Expr::Var(var) => cir::ExprData::Var(todo!()),
            ast::Expr::Lit(lit) => cir::ExprData::Lit(match lit.kind {
                ast::LiteralKind::Int(i) => cir::Lit::Int(i),
                ast::LiteralKind::Bool(b) => cir::Lit::Bool(b),
            }),
            ast::Expr::Lambda(binder, expr) =>
                self.in_binder(binder, |lcx| cir::ExprData::Lambda(lcx.lower_expr(expr))),
            ast::Expr::App(f, x) => cir::ExprData::App(self.lower_expr(f), self.lower_expr(x)),
        };
        self.exprs.alloc(expr)
    }

    fn in_binder<R>(&mut self, binder: &ast::Var, f: impl FnOnce(&mut Self) -> R) -> R {
        f(self)
    }

    fn lower_body(&mut self, expr: &ast::Expr) -> cir::Body {
        let body = cir::BodyData { expr: self.lower_expr(expr) };
        self.bodies.alloc(body)
    }
}

#[cfg(test)]
mod tests;
