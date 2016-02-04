// graph.rs

pub struct Graph<K, V> {
    nodes: Vec<Node<K, V>>,
}

#[derive(Clone)]
struct Node<K, V> {
    key:       K,
    value:     V,
    neighbors: Vec<NodePtr<K, V>>,
}

type NodePtr<K, V> = Option<Box<Node<K, V>>>;

// struct CursorMut<'a, K: 'a, V: 'a>(Option<&'a mut Node<K, V>>);

impl<K, V> Graph<K, V> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node<K, V>) {
        self.nodes.push(node);
    }
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            value: v,
            neighbors: vec![]
        }
    }

    pub fn add_neighbor<'a>(&mut self, ptr: NodePtr<K, V>) {
        if let &Some(ref n) = ptr {

        }
        else {
            
        }
    }



}
