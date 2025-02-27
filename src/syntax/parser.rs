use crate::driver::diag::{DiagPrinter, DiagType};
use super::{ast::{Ast, StatementType}, lexer::Lexer, token::{Token, TokenType}};
use std::process::exit;

pub struct Parser {
    lexer: Lexer,
    pub diag: DiagPrinter,
    pub current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer, diag: DiagPrinter) -> Self {
        let current_token: Token = lexer.next_token();
        Self { lexer, diag, current_token }
    }
    pub fn consume(&mut self) -> Token {
        let current: Token = std::mem::replace(&mut self.current_token, self.lexer.next_token());
        current
    }
    pub fn expect(&mut self, consume: bool, token_type: TokenType) -> Option<Token> {
        if self.current_token.get_type() == token_type {
            let mut ret_token: Token = self.current_token.clone();
            if consume {
                ret_token = self.consume();
            }
            return Some(ret_token);
        }
        self.diag.print_formatted(DiagType::Error, format!("Expected `{}` but got `{}` instead", token_type as u64, self.current_token.get_data()));
        exit(1);
    }
    pub fn parse_to_ast(&mut self) -> Ast {
        let mut body: Vec<StatementType> = vec![];
        while self.current_token.get_type() != TokenType::Eof {
            body.push(self.parse_top_stmt());
        }
        Ast::new(body)
    }
}