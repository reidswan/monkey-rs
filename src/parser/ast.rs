use super::ParseError;
use crate::lexer::tokens;

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement {
        token: tokens::Token,
        identifier: Identifier,
        value: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Dummy,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: tokens::Token,
    pub value: String,
}
