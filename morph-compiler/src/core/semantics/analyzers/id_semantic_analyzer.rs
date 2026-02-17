use crate::core::semantics::semantic_analyzer::SemanticAnalyzer;
use crate::core::{Packet, SemanticError};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{MorphError, MorphResult};

pub struct IdSemanticAnalyzer;

impl SemanticAnalyzer for IdSemanticAnalyzer {

    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();
        let mut existing_ids: Vec<i32> = Vec::new();

        for packet in packets {
            
            if packet.is_auto {
                continue
            }
            
            let packet_name = packet.name.to_string();
            let packet_id = packet.id;
            
            if packet_id < 0 {
                let err = SemanticError {
                    message: format!("Id of packet '{}' can not be less than 0, actual value is {}", packet_name, packet_id),
                };
                all_errors.push(Box::new(err));
            }
            
            if existing_ids.contains(&packet_id) {
                let err = SemanticError {
                    message: format!("Duplicate packet id: {}", packet_id),
                };
                all_errors.push(Box::new(err));
            }
            
            existing_ids.push(packet_id);
        }

        if all_errors.is_empty() {
            Success(())

        } else {
            Errors(all_errors)
            
        }

    }

}