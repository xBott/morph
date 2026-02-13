use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "morph")]
#[command(about = "Morph compiler")]
pub struct Cli {
    #[arg(value_enum)]
    pub lang: Lang,

    #[arg(short = 'i', long)]
    pub input_dir: PathBuf,
    #[arg(short = 'o', long, default_value = "/generated")]
    pub output_dir: PathBuf,

    #[arg(short = 'p', long, default_value = "generated")]
    pub package: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Lang {
    Java
}
