use crate::driver::diag::DiagType;
use super::{ast::{StatementType, BlockStatement}, parser::Parser, token::{TokenType}};
use std::process::exit;

impl Parser {
    pub fn parse_block_stmt(&mut self) -> BlockStatement {
        let mut body: Vec<StatementType> = vec![];
        while self.current_token.get_type() != TokenType::CloseCurly {
            body.push(self.parse_stmt());
        }
        self.expect(true, TokenType::CloseCurly);
        BlockStatement::new(body)
    }
    pub fn parse_stmt(&mut self) -> StatementType {
        match self.current_token.get_type() {
            TokenType::OpenCurly => {
                self.consume();
                StatementType::Block(self.parse_block_stmt())
            }
            TokenType::Return => {
                self.consume();
                let stmt: StatementType = StatementType::Return(self.parse_expr(0));
                self.expect(true, TokenType::Semicolon);
                stmt
            }
            _ => {
                let stmt: StatementType = StatementType::Expr(self.parse_expr(0));
                self.expect(true, TokenType::Semicolon);
                stmt
            }
        }
    }
    pub fn parse_top_stmt(&mut self) -> StatementType {
        match self.current_token.get_type() {
            TokenType::Func => {
                StatementType::Decleration(self.parse_decleration())
            }
            _ => {
                self.diag.print_formatted(DiagType::Error, format!("Unexpected token: `{}`", self.current_token.get_data()));
                exit(1);
            }
        }
    }
}