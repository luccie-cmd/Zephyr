use crate::driver::diag::DiagType;
use super::{ast::{ExprType, TypeSpec}, parser::Parser, token::{Token, TokenType}};
use std::process::exit;

fn get_precedency(token: TokenType) -> u8 {
    match token {
        TokenType::Plus => 1,
        _ => 0,
    }
}

impl Parser {
    pub fn parse_primary_expr(&mut self) -> ExprType {
        match self.current_token.get_type() {
            TokenType::Identifier => {
                let ret: ExprType = ExprType::Identifier(self.current_token.clone());
                self.consume();
                ret
            }
            TokenType::StringLiteral => {
                let ret: ExprType = ExprType::StringLiteral(self.current_token.clone());
                self.consume();
                ret
            }
            TokenType::NumericLiteral => {
                let ret: ExprType = ExprType::NumericLiteral(self.current_token.clone());
                self.consume();
                ret
            }
            TokenType::OpenParen => {
                self.consume();
                let expr: ExprType = self.parse_expr(0);
                self.expect(true, TokenType::CloseParen);
                expr
            }
            _ => {
                self.diag.print_formatted(DiagType::Error, format!("Unexpected token: `{}`", self.current_token.get_data()));
                exit(1);
            }
        }
    }
    pub fn parse_postfix_expr(&mut self) -> ExprType {
        let mut member: ExprType = self.parse_primary_expr();
        loop {
            match self.current_token.get_type() {
                TokenType::ColonColon => {
                    self.consume();
                    let property = self.parse_primary_expr();
                    member = ExprType::MemberAccess(Box::new(member), Box::new(property));
                }
                TokenType::OpenParen => {
                    self.consume();
                    let mut args: Vec<ExprType> = vec![];
                    if self.current_token.get_type() != TokenType::CloseParen {
                        loop {
                            args.push(self.parse_expr(0));
                            if self.current_token.get_type() != TokenType::Comma {
                                break;
                            }
                            self.consume();
                        }
                    }
                    if self.consume().get_type() != TokenType::CloseParen {
                        self.diag.print_formatted(DiagType::Error, format!("Expected `)` but got `{}` instead", self.current_token.get_data()));
                        exit(1);
                    }
                    member = ExprType::Call(Box::new(member), args);
                }
                _ => {
                    break;
                }
            }
        }
        member
    }
    pub fn parse_expr(&mut self, min_prec: u8) -> ExprType {
        let mut lhs: ExprType = self.parse_postfix_expr();
        while let TokenType::Plus | TokenType::Minus = self.current_token.get_type() {
            match self.current_token.get_type() {
                TokenType::Plus | TokenType::Minus => {
                    let operation: Token = self.current_token.clone();
                    let precedency: u8 = get_precedency(self.current_token.get_type());
                    if precedency < min_prec {
                        break;
                    }
                    self.consume();
                    let rhs: ExprType = self.parse_expr(precedency + 1);
                    lhs = ExprType::Binary(Box::new(lhs), operation, Box::new(rhs));
                }
                _ => { break }
            }
        }
        if self.current_token.get_type() == TokenType::As {
            self.consume();
            let type_spec: TypeSpec = self.parse_type_annotation();
            lhs = ExprType::Cast(Box::new(lhs), type_spec);
        }
        lhs
    }
}