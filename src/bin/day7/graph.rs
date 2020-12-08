use std::collections::{HashMap, HashSet};
use nom::lib::std::collections::VecDeque;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<String, HashSet<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str, quantity: i32) {
        let edge = Edge::new(to, quantity);
        match self.edges.get_mut(from) {
            Some(tos) => {
                tos.insert(edge);
            },
            None => {
                let mut nodes = HashSet::new();
                nodes.insert(edge);
                self.edges.insert(from.to_owned(), nodes);
            }
        }
    }

    pub fn nodes_reachable_from(&self, n: &str) -> Option<i32> {
        let mut nodes_to_check: VecDeque<&str> = self.edges.get(n)?.iter().map(|e| &e.value[..]).collect();
        let mut reachable_nodes: HashSet<&str> = nodes_to_check.iter().cloned().collect();
        while !nodes_to_check.is_empty() {
            let candidate = nodes_to_check.pop_front().unwrap();
            if let Some(edges) = self.edges.get(candidate) {
                for edge in edges {
                    if !reachable_nodes.contains(&edge.value[..]) {
                        reachable_nodes.insert(&edge.value);
                        nodes_to_check.push_back(&edge.value);
                    }
                }
            }
        }

        Some(reachable_nodes.len() as i32)
    }

    pub fn inverted(&self) -> Graph {
        let mut new_graph = Graph::new();

        for (from, tos) in self.edges.iter() {
            for to in tos {
                new_graph.add_edge(&to.value, from, to.quantity);
            }
        }

        new_graph
    }

    pub fn count_nodes(&self, start: &str) -> i32 {
        match self.edges.get(start) {
            Some(edges) => {
                edges.iter().map(|e| e.quantity * (1 + self.count_nodes(&e.value))).sum()
            },
            None => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Edge {
    quantity: i32,
    value: String,
}

impl Edge {
    fn new(value: &str, quantity: i32) -> Self {
        Edge { value: value.to_owned(), quantity }
    }
}