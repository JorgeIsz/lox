use std::fmt::{self, write, Display};

fn error(line: usize, message: &str) {
    report(line, message, "");
}

fn report(line: usize, message: &str, location: &str) {
    eprintln!("[line {}], Error {}: {}", line, location, message);
}

pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub enum LiteralType {
    LString(String),
    LNumber(f64),
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralType>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let mut token_type = TokenType::Bang;
                if self.token_match('=') {
                    token_type = TokenType::BangEqual
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let mut token_type = TokenType::Equal;
                if self.token_match('=') {
                    token_type = TokenType::EqualEqual
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let mut token_type = TokenType::Less;
                if self.token_match('=') {
                    token_type = TokenType::LessEqual
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let mut token_type = TokenType::Greater;
                if self.token_match('=') {
                    token_type = TokenType::GreaterEqual
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.token_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            '"' => self.string(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if self.is_digit(c) {
                    self.number();
                }
                else {
                    error(self.line, "Unexpected character");
                }
            },
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_digit(&self, c:char) -> bool {
        "0123456789".contains(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source.get(self.start..self.current).unwrap().parse::<f64>().unwrap();
        let literal = Some(LiteralType::LNumber(value));
        self.add_token(TokenType::Number, literal);

    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        } else {
            // The closing ".
            self.advance();

            // Trim the surrounding quotes.
            let value = self.source.get(self.start + 1..self.current - 1).unwrap();
            let literal = Some(LiteralType::LString(value.to_string()));
            self.add_token(TokenType::StringLiteral, literal);
        }
    }

    fn token_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let current_value = self.current.clone();
        self.current += 1;
        return self.source.chars().nth(current_value).unwrap();
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralType>) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}
