use crate::{
    ast::{Expr, Stmt},
    errors::{Error, LoxResult},
    scanner::{
        token::{LiteralType, Token},
        tokenType::TokenType,
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> LoxResult<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> LoxResult<Stmt> {
        if self.match_types(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> LoxResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer = None;

        if self.match_types(vec![TokenType::Equal]) {
            initializer = Some(Box::new(self.expression()?));
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Ok(Stmt::Var(name.clone(), initializer))
    }

    fn statement(&mut self) -> LoxResult<Stmt> {
        if self.match_types(vec![TokenType::Print]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> LoxResult<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ; after value.");
        Ok(Stmt::Print(Box::new(value)))
    }

    fn expression_statement(&mut self) -> LoxResult<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ; after expression.");
        Ok(Stmt::Expression(Box::new(value)))
    }

    fn expression(&mut self) -> LoxResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> LoxResult<Expr> {
        let expr = self.equality()?;

        if self.match_types(vec![TokenType::Equal]) {
            let value = self.assignment()?;

            if let Expr::Variable(token) = expr {
                return Ok(Expr::Assign(token, Box::new(value)));
            }

            return Err(Error::ParseError(
                self.peek().line,
                "Invalid assignment target".to_string(),
            ));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> LoxResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> LoxResult<Expr> {
        let mut expr = self.term()?;

        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> LoxResult<Expr> {
        let mut expr = self.factor()?;

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> LoxResult<Expr> {
        let mut expr = self.unary()?;
        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> LoxResult<Expr> {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> LoxResult<Expr> {
        if self.match_types(vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralType::LBoolean(false)));
        }
        if self.match_types(vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralType::LBoolean(true)));
        }
        if self.match_types(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralType::LNil));
        }
        if self.match_types(vec![TokenType::Number, TokenType::StringLiteral]) {
            return Ok(Expr::Literal(self.previous().clone().literal.unwrap()));
        }
        if self.match_types(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_types(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(Error::ParseError(self.peek().line, "Expect expression".to_string()))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> LoxResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }

        Err(Error::ParseError(self.peek().line, message.to_string()))
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
