use std::fmt::Display;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "morph")]
#[command(about = "Morph CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short = 'i', long)]
        input_dir: Option<PathBuf>,

    },

    Build {
        #[arg(value_enum)]
        lang: Lang,

        #[arg(short = 'i', long)]
        input_dir: Option<PathBuf>,
    },
}


#[derive(ValueEnum, Clone, Debug)]
pub enum Lang {
    Java,
}

impl Display for Lang {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}
