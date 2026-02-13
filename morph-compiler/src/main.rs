mod utils;
mod lexer;
mod token;
mod parser;
mod generator;
mod generators;
use crate::generator::{create_generator, Generator, Lang};
use crate::lexer::{Lexer, SimpleLexer};
use crate::parser::{Parser, SimpleParser};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use crate::utils::FileWrapper;

fn main() -> std::io::Result<()> {

    let mut args = env::args().skip(1);

    let lang: Lang = args
        .next()
        .expect("Usage: morph <lang> <file> [output_dir]")
        .parse()?;

    let input_dir: PathBuf = args
        .next()
        .expect("Usage: morph <lang> [input_dir] [output_dir]")
        .into();

    let output_dir: PathBuf = args
        .next()
        .expect("Usage: morph <lang> [input_dir] [output_dir]")
        .into();

    println!("Compiling {} to {:?}", input_dir.to_string_lossy(), lang);

    let mut wrapper = FileWrapper {
        path: input_dir.clone(),
        file: File::open(&input_dir)?,
    };

    let lexer = SimpleLexer;
    let tokens = lexer.tokenize(&mut wrapper)?;

    let parser = SimpleParser;
    let packets = parser.parse(&tokens)?;

    println!("\nParsed {} packets:", packets.len());
    for packet in &packets {
        println!("  - {} (id: {})", packet.name, packet.id);
    }

    println!("\nGenerating code...");
    let generator = create_generator(&lang, "me.bottdev.morph".to_string());
    generator.generate(&output_dir, &packets);

    Ok(())
}
