use cir::Ty;

use crate::TypecheckCtxt;

fn check_expr(s: &str) -> Ty {
    let body = cir_parse::parse_body(s);
    TypecheckCtxt { body }.check_body()
}

#[test]
fn test_typeck_scalar() {
    assert_eq!(check_expr("false"), ty!(Bool));
    assert_eq!(check_expr("true"), ty!(Bool));
    assert_eq!(check_expr("42"), ty!(Int));
}

#[test]
fn test_typeck_simple_lambda() {
    assert_eq!(check_expr("\\x: Bool.0"), ty!(Bool -> Int));
    assert_eq!(check_expr("\\x: Int.x"), ty!(Int -> Int));
    assert_eq!(check_expr("\\x: Int. \\y: Bool.x"), ty!(Int -> Bool -> Int));
    assert_eq!(check_expr("\\x: Int. \\y: Bool.y"), ty!(Int -> Bool -> Bool));
}

#[test]
fn test_typeck_higher_order_lambda() {
    assert_eq!(check_expr("\\p:(Int -> Bool).\\x: Int.(p x)"), ty!((Int -> Bool) -> Int -> Bool));
    assert_eq!(
        check_expr("\\p:(Int -> Bool).\\x: Int.p"),
        ty!((Int -> Bool) -> Int -> Int -> Bool)
    );
}

#[test]
fn test_typeck_type_lambda() {
    // what does it mean to have a type abstraction with no value abstraction? e.g. \@a.0
    assert_eq!(check_expr("\\@a.\\x:a.x"), ty!(forall a. a -> a));
}

#[test]
fn test_typeck_type_application() {
    // what does it mean to have a type abstraction with no value abstraction? e.g. \@a.0
    assert_eq!(check_expr("(\\@a.\\x:a.x) @Int"), ty!(Int -> Int));
    assert_eq!(check_expr("(\\@a.\\@b.\\x:a.\\y:b.x) @Int @Bool"), ty!(Int -> Bool -> Int));
}

#[test]
fn test_typeck_partial_type_application() {
    assert_eq!(check_expr("(\\@a.\\@b.\\x:a.\\y:b.x) @Int"), ty!(forall b. Int -> b -> Int));
    // Check the names of forall binders are not meaningful for equality
    assert_eq!(check_expr("(\\@a.\\@b.\\x:a.\\y:b.x) @Int"), ty!(forall a. Int -> a -> Int));
}
#[test]
fn test_typeck_simple_app() {
    assert_eq!(check_expr("(\\x:Int.x) 5"), ty!(Int));
}

#[test]
fn test_typeck_higher_order_app() {
    assert_eq!(check_expr("(\\p:Int -> Bool.\\x:Int.p x) (\\x:Int.false) 0"), ty!(Bool));
}
