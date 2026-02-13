use crate::token::{as_token_kind, Token};
use std::io::{BufRead, BufReader};
use crate::utils::FileWrapper;

pub trait Lexer {
    fn tokenize(&self, wrapper: &mut FileWrapper) -> std::io::Result<Vec<Token>>;

}

pub struct SimpleLexer;

impl SimpleLexer {

    fn tokenize_line(&self, line_number: usize, line: &str) -> std::io::Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut char_indices = line.char_indices().peekable();
        let mut current_word = String::new();
        let mut word_start_column = 0;

        while let Some((idx, ch)) = char_indices.next() {
            if ch.is_whitespace() {
                if !current_word.is_empty() {
                    if let Some(kind) = as_token_kind(&current_word.as_str()) {
                        tokens.push(Token {
                            kind,
                            line: line_number,
                            column: word_start_column,
                        });
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Unknown token '{}' at line: {} column: {}",
                                    current_word, line_number, word_start_column)
                        ));
                    }
                    current_word.clear();
                }
            } else {
                if current_word.is_empty() {
                    word_start_column = idx;
                }
                current_word.push(ch);
            }
        }

        if !current_word.is_empty() {

            match as_token_kind(&current_word.as_str()) {
                Some(kind) => {
                    tokens.push(Token {
                        kind,
                        line: line_number,
                        column: word_start_column,
                    });
                }
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown token '{}' at line: {} column: {}",
                                current_word, line_number, word_start_column)
                    ));
                }
            }

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
                    match self.tokenize_line(line_number, &line.as_str()) {
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