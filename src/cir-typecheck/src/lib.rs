use cir::*;

struct TypecheckCtxt {
    body: BodyData,
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

    fn check_binder(&self, binder: Binder) -> Ty {
        match &self.body.binders[binder] {
            cir::BinderData::Val(ty) => Ty::clone(ty),
        }
    }

    fn check_expr(&mut self, expr: Expr) -> Ty {
        match self.body[expr] {
            cir::ExprData::Var(binder) => self.check_binder(binder),
            cir::ExprData::Lit(lit) => match lit {
                cir::Lit::Bool(_) => ty!(Bool),
                cir::Lit::Int(_) => ty!(Int),
            },
            cir::ExprData::Lambda(binder, body) => {
                let binder_ty = self.check_binder(binder);
                let body_ty = self.check_expr(body);
                TyData::new(TyKind::Fn(binder_ty, body_ty)).intern()
            }
            cir::ExprData::App(f, x) => match self.check_expr(f).kind() {
                TyKind::Fn(param_ty, ret_ty) => {
                    let arg_ty = self.check_expr(x);
                    if &arg_ty != param_ty {
                        todo!("type mismatch between argument and parameter");
                    }
                    Ty::clone(ret_ty)
                }
                _ => todo!(),
            },
            cir::ExprData::Type(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests;
