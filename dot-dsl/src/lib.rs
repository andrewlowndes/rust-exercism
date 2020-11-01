pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge<'a> {
                a: &'a str,
                b: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(a: &'a str, b: &'a str) -> Self {
                    Edge {
                        a,
                        b,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    attrs.iter().for_each(|pair| {
                        self.attrs.insert(pair.0, pair.1);
                    });
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node<'a> {
                name: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Node {
                        name,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    attrs.iter().for_each(|pair| {
                        self.attrs.insert(pair.0, pair.1);
                    });
                    self
                }

                pub fn get_name(&self) -> &str {
                    self.name
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }
            }
        }
    }

    type Node<'a> = graph_items::node::Node<'a>;
    type Edge<'a> = graph_items::edge::Edge<'a>;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Default for Graph<'a> {
        fn default() -> Self {
            Graph::new()
        }
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.get_name() == name)
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            attrs.iter().for_each(|pair| {
                self.attrs.insert(pair.0.to_owned(), pair.1.to_owned());
            });
            self
        }
    }
}
