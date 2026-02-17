use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use crate::core::Lexer;
use crate::core::Token;
use crate::core::as_token_kind;
use crate::core::lexer::lexer::{LexerError, ReadError};
use crate::utils::{FileWrapper, MorphError, MorphResult};
use crate::utils::MorphResult::{Errors, Success};

pub struct SimpleLexer;

impl SimpleLexer {

    fn is_special_char(ch: char) -> bool {
        matches!(ch, '{' | '}' | '=' | ',' | ':' | ';' | '(' | ')' | '[' | ']')
    }

    fn tokenize_line(&self, line_number: usize, line: &str) -> MorphResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<Box<dyn MorphError>> = Vec::new();

        let mut chars = line.chars().enumerate().peekable();
        let mut current_word = String::new();
        let mut word_start_column = 0;

        while let Some((idx, ch)) = chars.next() {

            if ch.is_whitespace() {

                if !current_word.is_empty() {
                    self.process_word(current_word.as_str(), line_number, word_start_column, &mut tokens, &mut errors);
                    current_word.clear();
                }

                continue;
            }

            if Self::is_special_char(ch) {

                if !current_word.is_empty() {
                    self.process_word(current_word.as_str(), line_number, word_start_column, &mut tokens, &mut errors);
                    current_word.clear();
                }

                let special_token = ch.to_string();
                self.process_word(special_token.as_str(), line_number, idx, &mut tokens, &mut errors);
                continue;

            }

            if current_word.is_empty() {
                word_start_column = idx;
            }

            current_word.push(ch);
        }

        if !current_word.is_empty() {
            self.process_word(current_word.as_str(), line_number, word_start_column, &mut tokens, &mut errors);
        }

        if errors.is_empty() {
            Success(tokens)
        } else {
            Errors(errors)
        }
    }

    fn process_word(
        &self,
        word: &str,
        line: usize,
        column: usize,
        tokens: &mut Vec<Token>,
        errors: &mut Vec<Box<dyn MorphError>>,
    ) {
        match as_token_kind(word) {
            Some(kind) => {
                tokens.push(Token {
                    kind,
                    line,
                    column,
                });
            }
            None => {
                let err = LexerError {
                    message: format!("Unknown token '{}'", word),
                    line,
                    column,
                };
                errors.push(Box::new(err));
            }
        }
    }

}

impl Lexer for SimpleLexer {

    fn tokenize(&self, wrapper: &mut FileWrapper) -> MorphResult<Vec<Token>> {

        let mut tokens: Vec<Token> = Vec::new();
        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();

        let reader = BufReader::new(&wrapper.file);

        for (line_number, result) in reader.lines().enumerate() {
            match result {
                Ok(line) => {
                    match self.tokenize_line(line_number + 1, line.as_str()) {
                        Success(line_tokens) => { tokens.extend(line_tokens) }
                        Errors(errors) => { all_errors.extend(errors) }
                    }
                }
                Err(err) => {
                    let read_error = ReadError {
                        message: format!("Failed to read line {}, cause: {}", line_number, err)
                    };
                    all_errors.push(Box::new(read_error));
                }
            }
        }

        if all_errors.is_empty() {
            Success(tokens)
        } else {
            Errors(all_errors)
        }

    }

}