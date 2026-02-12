use crate::file::FileWrapper;
use crate::token::{as_token_kind, Token};
use std::io::{BufRead, BufReader};

pub trait Lexer {
    fn tokenize(&self, wrapper: &mut FileWrapper) -> std::io::Result<Vec<Token>>;

}

pub struct SimpleLexer;

impl SimpleLexer {

    fn tokenize_line(&self, line_number: usize, line: &String) -> std::io::Result<Vec<Token>> {

        let mut tokens: Vec<Token> = Vec::new();

        let mut column = 0;
        for word in line.split_whitespace() {

            if let Some(kind) = as_token_kind(word) {
                tokens.push(Token {
                    kind,
                    line: line_number,
                    column,
                });
            }

            column += word.len() + 1;
        }
        Ok(tokens)
    }

}

impl Lexer for SimpleLexer {

    fn tokenize(&self, wrapper: &mut FileWrapper) -> std::io::Result<Vec<Token>> {

        let mut tokens: Vec<Token> = Vec::new();
        let reader = BufReader::new(&wrapper.file);

        for (line_number, result) in reader.lines().enumerate() {
            match result {
                Ok(line) => {
                    match self.tokenize_line(line_number, &line) {
                        Ok(line_tokens) => { tokens.extend(line_tokens) }
                        Err(err) => return Err(err)
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)

    }

}