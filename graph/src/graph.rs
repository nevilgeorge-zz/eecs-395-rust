// graph.rs

pub struct Graph {
    nodes: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    key:       String,
    value:     String,
    neighbors: Vec<NodePtr>,
}

type NodePtr = Option<Box<Node>>;

// struct CursorMut<'a, K: 'a, V: 'a>(Option<&'a mut Node<K, V>>);

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

impl Node {
    pub fn new(k: String, v: String) -> Self {
        Node {
            key: k,
            value: v,
            neighbors: vec![]
        }
    }

    pub fn add_neighbor(&mut self, neighbor: NodePtr) {
        self.neighbors.push(neighbor);
    }

}
