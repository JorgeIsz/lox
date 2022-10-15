// inspired by / copied shamelessly from https://github.com/brightly-salty/rox/blob/master/src/ast.rs

use crate::scanner::token::{LiteralType, Token};

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

pub trait Visitor<T> {
    fn handle_expr(&self, expr: Expr) -> T {
        match expr {
            Expr::Binary(left, token, right) => self.visit_binary_expr(left, token, right),
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Unary(token, expr) => self.visit_unary_expr(token, expr),
            Expr::Variable(token) => self.visit_variable_expr(token),
        }
    }

    fn visit_binary_expr(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> T;
    fn visit_grouping_expr(&self, expr: Box<Expr>) -> T;
    fn visit_literal_expr(&self, literal: LiteralType) -> T;
    fn visit_unary_expr(&self, token: Token, expr: Box<Expr>) -> T;
    fn visit_variable_expr(&self, token: Token) -> T;
}

pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Option<Box<Expr>>),
}

pub trait StmtVisitor {
    fn handle_stmt(&self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => self.visit_expression_stmt(expr),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Var(token, expr) => self.visit_var_stmt(token, expr),
        }
    }

    fn visit_expression_stmt(&self, expr: Box<Expr>);
    fn visit_print_stmt(&self, expr: Box<Expr>);
    fn visit_var_stmt(&self, token: Token, expr: Option<Box<Expr>>);
}
