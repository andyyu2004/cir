mod ast;
mod lower;

use ast::*;
use cir::Name;

use codespan::Span;

use self::lower::{BodyLowerCtxt, LowerCtxt};

// FIXME minor hack for testing purposes for now
pub fn parse_body(s: &str) -> cir::BodyData {
    let expr: ast::Expr = cirparser::expr(s).unwrap();
    let mut lcx = LowerCtxt::default();
    let body_id = BodyLowerCtxt::new(&mut lcx).lower(&expr);
    lcx.bodies[body_id].clone()
}

pub fn parse_ty(s: &str) -> cir::Ty {
    let ty: ast::Type = cirparser::ty(s).unwrap();
    let mut lcx = LowerCtxt::default();
    lcx.lower_ty(&ty)
}

peg::parser! {
    pub grammar cirparser() for str {
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


        pub rule tyvar() -> TyVar = name:lname() {
            TyVar { name }
        }

        pub rule var() -> Var = precedence! {
            tyvar:tyvar() { Var::Ty(tyvar) }
            name:lname() { Var::Val { name } }
        }

        pub rule binder() -> Binder = precedence! {
            name:lname() _ ":" _ ty:ty() { Binder::Val(name, ty) }
            "@" tyvar:tyvar() { Binder::Ty(tyvar) }
        }

        rule expr_atom() -> Expr = precedence! {
            "(" expr:expr() ")" { expr }
            "\\" _ binder:binder() _ "." _ expr:expr() { Expr::Lambda(binder, Box::new(expr)) }
            "@" ty:ty() { Expr::Type(ty) }
            lit:literal() { Expr::Lit(lit) }
            name:lname() { Expr::Var(Var::Val { name }) }
        }

        pub rule expr() -> Expr = precedence! {
            f:(@) " " x:@ { Expr::App(Box::new(f), Box::new(x)) }
            _ atom:expr_atom() { atom }
        }


        rule ty_atom() -> Type = precedence! {
            "Bool"  { Type::Scalar(cir::Scalar::Bool) }
            "Int" { Type::Scalar(cir::Scalar::Int) }
            "forall" _ tyvar:tyvar() _ "." _ ty:ty() { Type::ForAll(tyvar, Box::new(ty)) }
            "(" ty:ty() ")" { ty }
            name:lname() { Type::Var(TyVar { name }) }
        }

        pub rule ty() -> Type = precedence! {
             l:@ _ "->" _ r:(@) { Type::Fn(Box::new(l), Box::new(r)) }
             --
            atom:ty_atom() { atom }
        }

        pub rule value_def() -> ValueDef = _ "let" _ name:lname() _ ":"  _ ty:ty() _ "=" _ expr:expr() _ {
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
