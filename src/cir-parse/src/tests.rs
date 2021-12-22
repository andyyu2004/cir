use super::*;
use expect_test::expect_file;

#[test]
fn test_parse_lname() -> anyhow::Result<()> {
    assert_eq!(cirparser::lname("x")?.symbol, "x");
    assert_eq!(cirparser::lname("lowerIdent")?.symbol, "lowerIdent");
    assert_eq!(cirparser::lname("lower123")?.symbol, "lower123");
    assert_eq!(cirparser::lname("_lower")?.symbol, "_lower");
    assert!(cirparser::lname("Upper123").is_err());
    Ok(())
}

#[test]
fn test_parse_uname() -> anyhow::Result<()> {
    assert_eq!(cirparser::uname("X")?.symbol, "X");
    assert_eq!(cirparser::uname("UpperIdent")?.symbol, "UpperIdent");
    assert_eq!(cirparser::uname("Upper123")?.symbol, "Upper123");
    assert!(cirparser::uname("lower123").is_err());
    Ok(())
}

#[test]
fn test_parse_lit() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::literal(" false ")?,
        Literal { span: Span::new(1, 6), kind: LiteralKind::Bool(false) }
    );
    assert_eq!(
        cirparser::literal("  true ")?,
        Literal { span: Span::new(2, 6), kind: LiteralKind::Bool(true) }
    );
    assert_eq!(
        cirparser::literal(" -288 ")?,
        Literal { span: Span::new(1, 5), kind: LiteralKind::Int(-288) }
    );
    Ok(())
}

#[test]
fn test_parse_expr() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::expr("x")?,
        Expr {
            span: Span::new(0, 1),
            kind: ExprKind::Var(Var { name: Name { span: Span::new(0, 1), symbol: "x".into() } })
        }
    );
    Ok(())
}

#[test]
fn test_parse_ty() -> anyhow::Result<()> {
    assert_eq!(cirparser::ty("int")?, ast::Ty::Scalar(cir::Scalar::Int));
    assert_eq!(cirparser::ty("bool")?, ast::Ty::Scalar(cir::Scalar::Bool));
    expect_file!["tests/expect/ty/arrow-simple.ast"].assert_debug_eq(&cirparser::ty(" a -> b ")?);
    expect_file!["tests/expect/ty/arrow-right-associative.ast"]
        .assert_debug_eq(&cirparser::ty(" a -> b -> c ")?);
    Ok(())
}

#[test]
fn test_parse_value_def() -> anyhow::Result<()> {
    let value_def = ValueDef {
        name: Name { span: Span::new(5, 6), symbol: "x".into() },
        ty: Ty::Var(TyVar { name: Name { span: Span::new(10, 11), symbol: "a".into() } }),
        expr: Expr {
            span: Span::new(14, 15),
            kind: ExprKind::Var(Var { name: Name { span: Span::new(14, 15), symbol: "k".into() } }),
        },
    };
    assert_eq!(cirparser::value_def(" let x :: a = k ")?, value_def);
    Ok(())
}

#[test]
fn test_parse_ty_arrow() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::ty("a")?,
        Ty::Var(TyVar { name: Name { span: Span::new(0, 1), symbol: "a".into() } })
    );
    Ok(())
}

#[test]
fn test_parse_item() -> anyhow::Result<()> {
    expect_file!["tests/expect/item/value_def.ast"]
        .assert_debug_eq(&cirparser::item(" let x :: a = k ")?);
    Ok(())
}

#[test]
fn test_parse_source_file() -> anyhow::Result<()> {
    expect_file!["tests/expect/file/value_defs.ast"]
        .assert_debug_eq(&cirparser::source_file(" let x :: a = k\n let y :: b = g")?);
    Ok(())
}
