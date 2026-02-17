use crate::core::semantics::semantic_analyzer::SemanticAnalyzer;
use crate::core::FieldType::{Array, Nested};
use crate::core::{DependencyGraph, DependencyResolvingError, Dependent, DependentGraphBuilder, Packet};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{MorphError, MorphResult};
use std::collections::HashSet;

#[derive(Debug)]
#[derive(Clone)]
pub struct DependentField {
    pub name: String,
}

impl DependentField {
    pub fn new(value: String) -> Self {
        DependentField { name: value }
    }
}

impl Dependent for DependentField {
    fn dependent_id(&self) -> &str {
        self.name.as_str()
    }
}

pub struct DependencySemanticAnalyzer;

impl DependencySemanticAnalyzer {

    fn analyze_dependencies(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let graph = match self.build_dependency_graph(packets) {
            Success(graph) => graph,
            Errors(errors) => return Errors(errors)
        };

        match graph.find_cycle() {
            Some(cycle) => {
                let err = DependencyResolvingError {
                    message: format!("Cycle detected: {}", cycle),
                };
                return Errors(vec![Box::new(err)]);
            }
            None => {}
        }

        //self.print_graph_dependencies(&graph);

        Success(())

    }

    fn build_dependency_graph(&self, packets: &Vec<Packet>) -> MorphResult<DependencyGraph<DependentField>> {
        let mut builder = DependentGraphBuilder::<DependentField>::new();

        let existing_ids = self.collect_existing_ids(packets);

        for packet in packets {
            self.add_packet_node(&mut builder, packet);
        }

        for packet in packets {
            match self.add_packet_dependencies(&mut builder, packet, &existing_ids) {
                Success(_) => {}
                Errors(errors) => return Errors(errors),
            }
        }

        Success(builder.build())
    }

    fn collect_existing_ids(&self, packets: &Vec<Packet>) -> HashSet<String> {
        packets.iter()
            .map(|p| DependentField::new(p.name.to_string()).dependent_id().to_string())
            .collect()
    }

    fn add_packet_node(&self, builder: &mut DependentGraphBuilder<DependentField>, packet: &Packet) {
        let node = DependentField::new(packet.name.to_string());
        builder.node(node);
    }

    fn add_packet_dependencies(
        &self,
        builder: &mut DependentGraphBuilder<DependentField>,
        packet: &Packet,
        existing_ids: &HashSet<String>,
    ) -> MorphResult<()> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();

        let current_id = DependentField::new(packet.name.to_string()).dependent_id().to_string();

        for field in &packet.fields {
            match &field.typ {
                Nested(type_name) => {
                    match self.add_graph_dependency(builder, current_id.as_str(), type_name.as_str(), existing_ids) {
                        Success(_) => {}
                        Errors(errors) => all_errors.extend(errors),
                    }
                }
                Array(inner_type) => {
                    if let Nested(type_name) = inner_type.as_ref() {
                        match self.add_graph_dependency(builder, current_id.as_str(), type_name.as_str(), existing_ids) {
                            Success(_) => {}
                            Errors(errors) => all_errors.extend(errors),
                        }
                    }
                }
                _ => {}
            }
        }

        if all_errors.is_empty() {
            Success(())

        } else {
            Errors(all_errors)

        }

    }

    fn add_graph_dependency(
        &self,
        builder: &mut DependentGraphBuilder<DependentField>,
        current_id: &str,
        type_name: &str,
        existing_ids: &HashSet<String>,
    ) -> MorphResult<()> {
        let dependency_id = DependentField::new(type_name.to_string()).dependent_id().to_string();

        if existing_ids.contains(&dependency_id) {
            let dep_node = DependentField::new(type_name.to_string());
            builder.node(dep_node);
            builder.edge(current_id.to_string(), dependency_id);
            Success(())

        } else {
            let err = DependencyResolvingError {
                message: format!("Dependency '{}' does not exist", type_name)
            };
            Errors(vec![Box::new(err)])

        }
    }

    fn print_graph_dependencies(&self, graph: &DependencyGraph<DependentField>) {

        println!("Dependencies are valid! Tree:");

        let mut visited: Vec<String> = Vec::new();
        graph.nodes.iter().for_each(|(id, _)| {
            visited.push(id.clone());
            let mut state = ();
            graph.dfs(
                id,
                &mut state,
                |node: &DependentField, _, depth| {
                    visited.push(node.dependent_id().to_string());
                    let indent = "   ".repeat(depth);
                    println!("{} -> {}", indent, node.dependent_id());
                    true
                },
                |_, _, _| { true }
            )
        });
        println!("\n");

    }


}

impl SemanticAnalyzer for DependencySemanticAnalyzer {

    fn analyze(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();

        match self.analyze_dependencies(packets) {
            Errors(errors) => all_errors.extend(errors),
            _ => {}
        }

        if all_errors.is_empty() {
            Success(())
        } else {
            Errors(all_errors)
        }
    }

}