use crate::utils::{MorphError, MorphResult};
use std::fmt::{Display, Formatter};
use crate::core::Packet;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SemanticError: {}", self.message)
    }
}

impl MorphError for SemanticError {
    fn message(&self) -> String {
        format!("SemanticError: {}", self.message)
    }
}

pub trait SemanticAnalyzer {
    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()>;
    
}