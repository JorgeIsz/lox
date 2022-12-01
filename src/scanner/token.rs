use std::{
    fmt::{self, Display},
    ops::{Add, Deref, Div, Mul, Sub},
};

use crate::scanner::tokenType::TokenType;

#[derive(Clone)]
pub enum LiteralType {
    LString(String),
    LNumber(f64),
    LBoolean(bool),
    LNil,
}

impl LiteralType {
    pub fn stringify(&self) -> String {
        match self {
            Self::LNil => String::from("nil"),
            Self::LString(val) => val.to_owned(),
            Self::LBoolean(val) => val.to_string(),
            Self::LNumber(val) => {
                let text = val.to_string();
                if text.ends_with(".0") {
                    return text.get(..text.len() - 2).unwrap().to_owned();
                }
                text
            }
        }
    }
    pub fn greater(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left > right)
    }

    pub fn greater_equal(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left >= right)
    }

    pub fn less(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left < right)
    }

    pub fn less_equal(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left <= right)
    }
    pub fn equal(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left == right)
    }
    pub fn not_equal(self, other: LiteralType) -> Self {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LBoolean(left != right)
    }

    fn get_number_operands(self, other: LiteralType) -> (f64, f64) {
        let left = match self {
            LiteralType::LNumber(val) => val,
            _ => panic!("Could not perform operation (left is not number)"),
        };
        let right = match other {
            LiteralType::LNumber(val) => val,
            _ => panic!("Could not perform operation (right is not number)"),
        };
        (left, right)
    }
}

impl Sub for LiteralType {
    type Output = Self;

    fn sub(self, other: LiteralType) -> Self::Output {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LNumber(left - right)
    }
}

impl Div for LiteralType {
    type Output = Self;

    fn div(self, other: LiteralType) -> Self::Output {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LNumber(left / right)
    }
}

impl Mul for LiteralType {
    type Output = Self;

    fn mul(self, other: LiteralType) -> Self::Output {
        let (left, right) = self.get_number_operands(other);
        LiteralType::LNumber(left * right)
    }
}

impl Add for LiteralType {
    type Output = Self;

    fn add(self, other: LiteralType) -> Self::Output {
        match self {
            LiteralType::LNumber(left) => match other {
                LiteralType::LNumber(right) => LiteralType::LNumber(left + right),
                _ => panic!("cannot add a number and a different thing"),
            },
            LiteralType::LString(left) => match other {
                LiteralType::LString(right) => LiteralType::LString(left + &right),
                _ => panic!("cannot concat a string and a different thing"),
            },
            _ => panic!("dunno what you're trying to sum/concat but it won't work"),
        }
    }
}

#[derive(Clone)]
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
