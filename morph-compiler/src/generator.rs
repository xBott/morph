use crate::parser::Packet;
use std::path::Path;
pub(crate) use crate::cli::Lang;

pub trait Generator {
    fn generate(&self, output_dir: &Path, packets: &Vec<Packet>);

}

pub fn create_generator(lang: &Lang, package: String) -> Box<dyn Generator> {
    match lang {
        Lang::Java => Box::new(crate::generators::JavaGenerator::new(package)),
    }
}