mod file;
mod lexer;
mod token;
mod parser;

use std::env;
use std::fs::File;
use std::path::{PathBuf};
use crate::file::FileWrapper;
use crate::lexer::{Lexer, SimpleLexer};
use crate::token::Token;

fn main() {

    let path: PathBuf = env::args().nth(1).expect("Usage: ...").into();
    println!("Compiling {}", path.to_string_lossy());

    match File::open(&path) {
        Ok(file) => {
            let mut wrapper = FileWrapper { path, file };
            let lexer = SimpleLexer;
            match lexer.tokenize(&mut wrapper) {
                Ok(tokens) => {
                    print_tokens(&tokens);
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

}

fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{:?}", token);
    }
}
