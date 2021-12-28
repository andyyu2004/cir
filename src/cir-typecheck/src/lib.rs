use cir::{Intern, Ty, TyData, TyKind};
use subst::Substitute;

struct TypecheckCtxt {
    body: cir::BodyData,
}

macro_rules! ty {
    (Bool) => {{ cir::TyKind::Scalar(cir::Scalar::Bool).intern() }};
    (Int) => {{ cir::TyKind::Scalar(cir::Scalar::Int).intern() }};
    ($($tt:tt)*) => {{ cir_parse::parse_ty(stringify!($($tt)*)) }};
}

impl TypecheckCtxt {
    fn check_body(&mut self) -> Ty {
        self.check_expr(self.body.expr)
    }

    fn binder(&self, binder: cir::Binder) -> &cir::BinderData {
        &self.body.binders[binder]
    }

    fn check_binder(&self, binder: cir::Binder) -> Ty {
        match self.binder(binder) {
            cir::BinderData::Val(ty) => Ty::clone(ty),
            cir::BinderData::Ty => panic!(),
        }
    }

    fn check_expr(&mut self, expr: cir::Expr) -> Ty {
        match self.body[expr] {
            cir::ExprData::Var(binder) => self.check_binder(binder),
            cir::ExprData::Lit(lit) => match lit {
                cir::Lit::Bool(_) => ty!(Bool),
                cir::Lit::Int(_) => ty!(Int),
            },
            cir::ExprData::Lambda(binder, body) => {
                let body_ty = self.check_expr(body);
                match self.binder(binder) {
                    cir::BinderData::Val(binder_ty) => {
                        let binder_ty = Ty::clone(binder_ty);
                        TyData::new(TyKind::Fn(binder_ty, body_ty)).intern()
                    }
                    cir::BinderData::Ty => TyKind::ForAll(body_ty).intern(),
                }
            }
            cir::ExprData::App(f, x) => match self.check_expr(f).kind() {
                TyKind::Fn(param_ty, ret_ty) => {
                    let arg_ty = self.check_expr(x);
                    if &arg_ty != param_ty {
                        todo!("type mismatch between argument and parameter");
                    }
                    Ty::clone(ret_ty)
                }
                TyKind::ForAll(body_ty) => {
                    let subst = match &self.body[x] {
                        cir::ExprData::Type(ty) => Ty::clone(&ty),
                        _ => todo!("expected type for type lambda"),
                    };
                    dbg!(&body_ty);
                    dbg!(&subst);
                    body_ty.substitute(&subst)
                }
                _ => todo!(),
            },
            cir::ExprData::Type(_) => unreachable!("found type in expression position"),
        }
    }
}

mod subst;
#[cfg(test)]
mod tests;
