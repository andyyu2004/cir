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
    // assert_eq!(check_expr("\\x: Int.5"), ty!(x -> y));
}
