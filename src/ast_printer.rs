use crate::ast::{Expr, Visitor};
use crate::scanner::token::{LiteralType, Token};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        self.handle(expr)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<Expr>) -> String {
        let mut res = String::new();
        res.push_str(&format!("({}", name));

        for expr in exprs {
            res.push_str(" ");
            res.push_str(&self.handle(expr));
        }
        res.push_str(")");

        res
    }
}
impl Visitor<String> for AstPrinter {
    fn visit_unary_expr(&self, token: Token, expr: Box<Expr>) -> String {
        self.parenthesize(&token.lexeme, vec![*expr])
    }

    fn visit_binary_expr(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> String {
        self.parenthesize(&token.lexeme, vec![*left, *right])
    }

    fn visit_literal_expr(&self, literal: LiteralType) -> String {
        match literal {
            LiteralType::LString(val) => val,
            LiteralType::LNumber(val) => val.to_string(),
        }
    }

    fn visit_grouping_expr(&self, expr: Box<Expr>) -> String {
        self.parenthesize("group", vec![*expr])
    }
}
