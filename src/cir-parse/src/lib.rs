use codespan::Span;
use smol_str::SmolStr;

peg::parser! {
    grammar cirparser() for str {
        rule lower() -> &'input str = s:$(['_' | 'a'..='z'] alphanumeric()?) { s }
        rule upper() -> &'input str = s:$(['A'..='Z'] alphanumeric()?) { s }
        rule alphanumeric() -> &'input str = s:$(['_' | 'a'..='z' | 'A'..='Z' | '0'..='9']+) { s }

        rule spanned<T>(t: rule<T>) -> Spanned<T> = start:position!() node:t() end:position!() {
            Spanned {
                span: Span::new(start as u32, end as u32),
                node,
            }
        }

        pub rule lname() -> Name = s:spanned(<lower()>) {
            Name { span: s.span, symbol: SmolStr::new(s.node) }
        }

        pub rule uname() -> Name = s:spanned(<upper()>) {
            Name { span: s.span, symbol: SmolStr::new(s.node) }
        }

        pub rule expr_kind() -> ExprKind = precedence! {
            var:spanned(<lname()>) { ExprKind::Var(Var { name: var.node }) }
        }

        pub rule expr() -> Expr = kind:spanned(<expr_kind()>) {
            Expr {
                span: kind.span,
                kind: kind.node,
            }
        }

        pub rule ty_kind() -> TyKind = precedence! {
            tyvar:spanned(<lname()>) { TyKind::Var(TyVar { name: tyvar.node }) }
        }

        pub rule ty() -> Ty = kind:spanned(<ty_kind()>) {
            Ty {
                span: kind.span,
                kind: kind.node,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expr {
    span: Span,
    kind: ExprKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprKind {
    Var(Var),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ty {
    span: Span,
    kind: TyKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Var {
    name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TyVar {
    name: Name,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyKind {
    Var(TyVar),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    pub span: Span,
    pub symbol: SmolStr,
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_expr() -> anyhow::Result<()> {
        assert_eq!(
            cirparser::expr("x")?,
            Expr {
                span: Span::new(0, 1),
                kind: ExprKind::Var(Var {
                    name: Name { span: Span::new(0, 1), symbol: "x".into() }
                })
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_ty() -> anyhow::Result<()> {
        assert_eq!(
            cirparser::ty("a")?,
            Ty {
                span: Span::new(0, 1),
                kind: TyKind::Var(TyVar {
                    name: Name { span: Span::new(0, 1), symbol: "a".into() }
                })
            }
        );
        Ok(())
    }
}
