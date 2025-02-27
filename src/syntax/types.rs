use crate::driver::diag::DiagType;
use super::{ast::{TypeSpec}, parser::Parser, token::{TokenType}};
use std::process::exit;

impl Parser {
    const TOKENTYPE_AS_TYPESPEC: &[(TokenType, TypeSpec)] = &[
        (TokenType::Int, TypeSpec::Int),
        (TokenType::String, TypeSpec::String),
    ];
    pub fn parse_type_annotation(&mut self) -> TypeSpec {
        if self.current_token.get_type() > TokenType::__TYPESSTART {
            for (token_type, type_spec) in Self::TOKENTYPE_AS_TYPESPEC {
                if *token_type == self.current_token.get_type() {
                    self.consume();
                    return *type_spec;
                }
            }
        }
        self.diag.print_formatted(DiagType::Error, format!("Expected type specifier, but got `{}` instead", self.current_token.get_data()));
        exit(1);
    }
}