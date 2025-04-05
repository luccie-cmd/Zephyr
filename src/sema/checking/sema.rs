use crate::{driver::diag::{DiagPrinter, DiagType}, syntax::ast::{Ast, DeclerationType, ExprType, FunctionDeclerationStatement, StatementType, TypeSpec}};
use std::collections::HashMap;
use std::process::exit;

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
    parent: Option<String>
}

impl SymbolTable {
    pub fn new(parent: Option<String>) -> Self {
        Self { symbols: HashMap::new(), parent }
    }
    pub fn get_symbol_by_name(&self, semacheck: &SemaChecker, name: &String) -> Option<SymbolKind> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol.clone())
        } else if let Some(ref parent) = self.parent {
            // Recursively look in the parent scope
            if let Some(parent_scope) = semacheck.get_scope(parent) {
                parent_scope.get_symbol_by_name(semacheck, name)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn contains(&self, semacheck: &SemaChecker, name: &String) -> bool {
        self.symbols.contains_key(name) || 
        self.parent.as_ref().and_then(|parent| {
            if let Some(parent_scope) = semacheck.get_scope(parent) {
                Some(parent_scope.contains(semacheck, name))
            } else {
                None
            }
        }).unwrap_or(false)
    }
    pub fn append(&mut self, name: String, kind: SymbolKind) {
        self.symbols.insert(name, kind);
    }
}

impl SemaChecker {
    pub fn new(ast: Ast, diag: DiagPrinter) -> Self {
        let table: SymbolTable = SymbolTable::new(None);
        let mut hash: HashMap<String, SymbolTable> = HashMap::new();
        hash.insert("__top_scope__".to_string(), table);
        Self { ast, diag, scopes: hash, current_scope: "__top_scope__".to_string(), scope_names: vec![]}
    }
    pub fn get_scope(&self, name: &str) -> Option<SymbolTable> {
        self.scopes.get(name).cloned()
    }
    fn get_symbol(&self, name: String) -> SymbolKind {
        if let Some(sym_table) = self.scopes.get(&name) {
            sym_table.get_symbol_by_name(self, &name).expect("Expected symbol to be in table")
        } else {
            self.diag.print_formatted(DiagType::Ice, "Improper checking of scopes".to_string());
            exit(1);
        }
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
        if let Some(sym_table) = self.scopes.get(&table) {
            sym_table.contains(self, &name)
        } else {
            false
        }
    }
    fn collect_func_decl(&mut self, func: FunctionDeclerationStatement) {
        let name: String = func.name().get_data();
        if self.contains_name("__top_scope__".to_string(), name.clone()) {
            self.diag.print_formatted(DiagType::Error, format!("Redefinition of function `{}`", name));
        }
        self.append_top(name.clone(), SymbolKind::Function(vec![], func.return_type()));
        let func_scope: SymbolTable = SymbolTable::new(Some(self.current_scope.clone()));
        // TODO: Function arguments
        self.scope_names.push(self.current_scope.clone());
        self.current_scope = name.clone();
        self.add_table(name.clone(), func_scope);
        self.collect_symbols(&func.body());
        self.current_scope = self.scope_names.pop().expect("Expected atleast 1 scope because we pushed atleast 1");
    }
    fn collect_symbols(&mut self, stmt: &StatementType) {
        match stmt {
            StatementType::Decleration(DeclerationType::Function(func)) => {
                self.collect_func_decl(*func.clone());
            }
            StatementType::Block(block) => {
                let block_scope_name = format!("__block_{}__", block.get_id());
                let block_scope: SymbolTable = SymbolTable::new(Some(self.current_scope.clone()));
                self.add_table(block_scope_name.clone(), block_scope);
                self.scope_names.push(self.current_scope.clone());
                self.current_scope = block_scope_name.clone();
                for stmt in &block.body {
                    self.collect_symbols(stmt);
                }
                self.current_scope = self.scope_names.pop().expect("Expected atleast 1 scope because we pushed atleast 1");
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
            ExprType::Identifier(identifier) => {
                if !self.contains_name(self.current_scope.clone(), identifier.get_data()) {
                    self.diag.print_formatted(DiagType::Error, format!("Use of undeclared identifier `{}`", identifier.get_data()));
                }
            }
            ExprType::Call(callee, args) => {
                // Box<ExprType>, Vec<ExprType>
                self.validate_expr(*callee);
                for arg in args {
                    self.validate_expr(arg);
                }
            }
            ExprType::MemberAccess(member, property) => {
                self.validate_expr(*member.clone());
                let ExprType::Identifier(ref member_ident) = *member else { panic!("Improper parsing of memberExpr\n"); };
                let ExprType::Identifier(ref property_ident) = *property else { panic!("Improper parsing of memberExpr\n"); };
                let symbol: SymbolKind = self.get_symbol(member_ident.get_data());
                let SymbolKind::Namespace = symbol else { self.diag.print_formatted(DiagType::Error, format!("Symbol `{}` isn't a structure or a namespace", member_ident.get_data())); exit(1); };
                if !self.contains_name(member_ident.get_data(), property_ident.get_data()) {
                    self.diag.print_formatted(DiagType::Error, format!("No property named `{}` found in scope `{}`", property_ident.get_data(), member_ident.get_data()));
                }
            }
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
        println!("Sema First pass");
        self.first_pass();
        println!("Symbols: {:#?}", self.scopes);
        println!("Sema Second pass");
        self.second_pass();
    }
}