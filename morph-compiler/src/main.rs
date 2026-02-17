mod cli;
mod core;
mod utils;

use crate::cli::Cli;
use crate::core::*;
use std::fs::File;
use clap::Parser;
use crate::utils::{print_morph_errors, FileWrapper};
use crate::utils::MorphResult::{Errors, Success};

fn main() -> std::io::Result<()> {

    let cli = Cli::parse();

    println!("Compiling {} to {:?}", cli.input_dir.to_string_lossy(), cli.lang);

    let mut wrapper = FileWrapper {
        path: cli.input_dir.clone(),
        file: File::open(&cli.input_dir)?,
    };

    let lexer = SimpleLexer;
    let tokens: Vec<Token>;

    match lexer.tokenize(&mut wrapper) {
        Success(values) => tokens = values,
        Errors(errors) => {
            print_morph_errors(&errors);
            panic!("Some errors occurred while tokenizing stage");
        }
    }

    let parser = SimpleParser;
    let packets: Vec<Packet>;

    match parser.parse(&tokens) {
        Success(values) => packets = values,
        Errors(errors) => {
            print_morph_errors(&errors);
            panic!("Some errors occurred while parsing stage");
        }
    }

    let mut semantic_analyzer = CompositeSemanticAnalyzer::new();
    semantic_analyzer.add_analyzer(Box::new(NameSemanticAnalyzer));
    semantic_analyzer.add_analyzer(Box::new(DependencySemanticAnalyzer));

    match semantic_analyzer.analyze(&packets) {
        Errors(errors) => {
            print_morph_errors(&errors);
            panic!("Some errors occurred while analyzing stage");
        }
        _ => {}
    }

    println!("\nGenerating code...");
    // let generator = create_generator(&cli.lang, cli.package);
    // generator.generate(&cli.output_dir, &packets);

    Ok(())
}
