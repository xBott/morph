use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::utils::{MorphError, MorphResult};
use crate::utils::MorphResult::{Errors, Success};

pub trait Dependent {
    fn dependent_id(&self) -> &str;
}

pub struct DependencyGraph<T: Dependent> {
    pub nodes: HashMap<String, T>,
    pub edges: HashMap<String, Vec<String>>,
}

pub struct DFSLevel {
    pub start_id: String,
    pub successors: Vec<String>
}

impl DFSLevel {

    pub fn new( start_id: String, successors: Vec<String>) -> Self {
        DFSLevel { start_id, successors }
    }

    pub fn pop_successor(&mut self) -> Option<String> {
        self.successors.pop()
    }

    pub fn is_ready(&self) -> bool {
        self.successors.is_empty()
    }

}

#[derive(Debug)]
pub struct DependencyCycle {
    pub nodes: Vec<String>,
}

impl DependencyCycle {
    pub fn new(nodes: Vec<String>) -> Self {
        DependencyCycle { nodes }
    }
}

impl Display for DependencyCycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dependency cycle: {:?}", self.nodes)
    }
}

#[derive(Debug)]
pub struct DependencyResolvingError {
    pub message: String,
}

impl Display for DependencyResolvingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DependencyResolvingError: {}", self.message)
    }
}

impl MorphError for DependencyResolvingError {
    fn message(&self) -> String {
        format!("DependencyResolvingError: {}", self.message)
    }
}

impl<T: Dependent> DependencyGraph<T> {

    pub fn new(nodes: HashMap<String, T>, edges: HashMap<String, Vec<String>>) -> Self {
        DependencyGraph { nodes, edges }
    }
    
    pub fn has_node(&self, node_id: &str) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn node_by_id(&self, node_id: &str) -> Option<&T> {
        self.nodes.get(node_id)
    }

    pub fn successor_ids(&self, node_id: &str) -> Vec<String> {
        self.edges
            .get(node_id)
            .map(|ids| ids.clone())
            .unwrap_or_default()
    }

    pub fn successors(&self, node_id: &str) -> Vec<&T> {
        self.successor_ids(node_id)
            .into_iter()
            .filter_map(|successor_id| {
                self.node_by_id(successor_id.as_str())
            })
            .collect()
    }

    pub fn dfs<S, FEnter, FExit>(
        &self,
        start_node_id: &str,
        state: &mut S,
        mut on_enter: FEnter,
        mut on_exit: FExit,
    )
    where
        FEnter: FnMut(&T, &mut S, usize) -> bool,
        FExit: FnMut(&T, &mut S, usize) -> bool,
    {

        let mut stack: Vec<DFSLevel> = Vec::new();

        let root_level = DFSLevel::new(
            start_node_id.to_string(),
            self.successor_ids(start_node_id)
        );

        if let Some(node) = self.node_by_id(start_node_id) {
            on_enter(node, state, stack.len());
        }

        stack.push(root_level);

        while let Some(level) = stack.last_mut() {

            if level.is_ready() {

                let node_id = level.start_id.clone();

                if let Some(node) = self.node_by_id(&node_id) {
                    let cont = on_exit(node, state, stack.len());
                    if !cont {
                        return;
                    }
                }

                stack.pop();

            } else {
                if let Some(next_id) = level.pop_successor() {

                    let successors = self.successor_ids(&next_id);

                    if let Some(node) = self.node_by_id(&next_id) {
                        let cont = on_enter(node, state, stack.len());
                        if !cont {
                            return;
                        }
                    }

                    stack.push(DFSLevel::new(
                        next_id,
                        successors,
                    ));
                }

            }
        }

    }

    pub fn find_cycle(&self) -> Option<DependencyCycle> {

        for node in self.nodes.values() {
            let node_id = node.dependent_id();
            if let Some(cycle) = self.find_cycle_from_node(node_id) {
                return Some(cycle);
            }
        }

        None
    }

    fn find_cycle_from_node(&self, start_node_id: &str) -> Option<DependencyCycle> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        fn dfs<T: Dependent>(
            graph: &DependencyGraph<T>,
            node_id: &str,
            visited: &mut HashSet<String>,
            path: &mut Vec<String>,
        ) -> Option<DependencyCycle> {

            if let Some(pos) = path.iter().position(|id| id == node_id) {
                let mut cycle_nodes = path[pos..].to_vec();
                cycle_nodes.push(node_id.to_string());
                return Some(DependencyCycle::new(cycle_nodes));
            }

            if visited.contains(node_id) {
                return None;
            }

            path.push(node_id.to_string());

            if let Some(children) = graph.edges.get(node_id) {
                for child_id in children {
                    if let Some(cycle) = dfs(graph, child_id, visited, path) {
                        return Some(cycle);
                    }
                }
            }

            path.pop();
            visited.insert(node_id.to_string());
            None

        }

        dfs(self, start_node_id, &mut visited, &mut path)
    }

    pub fn topological_sort(&self) -> MorphResult<Vec<&T>> {

        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // 1️⃣ инициализация
        for node in self.nodes.keys() {
            in_degree.insert(node.clone(), 0);
        }

        // 2️⃣ считаем входящие рёбра
        for successors in self.edges.values() {
            for succ in successors {
                *in_degree.get_mut(succ).unwrap() += 1;
            }
        }

        let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();

        for (node, degree) in &in_degree {
            if *degree == 0 {
                heap.push(Reverse(node.clone()));
            }
        }


        let mut result: Vec<&T> = Vec::new();

        while let Some(Reverse(node_id)) = heap.pop() {

            let node = self.node_by_id(&node_id).unwrap();
            result.push(node);

            for succ in self.successor_ids(&node_id) {
                let entry = in_degree.get_mut(&succ).unwrap();
                *entry -= 1;

                if *entry == 0 {
                    heap.push(Reverse(succ));
                }
            }
        }

        if result.len() != in_degree.len() {

            let mut errors: Vec<Box<dyn MorphError>> = Vec::new();

            in_degree.iter()
                .filter(|(_, node_degree)| {
                    **node_degree == 0
                })
                .for_each(|(node_id, _)| {
                    let err = DependencyResolvingError {
                        message: format!("node '{}' does not have all necessary dependencies", node_id)
                    };
                    errors.push(Box::new(err));
                });

            return Errors(errors);
        }

        Success(result)

    }

}

pub struct DependentGraphBuilder<T: Dependent> {
    nodes: HashMap<String, T>,
    edges: HashMap<String, Vec<String>>,
}

impl<T: Dependent> DependentGraphBuilder<T> {

    pub fn new() -> DependentGraphBuilder<T> {
        DependentGraphBuilder {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn node(&mut self, node: T) {
        let node_id = node.dependent_id();
        self.nodes.entry(node_id.to_string()).or_insert(node);
    }

    pub fn edge(&mut self, from: String, to: String) {
        self.edges.entry(from).or_default().push(to);
    }

    pub fn build(self) -> DependencyGraph<T> {
        DependencyGraph::new(self.nodes, self.edges)
    }

}
