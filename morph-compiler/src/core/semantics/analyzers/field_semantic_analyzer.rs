use crate::core::semantics::semantic_analyzer::SemanticAnalyzer;
use crate::core::{Packet, SemanticError};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{MorphError, MorphResult};

pub struct FieldSemanticAnalyzer;

impl SemanticAnalyzer for FieldSemanticAnalyzer {

    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();
        let mut existing_names: Vec<String> = Vec::new();

        for packet in packets {

            for field in &packet.fields {

                let field_name = field.name.to_string();

                if existing_names.contains(&field_name) {
                    let err = SemanticError {
                        message: format!("Duplicate field name in packet '{}': {}", packet.name, field_name),
                    };
                    all_errors.push(Box::new(err));
                }

                existing_names.push(field_name);

            }

        }

        if all_errors.is_empty() {
            Success(())

        } else {
            Errors(all_errors)
            
        }

    }

}