


pub mod graph {
       use std::collections::HashMap;
    pub mod graph_items {
        pub mod node {
             use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
           

            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }
              
                pub fn attr(mut self, key: &str, value: &str) -> Self {
                    self.attrs.insert(key.to_string(), value.to_string());
                    self
                }

            }
        }

        pub mod edge {
               use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str()) 
                }                
            }
        }
    }
    pub struct Graph{
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }   
        }
        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }
        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self { 
            self.edges.extend_from_slice(edges);
            self
        }    
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }
       // Function to find data from node
        pub fn node (mut self, input: &str ) -> Option<&str> {   
            // find node matching input
            let node = self.nodes.iter().find ( |n| n.name == input)  ;
            // if node is found, return the second value of the first attribute, otherwise return None
            if 
            
            }
           
        }
       

    }
}

/*
use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;

*/
