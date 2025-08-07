//! World model for managing game state

use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::parser::{Room, Item, Character};
use crate::types::{Direction as CompassDirection, Value};

mod graph;
mod query;
mod world_extensions;

pub use graph::*;
pub use query::*;

/// The game world containing all locations, items, and characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    /// Graph of room connections
    #[serde(skip)]
    room_graph: DiGraph<String, CompassDirection>,
    
    /// Room ID to graph node mapping
    #[serde(skip)]
    room_nodes: HashMap<String, NodeIndex>,
    
    /// All rooms in the world
    pub rooms: HashMap<String, Room>,
    
    /// All items in the world
    pub items: HashMap<String, Item>,
    
    /// All characters in the world
    pub characters: HashMap<String, Character>,
    
    /// Current player location
    pub player_location: String,
    
    /// Player inventory
    pub inventory: Vec<String>,
    
    /// Global variables
    pub variables: HashMap<String, Value>,
    
    /// Game flags
    pub flags: HashSet<String>,
    
    /// Turn counter
    pub turn_count: u32,
    
    /// Score
    pub score: i32,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        Self {
            room_graph: DiGraph::new(),
            room_nodes: HashMap::new(),
            rooms: HashMap::new(),
            items: HashMap::new(),
            characters: HashMap::new(),
            player_location: String::new(),
            inventory: Vec::new(),
            variables: HashMap::new(),
            flags: HashSet::new(),
            turn_count: 0,
            score: 0,
        }
    }
    
    /// Initialize world from parsed script
    pub fn from_script(script: &crate::parser::Script) -> Result<Self> {
        let mut world = Self::new();
        
        // Add all rooms
        for (id, room) in &script.rooms {
            world.add_room(id.clone(), room.clone())?;
        }
        
        // Add all items
        for (id, item) in &script.items {
            world.add_item(id.clone(), item.clone())?;
        }
        
        // Add all characters
        for (id, character) in &script.characters {
            world.add_character(id.clone(), character.clone())?;
        }
        
        // Set starting location
        if let Some(game) = &script.game {
            if let Some(start) = game.config.get("start_location") {
                if let Some(loc) = start.as_string() {
                    world.player_location = loc.to_string();
                }
            }
        }
        
        // Build room graph
        world.build_room_graph()?;
        
        Ok(world)
    }
    
    /// Add a room to the world
    pub fn add_room(&mut self, id: String, room: Room) -> Result<()> {
        if self.rooms.contains_key(&id) {
            return Err(Error::WorldError(format!("Room '{}' already exists", id)));
        }
        
        self.rooms.insert(id, room);
        Ok(())
    }
    
    /// Add an item to the world
    pub fn add_item(&mut self, id: String, item: Item) -> Result<()> {
        if self.items.contains_key(&id) {
            return Err(Error::WorldError(format!("Item '{}' already exists", id)));
        }
        
        self.items.insert(id, item);
        Ok(())
    }
    
    /// Add a character to the world
    pub fn add_character(&mut self, id: String, character: Character) -> Result<()> {
        if self.characters.contains_key(&id) {
            return Err(Error::WorldError(format!("Character '{}' already exists", id)));
        }
        
        self.characters.insert(id, character);
        Ok(())
    }
    
    /// Build the room connection graph
    fn build_room_graph(&mut self) -> Result<()> {
        self.room_graph.clear();
        self.room_nodes.clear();
        
        // Add nodes for each room
        for room_id in self.rooms.keys() {
            let node = self.room_graph.add_node(room_id.clone());
            self.room_nodes.insert(room_id.clone(), node);
        }
        
        // Add edges for connections
        for (room_id, room) in &self.rooms {
            let from_node = self.room_nodes[room_id];
            
            for (direction, target_id) in &room.exits {
                if let Some(&to_node) = self.room_nodes.get(target_id) {
                    self.room_graph.add_edge(from_node, to_node, *direction);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get the current room
    pub fn current_room(&self) -> Result<&Room> {
        self.rooms.get(&self.player_location)
            .ok_or_else(|| Error::NotFound(format!("Room '{}'", self.player_location)))
    }
    
    /// Get a mutable reference to the current room
    pub fn current_room_mut(&mut self) -> Result<&mut Room> {
        let loc = self.player_location.clone();
        self.rooms.get_mut(&loc)
            .ok_or_else(|| Error::NotFound(format!("Room '{}'", loc)))
    }
    
    /// Move player in a direction
    pub fn move_player(&mut self, direction: CompassDirection) -> Result<String> {
        let target = {
            let current = self.current_room()?;
            current.exits.get(&direction).cloned()
        };
        
        if let Some(target) = target {
            // Check if move is allowed
            if let Some(room) = self.rooms.get(&target) {
                if room.dark && !self.has_light_source() {
                    return Err(Error::RuntimeError("It's too dark to move safely".to_string()));
                }
            }
            
            self.player_location = target.clone();
            self.turn_count += 1;
            
            // Mark room as visited
            if let Some(room) = self.rooms.get_mut(&target) {
                room.visited = true;
            }
            
            Ok(target)
        } else {
            Err(Error::RuntimeError("You can't go that way".to_string()))
        }
    }
    
    /// Check if player has a light source
    pub fn has_light_source(&self) -> bool {
        self.inventory.iter().any(|item_id| {
            self.items.get(item_id)
                .map(|item| item.properties.get("provides_light")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false))
                .unwrap_or(false)
        })
    }
    
    /// Take an item
    pub fn take_item(&mut self, item_id: &str) -> Result<()> {
        let item = self.items.get(item_id)
            .ok_or_else(|| Error::NotFound(format!("Item '{}'", item_id)))?;
        
        if !item.takeable {
            return Err(Error::RuntimeError(format!("You can't take the {}", item_id)));
        }
        
        // Check if item is in current room
        let current = self.current_room()?;
        if !current.items.contains(&item_id.to_string()) {
            return Err(Error::RuntimeError(format!("There's no {} here", item_id)));
        }
        
        // Remove from room and add to inventory
        if let Some(room) = self.rooms.get_mut(&self.player_location) {
            room.items.retain(|id| id != item_id);
        }
        
        self.inventory.push(item_id.to_string());
        self.turn_count += 1;
        
        Ok(())
    }
    
    /// Drop an item
    pub fn drop_item(&mut self, item_id: &str) -> Result<()> {
        if !self.inventory.contains(&item_id.to_string()) {
            return Err(Error::RuntimeError(format!("You don't have the {}", item_id)));
        }
        
        // Remove from inventory
        self.inventory.retain(|id| id != item_id);
        
        // Add to current room
        if let Some(room) = self.rooms.get_mut(&self.player_location) {
            room.items.push(item_id.to_string());
        }
        
        self.turn_count += 1;
        
        Ok(())
    }
    
    /// Get items in current room
    pub fn room_items(&self) -> Result<Vec<&Item>> {
        let room = self.current_room()?;
        Ok(room.items.iter()
            .filter_map(|id| self.items.get(id))
            .collect())
    }
    
    /// Get characters in current room
    pub fn room_characters(&self) -> Result<Vec<&Character>> {
        let room = self.current_room()?;
        Ok(room.characters.iter()
            .filter_map(|id| self.characters.get(id))
            .collect())
    }
    
    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Set a variable value
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// Get a flag value
    pub fn get_flag(&self, name: &str) -> bool {
        self.flags.contains(name)
    }
    
    /// Set a flag value
    pub fn set_flag(&mut self, name: String, value: bool) {
        if value {
            self.flags.insert(name);
        } else {
            self.flags.remove(&name);
        }
    }
    
    /// Find shortest path between rooms
    pub fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        let from_node = self.room_nodes.get(from)?;
        let to_node = self.room_nodes.get(to)?;
        
        let path = petgraph::algo::astar(
            &self.room_graph,
            *from_node,
            |n| n == *to_node,
            |_| 1,
            |_| 0,
        );
        
        path.map(|(_, nodes)| {
            nodes.into_iter()
                .map(|n| self.room_graph[n].clone())
                .collect()
        })
    }
    
    /// Get all connected rooms from current location
    pub fn connected_rooms(&self) -> Vec<(&CompassDirection, &str)> {
        if let Some(&node) = self.room_nodes.get(&self.player_location) {
            self.room_graph
                .edges_directed(node, Direction::Outgoing)
                .map(|edge| {
                    let target = &self.room_graph[edge.target()];
                    (edge.weight(), target.as_str())
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_world_creation() {
        let world = World::new();
        assert_eq!(world.turn_count, 0);
        assert_eq!(world.score, 0);
        assert!(world.inventory.is_empty());
    }
    
    #[test]
    fn test_add_room() {
        let mut world = World::new();
        let room = Room {
            id: "kitchen".to_string(),
            title: Some("Kitchen".to_string()),
            ..Default::default()
        };
        
        assert!(world.add_room("kitchen".to_string(), room).is_ok());
        assert!(world.rooms.contains_key("kitchen"));
    }
    
    #[test]
    fn test_movement() {
        let mut world = World::new();
        
        // Create two connected rooms
        let mut kitchen = Room {
            id: "kitchen".to_string(),
            ..Default::default()
        };
        kitchen.exits.insert(CompassDirection::North, "hallway".to_string());
        
        let hallway = Room {
            id: "hallway".to_string(),
            ..Default::default()
        };
        
        world.add_room("kitchen".to_string(), kitchen).unwrap();
        world.add_room("hallway".to_string(), hallway).unwrap();
        world.player_location = "kitchen".to_string();
        world.build_room_graph().unwrap();
        
        // Test movement
        assert!(world.move_player(CompassDirection::North).is_ok());
        assert_eq!(world.player_location, "hallway");
        assert_eq!(world.turn_count, 1);
    }
}