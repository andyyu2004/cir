use cir::{BinderData, Expr, ExprData, TyKind};
use la_arena::RawIdx;

use crate::{parse_body, parse_ty};

use super::*;

#[test]
fn test_lower_program() -> anyhow::Result<()> {
    let source = crate::cirparser::source_file("let x: Int = 1; let y: Bool = false;")?;
    let mut lcx = LowerCtxt::default();
    let file = lcx.lower_source_file(&source);
    assert_eq!(file.items.len(), 2);
    Ok(())
}

#[test]
fn test_lower_value_def() -> anyhow::Result<()> {
    macro_rules! lower {
        ($s:expr) => {{
            let value_def = crate::cirparser::value_def($s)?;
            let mut lcx = LowerCtxt::default();
            lcx.lower_value_def(&value_def);
        }};
    }

    lower!("let x: Int = 1");
    lower!("let f: forall a. forall b. (a -> b) -> a -> b = \\@a.\\@b.\\f:a -> b.\\x:a.f x");
    lower!("let id: forall a. a -> a = \\@t.\\x:t.x");
    Ok(())
}

#[test]
fn test_lower_binders() -> anyhow::Result<()> {
    let body = parse_body("\\x: Int.x");
    assert_eq!(body.expr, Expr::from_raw(RawIdx::from(1)));
    let expr = &body.exprs[Idx::from_raw(RawIdx::from(0))];
    let binder = match expr {
        ExprData::Var(binder) => *binder,
        _ => panic!(),
    };
    let binder = &body.binders[binder];
    assert_eq!(binder, &BinderData::Val(TyKind::Scalar(cir::Scalar::Int).intern()));
    Ok(())
}

#[test]
fn test_lower_universal_type() -> anyhow::Result<()> {
    let _ty = parse_ty("forall a. a -> a");
    let body = parse_body("(\\@a.\\x:a.x) @Int 0");
    Ok(())
}

#[test]
fn test_lower_type_binders() -> anyhow::Result<()> {
    let _body = parse_body("\\@a.\\x:a.x");
    Ok(())
}

#[test]
fn test_lower_binders_shadows() -> anyhow::Result<()> {
    // FIXME this test is pretty verbose and brittle
    // Hopefully there is a better way than manually constructing the indices
    let body = parse_body("\\x:Int.\\x:Bool.x");
    assert_eq!(body.expr, Expr::from_raw(RawIdx::from(2)));

    let expr = &body.exprs[Idx::from_raw(RawIdx::from(0))];
    let binder = match expr {
        ExprData::Var(binder) => *binder,
        _ => panic!(),
    };
    let binder = &body.binders[binder];
    assert_eq!(binder, &BinderData::Val(TyKind::Scalar(cir::Scalar::Bool).intern()));

    Ok(())
}
