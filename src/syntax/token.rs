use crate::driver::diag::{DiagPrinter, DiagType};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u64)]
pub enum TokenType {
    Eof = 0,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Comma,
    Colon,
    Semicolon,
    Plus,
    PlusPlus,
    PlusEqual,
    Minus,
    MinusMinus,
    MinusEqual,
    ColonColon,
    Identifier,
    NumericLiteral,
    StringLiteral,
    __KEYWORDSSTART = 255,
    Func,
    Return,
    As,
    __TYPESSTART = 511,
    Int,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Default for Token {
    fn default() -> Self {
        Self { token_type: TokenType::Eof, value: "".to_string() }
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self { token_type, value }
    }
    pub fn print(&self, formatter: &DiagPrinter) {
        formatter.print_formatted(DiagType::Debug, format!("{} (`{}`)", self.token_type as u64, self.value));
    }
    pub fn get_type(&self) -> TokenType {
        self.token_type
    }
    pub fn get_data(&self) -> String {
        if self.token_type == TokenType::StringLiteral {
            return '"'.to_string() + &self.value + "\"";
        }
        self.value.clone()
    }
}