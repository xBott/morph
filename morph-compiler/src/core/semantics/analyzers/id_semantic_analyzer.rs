use crate::core::semantics::semantic_analyzer::SemanticAnalyzer;
use crate::core::{Packet, SemanticError};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{MorphError, MorphResult};

pub struct NameSemanticAnalyzer;

impl SemanticAnalyzer for NameSemanticAnalyzer {

    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();
        let mut existing_names: Vec<String> = Vec::new();

        for packet in packets {
            let packet_name = packet.name.to_string();
            if existing_names.contains(&packet_name) {
                let err = SemanticError {
                    message: format!("Duplicate packet name: {}", packet_name),
                };
                all_errors.push(Box::new(err));
            }
            existing_names.push(packet_name);
        }

        if all_errors.is_empty() {
            Success(())

        } else {
            Errors(all_errors)
            
        }

    }

}