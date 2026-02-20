mod cli;
mod core;
mod utils;

use std::fs;
use crate::cli::{Cli, Commands, Lang};
use crate::core::*;
use std::path::{Path, PathBuf};
use clap::Parser;
use walkdir::WalkDir;
use crate::utils::{print_morph_errors, FileWrapper, MorphResult};
use crate::utils::MorphResult::{Errors, Success};

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { input_dir } => {

            let init_dir = input_dir.unwrap_or_else(|| std::env::current_dir().unwrap());

            if let Err(err) = init(&init_dir) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }

        }
        Commands::Build { lang, input_dir } => {

            let build_dir = input_dir.unwrap_or_else(|| std::env::current_dir().unwrap());

            if let Err(err) = build(&lang, &build_dir) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }


        }
    }

}

fn init(dir: &PathBuf) -> Result<(), std::io::Error> {
    println!("Initializing project in {:?}", dir);

    if !dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory does not exist",
        ));
    }

    let config_path = dir.join("morph.toml");
    let example_path = dir.join("main.morph");

    if !config_path.exists() {
        let output_dir = dir.join("generated").join("java");

        let default_config = format!(
            r#"[java]
output_dir              = "{}"
package                 = "packets"
no_args_constructor     = true
generate_to_string      = true
generate_equals         = true
generate_hashcode       = true
"#,
            output_dir.display()
        );

        fs::write(&config_path, default_config)?;
        println!("Created {:?}", config_path);
    } else {
        println!("morph.toml already exists, skipping");
    }

    if !example_path.exists() {
        fs::write(&example_path, "")?;
        println!("Created {:?}", example_path);
    } else {
        println!("main.morph already exists, skipping");
    }

    println!("Project initialized successfully.");
    Ok(())
}

fn build(lang: &Lang, dir: &PathBuf) -> Result<(), std::io::Error> {


    if !dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory does not exist",
        ));
    }

    let config_path: PathBuf = dir.join("morph.toml");

    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("'morph.toml' does not exist in '{}'", dir.to_string_lossy()),
        ));
    }

    println!("Building morph files to '{}'", lang);

    let lexer = SimpleLexer;
    let parser = SimpleParser;
    let mut semantic_analyzer = CompositeSemanticAnalyzer::new();
    semantic_analyzer.add_analyzer(Box::new(NameSemanticAnalyzer));
    semantic_analyzer.add_analyzer(Box::new(IdSemanticAnalyzer));
    semantic_analyzer.add_analyzer(Box::new(DependencySemanticAnalyzer));
    semantic_analyzer.add_analyzer(Box::new(FieldSemanticAnalyzer));

    let morph_file_paths = find_morph_files(dir);
    println!("Found {}x morph files", morph_file_paths.len());

    for path in morph_file_paths {

        let path_string = path.to_string_lossy();

        println!("- Building '{}'", path_string);
        match build_file(
            &lang,
            &config_path,
            &path,
            &lexer,
            &parser,
            &semantic_analyzer,
        ) {
            Success(()) => println!("- Successfully built '{}'", path_string),
            Errors(errors) => {
                println!("- Some errors occurred while building '{}'", path_string);
                print_morph_errors(&errors, "\t\t")
            }
        }

    }

    Ok(())

}

fn find_morph_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "morph")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn build_file(
    lang: &Lang,
    config_path: &Path,
    path: &PathBuf,
    lexer: &SimpleLexer,
    parser: &SimpleParser,
    analyzer: &CompositeSemanticAnalyzer
) -> MorphResult<()> {

    let mut wrapper = FileWrapper {
        path: path.clone(),
    };

    let tokens: Vec<Token>;

    match lexer.tokenize(&mut wrapper) {
        Success(values) => tokens = values,
        Errors(errors) => return Errors(errors),
    }

    let packets: Vec<Packet>;

    match parser.parse(&tokens) {
        Success(values) => packets = values,
        Errors(errors) => return Errors(errors),
    }

    // println!("\nParsed {} packets:", packets.len());
    // for packet in &packets {
    //     if packet.is_auto {
    //         println!("  - {} (id: auto)", packet.name);
    //     } else {
    //         println!("  - {} (id: {})", packet.name, packet.id);
    //     }
    //     for field in &packet.fields {
    //         println!("    {}: {}", field.name, field.typ);
    //     }
    // }

    match analyzer.analyze(&packets) {
        Errors(errors) => return Errors(errors),
        _ => {}
    }

    match generate(lang, &config_path, &packets) {
        Success(()) => {
            Success(())
        },
        Errors(errors) => Errors(errors),
    }

}


fn generate(lang: &Lang, config_path: &Path, packets: &Vec<Packet>) -> MorphResult<()> {

    match GenerationConfig::from_file(config_path) {

        Ok(config) => {

            let generator: Box<dyn Generator> = match lang {
                Lang::Java => {
                    let options: JavaOptions = match config.parse_lang_options("java") {
                        Ok(opts) => opts,
                        Err(err) => {
                            let morph_error = GenerationError {
                                message: format!("Failed to parse options: {}", err),
                            };
                            return Errors(vec![Box::new(morph_error)])
                        },
                    };
                    Box::new(JavaGenerator { options })
                }
            };

            generator.generate(packets)

        }

        Err(errors) => {
            let err = GenerationError {
                message: errors.to_string(),
            };
            Errors(vec![Box::new(err)])
        }

    }



}
