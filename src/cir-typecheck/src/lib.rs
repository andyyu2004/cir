use cir::{BodyData, Expr, Scalar, Ty, TyData, TyKind};

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

    fn check_expr(&mut self, expr: Expr) -> Ty {
        let body = &self.body;
        match self.body[expr] {
            cir::ExprData::Var(_) => todo!(),
            cir::ExprData::Lit(lit) => match lit {
                cir::Lit::Bool(_) => ty!(Bool),
                cir::Lit::Int(_) => ty!(Int),
            },
            cir::ExprData::Lambda(_) => todo!(),
            cir::ExprData::App(f, x) => match self.check_expr(f).kind() {
                TyKind::Fn(_, _) => todo!(),
                _ => todo!(),
            },
            cir::ExprData::Type(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests;
