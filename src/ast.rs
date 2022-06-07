use crate::scanner::token::{LiteralType, Token};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Unary(Token, Box<Expr>),
}

pub trait Visitor<T> {
    fn handle(&self, expr: Expr) -> T {
        match expr {
            Expr::Binary(left, token, right) => self.visit_binary_expr(left, token, right),
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Unary(token, expr) => self.visit_unary_expr(token, expr),
        }
    }

    fn visit_binary_expr(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> T;
    fn visit_grouping_expr(&self, expr: Box<Expr>) -> T;
    fn visit_literal_expr(&self, lilteral: LiteralType) -> T;
    fn visit_unary_expr(&self, token: Token, expr: Box<Expr>) -> T;
}
