// graph_tests.rs

#[cfg(test)]
mod graph_new_tests {
    use graph::Graph;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn create_empty_graph() {
        let map = HashMap::<String, Vec<String>>::new();
        let new_graph = Graph::new(map);
        let empty_map = HashMap::<String, HashSet<String>>::new();

        assert_eq!(new_graph.map, empty_map);
    }

    #[test]
    fn create_full_graph() {
        let mut map = HashMap::<String, Vec<String>>::new();
        map.insert('a'.to_string(), vec!['b'.to_string(), 'c'.to_string()]);
        map.insert('b'.to_string(), vec!['a'.to_string()]);
        map.insert('c'.to_string(), vec!['b'.to_string()]);

        let a_pointer = &('a'.to_string());
        let b_pointer = &('b'.to_string());
        let c_pointer = &('c'.to_string());

        let new_graph = Graph::new(map);

        let a_set = new_graph.map.get(a_pointer).unwrap();
        let b_set = new_graph.map.get(b_pointer).unwrap();
        let c_set = new_graph.map.get(c_pointer).unwrap();

        assert!(a_set.contains(b_pointer));
        assert!(a_set.contains(c_pointer));

        assert!(b_set.contains(a_pointer));
        assert!(b_set.contains(c_pointer));

        assert!(c_set.contains(a_pointer));
        assert!(c_set.contains(b_pointer));
    }
}

#[cfg(test)]
mod graph_add_node_tests {
    use graph::Graph;
    use std::collections::HashMap;

    #[test]
    fn add_node() {
        let map = HashMap::<String, Vec<String>>::new();
        let mut new_graph = Graph::new(map);
        let a = 'a'.to_string();
        new_graph.add_node(a.clone());

        assert!(new_graph.map.contains_key(&a));
        assert_eq!(new_graph.map.get(&a).unwrap().len(), (0 as usize));
    }
}

#[cfg(test)]
mod graph_add_edge_tests {
    use graph::Graph;
    use std::collections::{HashMap, HashSet};

    fn initialize_graph() -> Graph {
        let map = HashMap::<String, Vec<String>>::new();
        let mut new_graph = Graph::new(map);
        let a = 'a'.to_string();
        let b = 'b'.to_string();

        new_graph.add_node(a.clone());
        new_graph.add_node(b.clone());

        let empty_set = HashSet::<String>::new();

        assert_eq!(new_graph.map.get(&a), Some(&empty_set));
        assert_eq!(new_graph.map.get(&b), Some(&empty_set));

        new_graph
    }

    #[test]
    fn add_edge() {
        let mut graph = initialize_graph();
        let a = 'a'.to_string();
        let b = 'b'.to_string();

        graph.add_edge(a.clone(), b.clone());

        let a_set = graph.map.get(&a).unwrap();
        let b_set = graph.map.get(&b).unwrap();

        assert!(a_set.contains(&b));
        assert!(b_set.contains(&a));
    }

    #[test]
    fn add_edge_missing_node() {
        let mut graph = initialize_graph();
        let c = 'c'.to_string();
        let d = 'd'.to_string();

        graph.add_edge(c.clone(), d.clone());

        let c_entry = graph.map.get(&c);
        let d_entry = graph.map.get(&d);

        assert_eq!(c_entry, None);
        assert_eq!(d_entry, None);
    }
}

#[cfg(test)]
mod graph_find_path_tests {
    use graph::Graph;
    use std::collections::{HashMap, HashSet};

    fn initialize_graph() -> Graph {
        let map = HashMap::<String, Vec<String>>::new();
        let mut new_graph = Graph::new(map);
        let a = 'a'.to_string();
        let b = 'b'.to_string();
        let c = 'c'.to_string();
        let d = 'd'.to_string();
        let e = 'e'.to_string();

        new_graph.add_node(a.clone());
        new_graph.add_node(b.clone());
        new_graph.add_node(c.clone());
        new_graph.add_node(d.clone());
        new_graph.add_node(e.clone());


        let mut empty_set = HashSet::<String>::new();

        assert_eq!(new_graph.map.get(&a), Some(&empty_set));
        assert_eq!(new_graph.map.get(&b), Some(&empty_set));
        assert_eq!(new_graph.map.get(&c), Some(&empty_set));
        assert_eq!(new_graph.map.get(&d), Some(&empty_set));
        assert_eq!(new_graph.map.get(&e), Some(&empty_set));

        new_graph.add_edge(a.clone(), b.clone());
        new_graph.add_edge(b.clone(), c.clone());
        new_graph.add_edge(c.clone(), d.clone());

        empty_set.insert(b.clone());

        assert_eq!(new_graph.map.get(&a), Some(&empty_set));

        new_graph
    }

    #[test]
    fn find_path_missing_node() {
        let mut graph = initialize_graph();
        let path = graph.find_path("e".to_owned(), "f".to_owned());
        assert_eq!(path, "A given node does not exist in the graph!\n".to_owned());
    }

    #[test]
    fn find_path_missing_edge() {
        let mut graph = initialize_graph();
        let path = graph.find_path("a".to_owned(), "e".to_owned());
        assert_eq!(path, "Path does not exist.\n".to_owned( ));
    }

    #[test]
    fn find_existing_path() {
        let mut graph = initialize_graph();

        let mut path = graph.find_path("a".to_owned(), "d".to_owned());
        assert_eq!(path, "a b c d\n".to_owned( ));

        path = graph.find_path("b".to_owned(), "c".to_owned());
        assert_eq!(path, "b c\n".to_owned( ));
    }
}
