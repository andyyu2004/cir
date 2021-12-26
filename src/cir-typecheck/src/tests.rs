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
fn test_typeck_simple_app() {
    assert_eq!(check_expr("(\\x:Int.x) 5"), ty!(Int));
}
