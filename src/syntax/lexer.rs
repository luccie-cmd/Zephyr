use crate::driver::diag::{DiagType, DiagPrinter};
use crate::syntax::token::Token;
use std::string::String;
use std::process::exit;

use super::token::TokenType;

pub struct Lexer {
    file_data: String,
    pub current_index: usize,
    pub current_char: char,
    first_run: bool,
    diag: DiagPrinter,
}

fn is_whitespace(c: char) -> bool {
    if c == ' ' || c == '\n' {
        return true;
    }
    false
}

fn is_identifier(c: char) -> bool {
    if c.is_ascii_lowercase() | c.is_ascii_uppercase() | c.is_ascii_digit() | (c == '_') {
        return true;
    }
    false
}

const KEYWRODS: &[(&str, TokenType)] = &[
    ("func", TokenType::Func),
    ("int", TokenType::Int),
    ("string", TokenType::String),
    ("return", TokenType::Return),
    ("as", TokenType::As),
];

fn get_keyword(value: String) -> Token {
    for (lexme, token_type) in KEYWRODS {
        if *lexme.to_string() == value {
            return Token::new(*token_type, value);
        }
    }
    Token::new(TokenType::Identifier, value)
}

impl Lexer {
    pub fn new(file_data: String, diag_printer: DiagPrinter) -> Self {
        Self { file_data, current_index: 0, current_char: '\0', first_run: true, diag: diag_printer }
    }
    fn next_char(&mut self){
        self.current_char = self.file_data.chars().nth(self.current_index).unwrap_or('\0');
        self.current_index += 1;
    }
    fn skip_whitespace(&mut self) {
        while is_whitespace(self.current_char) {
            self.next_char();
        }
    }
    fn parse_keywordidentifier(&mut self) -> Token {
        let mut value: String = String::default();
        while is_identifier(self.current_char) {
            value.push(self.current_char);
            self.next_char();
        }
        get_keyword(value)
    }
    fn parse_numeric(&mut self) -> Token {
        let mut value: String = String::default();
        while self.current_char.is_ascii_digit() {
            value.push(self.current_char);
            self.next_char();
        }
        Token::new(TokenType::NumericLiteral, value)
    }
    fn parse_string(&mut self) -> Token {
        let mut value: String = String::default();
        self.next_char();
        while self.current_char != '"' && self.current_index < self.file_data.len() {
            value.push(self.current_char);
            self.next_char();
        }
        if self.current_index >= self.file_data.len() {
            self.diag.print_formatted(DiagType::Error, "Unterminated string".to_string());
        }
        self.next_char();
        Token::new(TokenType::StringLiteral, value)
    }
    fn parse_singletoken(&mut self) -> Token {
        let token_type: TokenType;
        let mut value: String = String::default();
        match self.current_char {
            '(' => {
                value.push('(');
                self.next_char();
                token_type = TokenType::OpenParen;
            }
            ')' => {
                value.push(')');
                self.next_char();
                token_type = TokenType::CloseParen;
            }
            '{' => {
                value.push('{');
                self.next_char();
                token_type = TokenType::OpenCurly;
            }
            '}' => {
                value.push('}');
                self.next_char();
                token_type = TokenType::CloseCurly;
            }
            ';' => {
                value.push(';');
                self.next_char();
                token_type = TokenType::Semicolon;
            }
            ':' => {
                value.push(':');
                self.next_char();
                if self.current_char == ':' {
                    value.push(':');
                    self.next_char();
                    token_type = TokenType::ColonColon;
                } else {
                    token_type = TokenType::Colon;
                }
            }
            '+' => {
                value.push('+');
                self.next_char();
                if self.current_char == '=' {
                    value.push('=');
                    self.next_char();
                    token_type = TokenType::PlusEqual;
                } else if self.current_char == '+' {
                    value.push('+');
                    self.next_char();
                    token_type = TokenType::PlusPlus;
                } else {
                    token_type = TokenType::Plus;
                }
            }
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Unhandled token `{}`", self.current_char));
                exit(1);
            }
        }
        Token::new(token_type, value)
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.first_run {
            self.next_char();
            self.first_run = false;
        }
        if self.current_char == '\0' {
            return Token::new(TokenType::Eof, "\0".to_string());
        }
        match self.current_char {
            'a'..='z' | 'A'..='Z' | '_' => {
                self.parse_keywordidentifier()
            }
            '0'..='9' => {
                self.parse_numeric()
            }
            '"' => {
                self.parse_string()
            }
            ':' |
            ';' |
            '+' |
            '(' |
            ')' |
            '{' |
            '}' => {
                self.parse_singletoken()
            }
            _ => {
                self.diag.print_formatted(DiagType::Error, format!("Unknown token `{}`", self.current_char));
                exit(1);
            }
        }
    }
}