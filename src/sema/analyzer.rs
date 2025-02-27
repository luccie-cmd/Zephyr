use crate::{driver::diag::DiagPrinter, syntax::ast::Ast};

use super::checking::sema::SemaChecker;

pub struct Sema {
    ast: Ast,
    diag: DiagPrinter,
}

impl Sema {
    pub fn new(ast: Ast, diag: DiagPrinter) -> Self {
        Self { ast, diag }
    }
    pub fn run(&self) -> Ast {
        let mut sema_checker: SemaChecker = SemaChecker::new(self.ast.clone(), self.diag.clone());
        sema_checker.check();
        self.ast.clone()
    }
}