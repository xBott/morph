use crate::core::semantics::semantic_analyzer::SemanticAnalyzer;
use crate::core::Packet;
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::MorphResult;

pub struct CompositeSemanticAnalyzer {
    analyzers: Vec<Box<dyn SemanticAnalyzer>>,
}

impl CompositeSemanticAnalyzer {

    pub fn new() -> CompositeSemanticAnalyzer {
        CompositeSemanticAnalyzer { analyzers: vec![] }
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn SemanticAnalyzer>) {
        self.analyzers.push(analyzer);
    }

}

impl SemanticAnalyzer for CompositeSemanticAnalyzer {

    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        for analyzer in &self.analyzers {
            if let Errors(errors) = analyzer.analyze(packets) {
                return Errors(errors);
            }
        }

        Success(())

    }

}