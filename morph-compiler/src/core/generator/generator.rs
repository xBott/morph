use crate::parser::Packet;
use std::path::Path;
pub(crate) use crate::cli::Lang;

pub trait GenerationConfig {}

pub trait Generator {
    fn generate(&self, config: GenerationConfig, packets: &Vec<Packet>);

}