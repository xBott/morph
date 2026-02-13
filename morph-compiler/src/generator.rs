use crate::parser::Packet;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum Lang {
    Java
}

impl FromStr for Lang {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "java" => Ok(Lang::Java),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported language: {}", s)
            ))
        }
    }
}

pub trait Generator {
    fn generate(&self, output_dir: &Path, packets: &Vec<Packet>);

}

pub fn create_generator(lang: &Lang, package: String) -> Box<dyn Generator> {
    match lang {
        Lang::Java => Box::new(crate::generators::JavaGenerator::new(package)),
    }
}