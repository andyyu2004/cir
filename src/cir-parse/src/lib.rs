mod ast;
mod lower;

use ast::*;
use cir::Name;

use codespan::Span;

peg::parser! {
    grammar cirparser() for str {
        rule lower() -> &'input str = s:$(['_' | 'a'..='z'] alphanumeric()?) { s }
        rule upper() -> &'input str = s:$(['A'..='Z'] alphanumeric()?) { s }
        rule alphanumeric() -> &'input str = s:$(['_' | 'a'..='z' | 'A'..='Z' | '0'..='9']+) { s }
        rule integer() -> i64 = n:$("-"?['0'..='9']+) { n.parse().unwrap() }
        rule bool() -> bool = b:$("false" / "true") { b.parse().unwrap() }
        rule _ = [' ' | '\t' | '\n' | '\r']*

        rule spanned<T>(t: rule<T>) -> Spanned<T> = start:position!() node:t() end:position!() {
            Spanned {
                span: Span::new(start as u32, end as u32),
                node,
            }
        }

        pub rule integer_literal() -> Literal = i:spanned(<integer()>) {
            Literal {
                span: i.span,
                kind: LiteralKind::Int(i.node),
            }
        }

        pub rule boolean_literal() -> Literal = b:spanned(<bool()>) {
            Literal {
                span: b.span,
                kind: LiteralKind::Bool(b.node),
            }
        }

        pub rule literal() -> Literal = _ lit:(integer_literal() / boolean_literal()) _ {
            lit
        }

        pub rule lname() -> Name = s:spanned(<lower()>) {
            Name::new(s.span, s.node)
        }

        pub rule uname() -> Name = s:spanned(<upper()>) {
            Name::new(s.span, s.node)
        }

        rule expr_atom() -> Expr = precedence! {
            "(" expr:expr() ")" { expr }
            "\\" var:lname() _ "->" _ expr:expr() { Expr::Lambda(var, Box::new(expr)) }
            lit:literal() { Expr::Lit(lit) }
            name:lname() { Expr::Var(Var { name }) }
        }

        pub rule expr() -> Expr = precedence! {
            f:(@) " " x:@ { Expr::App(Box::new(f), Box::new(x)) }
            _ atom:expr_atom() { atom }
        }


        rule ty_atom() -> Ty = precedence! {
            "bool"  { Ty::Scalar(cir::Scalar::Bool) }
            "int" { Ty::Scalar(cir::Scalar::Int) }
            "forall" _ var:lname() "." ty:ty() { Ty::ForAll(var, Box::new(ty)) }
            "(" ty:ty() ")" { ty }
            tyvar:lname() { Ty::Var(TyVar { name: tyvar }) }
        }

        pub rule ty() -> Ty= precedence! {
             l:@ _ "->" _ r:(@)   { Ty::Fn(Box::new(l), Box::new(r)) }
             --
            _ atom:ty_atom() _ { atom }
        }

        pub rule value_def() -> ValueDef = _ "let" _ name:lname() _ "::"  _ ty:ty() _ "=" _ expr:expr() _ {
            ValueDef { name, expr, ty }
        }

        pub rule value_def_item() -> Item = _ def:spanned(<value_def()>) _ {
            Item {
                span: def.span,
                kind: ItemKind::ValueDef(def.node)
            }
        }

        pub rule item() -> Item = item:value_def_item() {
            item
        }

        pub rule source_file() -> SourceFile = _ items:(items:item())* {
            SourceFile { items }
        }
    }
}

#[cfg(test)]
mod tests;
