//! Graph-based world representation and algorithms

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::Direction as CompassDirection;

/// Room connectivity analyzer
pub struct RoomGraph {
    graph: DiGraph<String, CompassDirection>,
    nodes: HashMap<String, NodeIndex>,
}

impl RoomGraph {
    /// Create a new room graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            nodes: HashMap::new(),
        }
    }
    
    /// Add a room to the graph
    pub fn add_room(&mut self, id: String) -> NodeIndex {
        if let Some(&node) = self.nodes.get(&id) {
            return node;
        }
        
        let node = self.graph.add_node(id.clone());
        self.nodes.insert(id, node);
        node
    }
    
    /// Add a connection between rooms
    pub fn add_connection(&mut self, from: &str, to: &str, direction: CompassDirection) {
        if let (Some(&from_node), Some(&to_node)) = (self.nodes.get(from), self.nodes.get(to)) {
            self.graph.add_edge(from_node, to_node, direction);
        }
    }
    
    /// Check if rooms are connected
    pub fn are_connected(&self, from: &str, to: &str) -> bool {
        if let (Some(&from_node), Some(&to_node)) = (self.nodes.get(from), self.nodes.get(to)) {
            petgraph::algo::has_path_connecting(&self.graph, from_node, to_node, None)
        } else {
            false
        }
    }
    
    /// Find all reachable rooms from a starting point
    pub fn reachable_from(&self, start: &str) -> HashSet<String> {
        let mut reachable = HashSet::new();
        
        if let Some(&start_node) = self.nodes.get(start) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            
            queue.push_back(start_node);
            visited.insert(start_node);
            
            while let Some(node) = queue.pop_front() {
                reachable.insert(self.graph[node].clone());
                
                for edge in self.graph.edges(node) {
                    let target = edge.target();
                    if visited.insert(target) {
                        queue.push_back(target);
                    }
                }
            }
        }
        
        reachable
    }
    
    /// Find isolated room clusters
    pub fn find_clusters(&self) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut clusters = Vec::new();
        
        for &node in self.nodes.values() {
            if visited.contains(&node) {
                continue;
            }
            
            let mut cluster = Vec::new();
            let mut queue = VecDeque::new();
            
            queue.push_back(node);
            visited.insert(node);
            
            while let Some(current) = queue.pop_front() {
                cluster.push(self.graph[current].clone());
                
                // Check both directions (undirected connectivity)
                for edge in self.graph.edges_directed(current, Direction::Outgoing) {
                    let target = edge.target();
                    if visited.insert(target) {
                        queue.push_back(target);
                    }
                }
                
                for edge in self.graph.edges_directed(current, Direction::Incoming) {
                    let source = edge.source();
                    if visited.insert(source) {
                        queue.push_back(source);
                    }
                }
            }
            
            clusters.push(cluster);
        }
        
        clusters
    }
    
    /// Find rooms with only one exit (dead ends)
    pub fn find_dead_ends(&self) -> Vec<String> {
        self.nodes.iter()
            .filter(|(_, &node)| {
                self.graph.edges(node).count() == 1
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Find room hubs (rooms with many connections)
    pub fn find_hubs(&self, min_connections: usize) -> Vec<(String, usize)> {
        self.nodes.iter()
            .filter_map(|(id, &node)| {
                let connections = self.graph.edges(node).count();
                if connections >= min_connections {
                    Some((id.clone(), connections))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Calculate the diameter of the world (longest shortest path)
    pub fn diameter(&self) -> Option<usize> {
        let mut max_distance = 0;
        
        for &start in self.nodes.values() {
            let distances = petgraph::algo::dijkstra(&self.graph, start, None, |_| 1);
            
            for (_, &dist) in &distances {
                max_distance = max_distance.max(dist);
            }
        }
        
        if max_distance > 0 {
            Some(max_distance)
        } else {
            None
        }
    }
    
    /// Find bottlenecks (rooms whose removal would disconnect the graph)
    pub fn find_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();
        
        for (id, &node) in &self.nodes {
            // Create a copy of the graph without this node
            let mut test_graph = self.graph.clone();
            test_graph.remove_node(node);
            
            // Check if graph is still connected
            if !is_connected(&test_graph) {
                bottlenecks.push(id.clone());
            }
        }
        
        bottlenecks
    }
}

/// Check if a graph is connected
fn is_connected(graph: &DiGraph<String, CompassDirection>) -> bool {
    if graph.node_count() == 0 {
        return true;
    }
    
    let start = graph.node_indices().next().unwrap();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        for edge in graph.edges_directed(node, Direction::Outgoing) {
            let target = edge.target();
            if visited.insert(target) {
                queue.push_back(target);
            }
        }
        
        for edge in graph.edges_directed(node, Direction::Incoming) {
            let source = edge.source();
            if visited.insert(source) {
                queue.push_back(source);
            }
        }
    }
    
    visited.len() == graph.node_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_room_graph() {
        let mut graph = RoomGraph::new();
        
        // Create a simple map
        graph.add_room("kitchen".to_string());
        graph.add_room("hallway".to_string());
        graph.add_room("bedroom".to_string());
        
        graph.add_connection("kitchen", "hallway", CompassDirection::North);
        graph.add_connection("hallway", "bedroom", CompassDirection::East);
        
        assert!(graph.are_connected("kitchen", "bedroom"));
        
        let reachable = graph.reachable_from("kitchen");
        assert_eq!(reachable.len(), 3);
    }
    
    #[test]
    fn test_clusters() {
        let mut graph = RoomGraph::new();
        
        // Create two separate areas
        graph.add_room("area1_room1".to_string());
        graph.add_room("area1_room2".to_string());
        graph.add_room("area2_room1".to_string());
        graph.add_room("area2_room2".to_string());
        
        graph.add_connection("area1_room1", "area1_room2", CompassDirection::North);
        graph.add_connection("area2_room1", "area2_room2", CompassDirection::South);
        
        let clusters = graph.find_clusters();
        assert_eq!(clusters.len(), 2);
    }
}