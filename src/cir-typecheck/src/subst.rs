use cir::{Debruijn, Ty, TyKind};

pub type Subst = Ty;

pub trait Substitute {
    fn substitute(&self, subst: &Subst) -> Self;
}

impl Substitute for Ty {
    fn substitute(&self, subst: &Subst) -> Self {
        substitute_ty(self, subst, Debruijn::INNER)
    }
}

fn substitute_ty(ty: &Ty, subst: &Subst, cutoff: Debruijn) -> Ty {
    match ty.kind() {
        TyKind::Scalar(_) => Ty::clone(ty),
        TyKind::Fn(f, x) =>
            TyKind::Fn(substitute_ty(f, subst, cutoff), substitute_ty(x, subst, cutoff)).intern(),
        TyKind::Var(debruijn) if *debruijn == cutoff => subst.clone(),
        TyKind::Var(_) => Ty::clone(ty),
        TyKind::ForAll(ty) =>
            TyKind::ForAll(substitute_ty(ty, subst, cutoff.shifted_in())).intern(),
    }
}
