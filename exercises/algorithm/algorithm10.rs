/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;

        // Add node1 -> node2
        // Using `entry` is efficient for this. It gets the entry for node1,
        // or inserts an empty vector if it doesn't exist.
        self.adjacency_table_mutable()
            .entry(node1.to_string())
            .or_default()
            .push((node2.to_string(), weight));

        // Add node2 -> node1 (because it's undirected)
        // Also ensure node2 exists in the table, even if it has no other neighbors yet.
        self.adjacency_table_mutable()
            .entry(node2.to_string())
            .or_default()
            .push((node1.to_string(), weight));
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        // Use `entry` to check if the node exists and potentially insert an empty list.
        // `or_insert_with(Vec::new)` returns a mutable reference to the value (Vec).
        // If the entry was vacant (new node), `Vec::new()` is called.
        // `is_vacant_entry` isn't directly available, but `or_insert_with` returns a ref to the value.
        // We can check the original state before the insertion using `Occupied/Vacant` enums,
        // but a simpler way is to check if the entry was already present.
        // `entry(node.to_string()).or_insert_with(Vec::new);` always returns a mutable reference.
        // We can use the `entry` API more explicitly:
        use std::collections::hash_map::Entry;
        match self.adjacency_table_mutable().entry(node.to_string()) {
            Entry::Occupied(_) => false, // Node already existed
            Entry::Vacant(v) => {
                v.insert(Vec::new()); // Insert an empty list for the new node
                true // Node was newly added
            }
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
         // The default implementation might just panic or do nothing if not overridden,
         // but here we assume the implementor (UndirectedGraph) provides its own.
         // Since UndirectedGraph implements add_edge, this default won't be used for it.
         // However, if another struct implemented Graph without overriding add_edge,
         // this default would apply.
         // For this exercise, the UndirectedGraph implementation is what matters.
         // Let's make the default add a directed edge for completeness,
         // but the test relies on the UndirectedGraph implementation.
         let (node1, node2, weight) = edge;
         self.adjacency_table_mutable()
             .entry(node1.to_string())
             .or_default()
             .push((node2.to_string(), weight));
         // For an undirected graph, the implementor must add the reverse edge too.
         // This default does not do that.
         // *** IMPORTANT: The `UndirectedGraph` implementation of `add_edge` is the one used in the test. ***
         // *** Its implementation correctly adds *both* directions. ***
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}