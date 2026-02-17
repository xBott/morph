use std::fmt::{Display, Formatter};
use crate::core::{FieldType, Token};
use crate::utils::{MorphError, MorphResult};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub token: Option<Token>,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.token {
            Some(token) => {
                write!(f, "ParserError at line {} column {}: {}", token.line, token.column, self.message)
            }
            None => {
                write!(f, "ParserError: {}", self.message)
            },
        }
    }
}

impl MorphError for ParserError {
    fn message(&self) -> String {
        match &self.token {
            Some(token) => {
                format!("ParserError at line {} column {}: {}", token.line, token.column, self.message)
            }
            None => {
                format!("ParserError: {}", self.message)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i32,
    pub is_auto: bool,
    pub name: String,
    pub fields: Vec<Field>
}

#[derive(Debug, Clone)]
pub struct Field {
    pub typ: FieldType,
    pub name: String
}

pub trait AstParser {
    fn parse(&self, tokens: &Vec<Token>) -> MorphResult<Vec<Packet>>;
    
}