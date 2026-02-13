mod utils;
mod lexer;
mod token;
mod parser;
mod generator;
mod generators;
mod cli;

use crate::cli::Cli;
use crate::generator::create_generator;
use crate::lexer::{Lexer, SimpleLexer};
use crate::parser::{AstParser, SimpleParser};
use crate::utils::FileWrapper;
use std::fs::File;
use clap::Parser;

fn main() -> std::io::Result<()> {

    let cli = Cli::parse();

    println!("Compiling {} to {:?}", cli.input_dir.to_string_lossy(), cli.lang);

    let mut wrapper = FileWrapper {
        path: cli.input_dir.clone(),
        file: File::open(&cli.input_dir)?,
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
    let generator = create_generator(&cli.lang, cli.package);
    generator.generate(&cli.output_dir, &packets);

    Ok(())
}
