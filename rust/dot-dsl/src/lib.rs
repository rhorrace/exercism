use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: Vec<(String, String)>,
}

impl Node{

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: Vec::new(),
        }
    }

    pub fn with_attrs(self, attributes: &[(&str, &str)]) -> Self {
        Self {
            attrs: attributes.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ..self
        }
    }

    pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
        match self.attrs.iter()
            .find(|(k, _)| k == &attr_key) {
            Some(a) => Some(&a.1[..]),
            _ => None,
        }
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub vertex1: String,
    pub vertex2: String,
    pub attrs: Vec<(String, String)>,
}

impl Edge {

    pub fn new(vertex1: &str, vertex2: &str) -> Self {
        Self {
            vertex1: vertex1.to_string(),
            vertex2: vertex2.to_string(),
            attrs: Vec::new(),
        }
    }

    pub fn with_attrs(self, attributes: &[(&str, &str)]) -> Self {
        Self {
            attrs: attributes.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ..self
        }
    }
}

pub struct Graph <>{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph
{
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
        Self {
            nodes: nodes.to_vec(),
            ..self
        }
    }

    pub fn get_node(&self, node_name: &str) -> Option<&Node> {
        self.nodes.iter()
            .find(|node| node.name.eq(&node_name.to_string()))
    }

    pub fn with_edges(self, edges: & Vec<Edge>) -> Self {
        Self {
            edges: edges.to_vec(),
            ..self
        }
    }

    pub fn with_attrs(self, attributes: &[(& str, & str)]) -> Self {
        Self {
            attrs: attributes.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ..self
        }
    }
}


pub mod graph {

    pub use super::Graph;

    pub mod graph_items {

        pub mod node {
            pub use super::super::super::Node;
        }

        pub mod edge {
            pub use super::super::super::Edge;
        }
    }
}