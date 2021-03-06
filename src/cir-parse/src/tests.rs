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
fn test_parse_expr_lit() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::expr("x")?,
        Expr::Var(Var::Val { name: Name { span: Span::new(0, 1), symbol: "x".into() } })
    );
    Ok(())
}

#[test]
fn test_parse_expr_group() -> anyhow::Result<()> {
    expect_file!["tests/expect/expr/group.ast"]
        .assert_debug_eq(&cirparser::expr("(\\x: a. x) (\\y: b.  y)")?);
    Ok(())
}

#[test]
fn test_parse_expr_app() -> anyhow::Result<()> {
    expect_file!["tests/expect/expr/app.ast"].assert_debug_eq(&cirparser::expr("f x")?);
    expect_file!["tests/expect/expr/app-left-assoc.ast"]
        .assert_debug_eq(&cirparser::expr("f x y")?);
    expect_file!["tests/expect/expr/lambda-app.ast"]
        .assert_debug_eq(&cirparser::expr("(\\x: a. x) y")?);
    assert_ne!(cirparser::expr("(\\x: b. x) y")?, cirparser::expr("\\x: b. x y")?);

    expect_file!["tests/expect/expr/type-lambda-app.ast"]
        .assert_debug_eq(&cirparser::expr("(\\@a. 0) @Int")?);
    expect_file!["tests/expect/expr/type-lambda-app-nested.ast"]
        .assert_debug_eq(&cirparser::expr("(\\@a. \\x: a. x) @Int 5")?);
    Ok(())
}

#[test]
fn test_parse_binder() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::binder("x: a")?,
        Binder::Val(
            Name::new(Span::new(0, 1), "x"),
            Type::Var(TyVar { name: Name::new(Span::new(3, 4), "a") })
        )
    );

    assert_eq!(
        cirparser::binder("@t")?,
        Binder::Ty(TyVar { name: Name::new(Span::new(1, 2), "t") })
    );
    Ok(())
}

#[test]
fn test_parse_lambda() -> anyhow::Result<()> {
    expect_file!["tests/expect/expr/lambda.ast"].assert_debug_eq(&cirparser::expr("\\x: a. x")?);
    expect_file!["tests/expect/expr/nested-lambda.ast"]
        .assert_debug_eq(&cirparser::expr("\\x: a. \\y: b. x")?);
    expect_file!["tests/expect/expr/type-lambda.ast"]
        .assert_debug_eq(&cirparser::expr("\\@a. \\x: a. x")?);
    Ok(())
}

#[test]
fn test_parse_ty() -> anyhow::Result<()> {
    assert_eq!(cirparser::ty("Int")?, ast::Type::Scalar(cir::Scalar::Int));
    assert_eq!(cirparser::ty("((Int))")?, ast::Type::Scalar(cir::Scalar::Int));
    assert_eq!(cirparser::ty("Bool")?, ast::Type::Scalar(cir::Scalar::Bool));
    assert_eq!(
        cirparser::ty("a")?,
        ast::Type::Var(TyVar { name: Name { symbol: "a".into(), span: Span::new(0, 1) } })
    );
    expect_file!["tests/expect/ty/arrow-simple.ast"].assert_debug_eq(&cirparser::ty("a -> b")?);
    expect_file!["tests/expect/ty/arrow-right-assoc.ast"]
        .assert_debug_eq(&cirparser::ty("a -> b -> c")?);
    expect_file!["tests/expect/ty/forall.ast"].assert_debug_eq(&cirparser::ty("forall a.a -> a")?);
    expect_file!["tests/expect/ty/nested-forall.ast"]
        .assert_debug_eq(&cirparser::ty("forall a. forall b. a -> b")?);
    Ok(())
}

#[test]
fn test_parse_value_def() -> anyhow::Result<()> {
    let value_def = ValueDef {
        name: Name { span: Span::new(5, 6), symbol: "x".into() },
        ty: Type::Var(TyVar { name: Name { span: Span::new(8, 9), symbol: "a".into() } }),
        expr: Expr::Var(Var::Val { name: Name { span: Span::new(12, 13), symbol: "k".into() } }),
    };
    assert_eq!(cirparser::value_def(" let x: a = k ")?, value_def);
    Ok(())
}

#[test]
fn test_parse_ty_arrow() -> anyhow::Result<()> {
    assert_eq!(
        cirparser::ty("a")?,
        Type::Var(TyVar { name: Name { span: Span::new(0, 1), symbol: "a".into() } })
    );
    Ok(())
}

#[test]
fn test_parse_item() -> anyhow::Result<()> {
    expect_file!["tests/expect/item/value-def.ast"]
        .assert_debug_eq(&cirparser::item(" let x: a = k ; ")?);
    Ok(())
}

#[test]
fn test_parse_source_file() -> anyhow::Result<()> {
    expect_file!["tests/expect/file/value-defs.ast"]
        .assert_debug_eq(&cirparser::source_file(" let x: a = k; let y: b = g;")?);
    Ok(())
}

#[test]
fn test_parse_data_def() -> anyhow::Result<()> {
    expect_file!["tests/expect/data/simple-data-def.ast"]
        .assert_debug_eq(&cirparser::source_file("data T = A;")?);
    expect_file!["tests/expect/data/simple-data-def-multi-variant.ast"]
        .assert_debug_eq(&cirparser::source_file("data T = A | B;")?);
    expect_file!["tests/expect/data/data-def-either.ast"]
        .assert_debug_eq(&cirparser::source_file("data Either a b = Left a | Right b;")?);
    Ok(())
}

#[test]
fn test_parse_tycon_and_datacon_path_ref() -> anyhow::Result<()> {
    let src = r#"
        data T = T;
        data Y = Y;
        let t2y: T -> Y = \t:T.Y;
    "#;
    expect_file!["tests/expect/ty/path.ast"].assert_debug_eq(&cirparser::source_file(src)?);
    Ok(())
}
