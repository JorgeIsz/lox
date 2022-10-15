use crate::{
    ast::{Expr, Stmt, StmtVisitor, Visitor},
    environment::Environment,
    scanner::{
        token::{LiteralType, Token},
        tokenType::TokenType,
    },
};

#[derive(Clone)]
pub struct LoxValue {
    value: LiteralType,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&self, stmt: Stmt) {
        self.handle_stmt(stmt);
    }

    fn evaluate(&self, expr: &Expr) -> LoxValue {
        self.handle_expr(expr.clone())
    }

    fn is_truthy(&self, object: Option<LoxValue>) -> bool {
        match object {
            None => false,
            Some(lox_value) => match lox_value.value {
                LiteralType::LBoolean(literal) => literal,
                _ => true,
            },
        }
    }
}

impl Visitor<LoxValue> for Interpreter {
    fn visit_literal_expr(&self, literal: LiteralType) -> LoxValue {
        LoxValue { value: literal }
    }

    fn visit_grouping_expr(&self, expr: Box<Expr>) -> LoxValue {
        self.evaluate(expr.as_ref())
    }

    fn visit_unary_expr(&self, token: Token, expr: Box<Expr>) -> LoxValue {
        let right = self.evaluate(expr.as_ref());
        let mut value = LiteralType::LNil;
        let mut inner_value = 0f64;
        if let LiteralType::LNumber(val) = right.value {
            inner_value = val;
        };

        if let Expr::Unary(_, _) = *expr {
            match token.token_type {
                TokenType::Minus => value = LiteralType::LNumber(inner_value * -1_f64),
                TokenType::Bang => value = LiteralType::LBoolean(self.is_truthy(Some(right))),
                _ => (),
            }
        }

        LoxValue { value }
    }

    fn visit_binary_expr(&self, left: Box<Expr>, token: Token, right: Box<Expr>) -> LoxValue {
        let i_left = self.evaluate(left.as_ref());
        let i_right = self.evaluate(right.as_ref());

        let value = match token.token_type {
            TokenType::Greater => i_left.value.greater(i_right.value),
            TokenType::GreaterEqual => i_left.value.greater_equal(i_right.value),
            TokenType::Less => i_left.value.less(i_right.value),
            TokenType::LessEqual => i_left.value.less_equal(i_right.value),
            TokenType::EqualEqual => i_left.value.equal(i_right.value),
            TokenType::BangEqual => i_left.value.not_equal(i_right.value),
            TokenType::Minus => i_left.value - i_right.value,
            TokenType::Slash => i_left.value / i_right.value,
            TokenType::Star => i_left.value * i_right.value,
            TokenType::Plus => i_left.value + i_right.value,
            _ => panic!("invalid operator"),
        };

        LoxValue { value }
    }

    fn visit_variable_expr(&self, token: Token) -> LoxValue {
        self.environment.get(&token.lexeme).unwrap()
    }
}

impl StmtVisitor for Interpreter {
    fn visit_print_stmt(&self, expr: Box<Expr>) {
        let value = self.evaluate(&expr);
        println!("{}", value.value.stringify());
    }

    fn visit_expression_stmt(&self, expr: Box<Expr>) {
        self.evaluate(&expr);
    }

    fn visit_var_stmt(&self, token: Token, expr: Option<Box<Expr>>) {
        let mut value: Option<LoxValue> = None;
        if let Some(var) = expr {
            value = Some(self.evaluate(&var));
        }

        self.environment.define(token.lexeme, value);
    }
}
