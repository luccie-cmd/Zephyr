use crate::{driver::diag::{DiagPrinter, DiagType}, syntax::ast::{Ast, DeclerationType, ExprType, FunctionDeclerationStatement, StatementType, TypeSpec}};
use std::collections::HashMap;

pub struct SemaChecker {
    ast: Ast,
    diag: DiagPrinter,
    pub scopes: HashMap<String, SymbolTable>,
    pub current_scope: String,
    pub scope_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable(TypeSpec),
    Function(Vec<TypeSpec>, TypeSpec), // Arguments, Return type
    Namespace, // Represents a namespace, but is NOT a type
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolKind>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
    pub fn get_symbol_by_name(&self, name: String) -> SymbolKind {
        if !self.contains(name.clone()) {
            unreachable!();
        }
        self.symbols.get(&name).unwrap().clone()
    }
    pub fn contains(&self, name: String) -> bool {
        self.symbols.contains_key(&name)
    }
    pub fn append(&mut self, name: String, kind: SymbolKind) {
        self.symbols.insert(name, kind);
    }
}

impl SemaChecker {
    pub fn new(ast: Ast, diag: DiagPrinter) -> Self {
        let table: SymbolTable = SymbolTable::new();
        let mut hash: HashMap<String, SymbolTable> = HashMap::new();
        hash.insert("__top_scope__".to_string(), table);
        Self { ast, diag, scopes: hash, current_scope: "__top_scope__".to_string(), scope_names: vec![]}
    }
    fn add_table(&mut self, name: String, table: SymbolTable) {
        self.scopes.insert(name, table);
    }
    fn append_top(&mut self, name: String, kind: SymbolKind) {
        let table: String = "__top_scope__".to_string();
        if let Some(sym_table) = self.scopes.get_mut(&table) { // Get a mutable reference
            sym_table.append(name, kind);
        } else {
            eprintln!("Error: '__top_scope__' does not exist in scopes!");
        }
    }
    fn contains_name(&self, table: String, name: String) -> bool {
        self.scopes.get(&table).unwrap().contains(name)
    }
    fn collect_func_decl(&mut self, func: FunctionDeclerationStatement) {
        let name: String = func.name().get_data();
        if self.contains_name("__top_scope__".to_string(), name.clone()) {
            self.diag.print_formatted(DiagType::Error, format!("Redefinition of function `{}`", name));
        }
        self.append_top(name.clone(), SymbolKind::Function(vec![], func.return_type()));
        let func_scope: SymbolTable = SymbolTable::new();
        // TODO: Function arguments
        self.add_table(name.clone(), func_scope);
        self.collect_symbols(&func.body());
    }
    fn collect_symbols(&mut self, stmt: &StatementType) {
        match stmt {
            StatementType::Decleration(DeclerationType::Function(func)) => {
                self.collect_func_decl(*func.clone());
            }
            StatementType::Block(block) => {
                let block_scope_name = format!("block_{}", block.get_id());
                let block_scope: SymbolTable = SymbolTable::new();
                self.add_table(block_scope_name.clone(), block_scope);
                self.current_scope = block_scope_name.clone();
                for stmt in &block.body {
                    self.collect_symbols(stmt);
                }
                self.current_scope = "__top_scope__".to_string();
            }
            _ => {}
        }
    }
    fn first_pass(&mut self) {
        for stmt in self.ast.body.clone() {
            self.collect_symbols(&stmt);
        }
    }
    fn evaluate_expr_to_type(&self, expr: ExprType) -> TypeSpec {
        match expr {
            ExprType::NumericLiteral(_) => {
                return TypeSpec::Int;
            }
            ExprType::StringLiteral(_) => {
                return  TypeSpec::String;
            }
            ExprType::Binary(left, op, right) => {
                let left_type: TypeSpec = self.evaluate_expr_to_type(*left);
                let right_type: TypeSpec = self.evaluate_expr_to_type(*right);
                if left_type != right_type {
                    self.diag.print_formatted(DiagType::Error, format!("Invalid operands of types `{:?}` and `{:?}` to binary operator (`{}`)", left_type, right_type, op.get_data()));
                }
                return left_type;
            }
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Handle getting type for expression `{:?}`", expr));
            }
        }
        unreachable!();
    }
    fn get_return_types_in_block(&self, body: StatementType) -> Vec<TypeSpec> {
        let mut stmts: Vec<StatementType> = vec![];
        match body {
            StatementType::Block(block) => {
                stmts = block.body;
            }
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Unhandled function body statement `{:?}`", body));
            }
        };
        let mut return_types: Vec<TypeSpec> = vec![];
        for stmt in stmts {
            if let StatementType::Return(expr) = stmt {
                let expr_type: TypeSpec = self.evaluate_expr_to_type(expr);
                return_types.push(expr_type);
            };
        }
        return_types
    }
    fn validate_func(&mut self, func: FunctionDeclerationStatement) {
        let return_type_expected: TypeSpec = func.return_type();
        let return_type_actual: Vec<TypeSpec> = self.get_return_types_in_block(func.body());
        for actual_type in return_type_actual {
            if return_type_expected != actual_type {
                self.diag.print_formatted(DiagType::Error, format!("Unexpected return type, expected `{:?}` but got `{:?}`", return_type_expected, actual_type));
            }
        }
        self.validate_stmt(func.body());
    }
    fn validate_expr(&mut self, expr: ExprType) {
        match expr {
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Handle validating expression `{:?}`", expr));
            }
        }
    }
    fn validate_stmt(&mut self, stmt: StatementType) {
        match stmt {
            StatementType::Decleration(DeclerationType::Function(func)) => {
                self.validate_func(*func.clone());
            }
            StatementType::Block(block) => {
                for stmt in block.body {
                    self.validate_stmt(stmt);
                }
            }
            StatementType::Expr(expr) => {
                self.validate_expr(expr);
            }
            _ => {
                self.diag.print_formatted(DiagType::Ice, format!("Handle validating for statement `{:?}`", stmt));
            }
        }
    }
    fn second_pass(&mut self) {
        for stmt in self.ast.body.clone() {
            self.validate_stmt(stmt);
        }
    }
    pub fn check(&mut self) {
        self.first_pass();
        println!("Symbols: {:#?}", self.scopes);
        self.second_pass();
    }
}