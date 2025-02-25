use crate::driver::diag::DiagType;
use super::{ast::{DeclerationType, FunctionDeclerationStatement, StatementType, TypeSpec}, parser::Parser, token::{Token, TokenType}};
use std::process::exit;

impl Parser {
    pub fn parse_func_decleration(&mut self) -> FunctionDeclerationStatement  {
        let name: Token = self.expect(true, TokenType::Identifier).unwrap();
        self.expect(true, TokenType::OpenParen);
        self.expect(true, TokenType::CloseParen);
        self.expect(true, TokenType::Colon);
        let return_type: TypeSpec = self.parse_type_annotation();
        let body: StatementType = self.parse_stmt();
        FunctionDeclerationStatement::new(name, return_type, body)
    }
    pub fn parse_decleration(&mut self) -> DeclerationType {
        match self.current_token.get_type() {
            TokenType::Func => {
                self.consume();
                DeclerationType::Function(Box::new(self.parse_func_decleration()))
            }
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Unhandeld decleration token: `{}`", self.current_token.get_data()));
                exit(1);
            }
        }
    }
}