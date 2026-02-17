use std::fmt::{Display, Formatter};
use crate::core::Token;
use crate::utils::{FileWrapper, MorphError, MorphResult};

#[derive(Debug)]
pub struct ReadError {
    pub message: String,
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadError: {}", self.message)
    }
}

impl MorphError for ReadError {
    fn message(&self) -> String {
        format!("ReadError: {}", self.message)
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexerError at line {} column {}: {}", self.line, self.column, self.message)
    }
}

impl MorphError for LexerError {
    fn message(&self) -> String {
        format!("LexerError at line {} column {}: {}", self.line, self.column, self.message)
    }
}

pub trait Lexer {
    fn tokenize(&self, wrapper: &mut FileWrapper) -> MorphResult<Vec<Token>>;

}

