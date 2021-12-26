use cir::{Ty, TyKind};

pub type Subst = [Ty];

pub trait Substitute {
    fn substitute(&self, subst: &Subst) -> Self;
}

impl Substitute for Ty {
    fn substitute(&self, subst: &Subst) -> Self {
        match self.kind() {
            TyKind::Scalar(_) => self.clone(),
            TyKind::Fn(f, x) => TyKind::Fn(f.substitute(subst), x.substitute(subst)).intern(),
            // FIXME this is obviously wrong
            TyKind::Var(_) => Ty::clone(&subst[0]),
            // FIXME this is probably not right too
            TyKind::ForAll(ty) => TyKind::ForAll(ty.substitute(subst)).intern(),
        }
    }
}
