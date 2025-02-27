use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub body: Vec<StatementType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    Invalid,
    Decleration(DeclerationType),
    Expr(ExprType),
    Block(BlockStatement),
    Return(ExprType),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TypeSpec {
    Invalid,
    Int,
    String,
    // Alias(TypeSpec),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<StatementType>,
    id: usize,
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
    Identifier(Token),
    StringLiteral(Token),
    NumericLiteral(Token),
    Binary(Box<ExprType>, Token, Box<ExprType>),
    MemberAccess(Box<ExprType>, Box<ExprType>),
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
    pub fn name(&self) -> Token {
        self.name.clone()
    }
    pub fn return_type(&self) -> TypeSpec {
        self.return_type
    }
    pub fn body(&self) -> StatementType {
        self.body.clone()
    }
}

static mut BLOCKS: usize = 0;

fn gen_new_id() -> usize {
    unsafe {
        BLOCKS+=1;
        BLOCKS
    }
}

impl BlockStatement {
    pub fn new(body: Vec<StatementType>) -> Self {
        Self { body, id: gen_new_id() }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}