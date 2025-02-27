use crate::{driver::diag::DiagPrinter, sema::analyzer::Sema, syntax::{ast::Ast, lexer::Lexer, parser::Parser}};

use super::diag::DiagType;

pub struct Context{
    diagnostic_printer: DiagPrinter,
    file_data: String,
}

impl Context{
    pub fn new(diagnostic_printer: DiagPrinter, file_data: String) -> Self {
        Self { diagnostic_printer, file_data, }
    }
    pub fn print_info(&self, file_data: bool){
        self.diagnostic_printer.print_info();
        if file_data {
            self.diagnostic_printer.print_formatted(DiagType::Debug, format!("File data: `{}`", self.file_data));
        }
    }
    pub fn run(&self){
        let lexer: Lexer = Lexer::new(self.file_data.clone(), self.diagnostic_printer.clone());
        let mut parser: Parser = Parser::new(lexer, self.diagnostic_printer.clone());
        let ast: Ast = parser.parse_to_ast();
        let sema: Sema = Sema::new(ast, self.diagnostic_printer.clone());
        sema.run();
    }
}