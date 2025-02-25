use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub body: Vec<StatementType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    Invalid,
    Decleration(Box<DeclerationType>),
    Expr(Box<ExprType>),
    Block(Box<BlockStatement>),
    Return(Box<ExprType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpec {
    Invalid,
    Int,
    Alias,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<StatementType>,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u64)]
pub enum DeclerationType {
    Invalid,
    Function(Box<FunctionDeclerationStatement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclerationStatement {
    name: Token,
    return_type: TypeSpec,
    body: StatementType,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u64)]
pub enum ExprType {
    Invalid,
    Identifier(Box<Token>),
    StringLiteral(Box<Token>),
    NumericLiteral(Box<Token>),
    Binary(Box<ExprType>, Token, Box<ExprType>),
    MemberAccess(Box<ExprType>, Token),
    Call(Box<ExprType>, Vec<ExprType>),
    Cast(Box<ExprType>, TypeSpec),
}

impl Ast {
    pub fn new(body: Vec<StatementType>) -> Self {
        Self { body }
    }
    pub fn print(&self) {
        for stmt in self.body.clone().into_iter() {
            println!("{:#?}", stmt);
        }
    }
}

impl FunctionDeclerationStatement {
    pub fn new(name: Token, return_type: TypeSpec, body: StatementType) -> Self {
        Self { name, return_type, body }
    }
}

impl BlockStatement {
    pub fn new(body: Vec<StatementType>) -> Self {
        Self { body }
    }
}