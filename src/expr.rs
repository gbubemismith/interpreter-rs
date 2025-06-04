use crate::token::{LiteralTypes, Token};

#[derive(Debug)]
pub enum Expr {
    Assignment(Assignment),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Unary(Unary),
    Variable(Variable),
}

#[derive(Debug)]
pub struct Assignment {
    name: Token,
    value: Box<Expr>,
}

#[derive(Debug)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}

#[derive(Debug)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}

#[derive(Debug)]
pub struct Grouping {
    expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Literal {
    value: LiteralTypes,
}

#[derive(Debug)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct Variable {
    name: Token,
}
