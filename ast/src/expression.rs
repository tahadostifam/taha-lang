use core::fmt;
use token::{Span, Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(UnaryExpression),
    Infix(BinaryExpression),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct FunctionCall { 
    pub call: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(Integer),
    Boolean(Boolean),
    String(StringType),
    Array(Array),
    Hash(Hash),
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub operand: Box<Expression>,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub operator: Token, 
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub raw: i64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub raw: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StringType {
    pub raw: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Hash {
    pub pairs: Vec<(Expression, Expression)>,
    pub span: Span,
}


pub fn format_expressions(exprs: &Vec<Expression>) -> String {
    exprs.iter().map(|expr| expr.to_string()).collect()
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
