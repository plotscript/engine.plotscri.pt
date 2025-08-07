//! Extended World methods for all game formats

use crate::error::{Error, Result};
use crate::parser::{Room, Item, Character};
use crate::types::{Direction, Value};

impl super::World {
    /// Get a room by ID
    pub fn get_room(&self, room_id: &str) -> Option<&Room> {
        self.rooms.get(room_id)
    }
    
    /// Get a mutable room by ID
    pub fn get_room_mut(&mut self, room_id: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_id)
    }
    
    /// Get an item by ID
    pub fn get_item(&self, item_id: &str) -> Option<&Item> {
        self.items.get(item_id)
    }
    
    /// Get a mutable item by ID
    pub fn get_item_mut(&mut self, item_id: &str) -> Option<&mut Item> {
        self.items.get_mut(item_id)
    }
    
    /// Get a character by ID
    pub fn get_character(&self, character_id: &str) -> Option<&Character> {
        self.characters.get(character_id)
    }
    
    /// Get a mutable character by ID
    pub fn get_character_mut(&mut self, character_id: &str) -> Option<&mut Character> {
        self.characters.get_mut(character_id)
    }
    
    /// Place an item in a location
    pub fn place_item(&mut self, item_id: String, location_id: String) -> Result<()> {
        // Make sure the item exists
        if !self.items.contains_key(&item_id) {
            return Err(Error::NotFound(format!("Item '{}'", item_id)));
        }
        
        // Make sure the location exists
        if !self.rooms.contains_key(&location_id) {
            return Err(Error::NotFound(format!("Room '{}'", location_id)));
        }
        
        // Add item to room
        if let Some(room) = self.rooms.get_mut(&location_id) {
            if !room.items.contains(&item_id) {
                room.items.push(item_id.clone());
            }
        }
        
        // Update item location
        if let Some(item) = self.items.get_mut(&item_id) {
            item.location = Some(location_id);
        }
        
        Ok(())
    }
    
    
    /// Add a location
    pub fn add_location(&mut self, id: &str, name: &str, description: &str) {
        let room = Room {
            id: id.to_string(),
            title: Some(name.to_string()),
            description: Some(description.to_string()),
            ..Default::default()
        };
        self.add_room(id.to_string(), room).ok();
    }
    
    /// Connect two locations
    pub fn connect_locations(&mut self, from: &str, to: &str, direction: &str, bidirectional: bool) {
        if let Some(dir) = Direction::from_str(direction) {
            if let Some(from_room) = self.rooms.get_mut(from) {
                from_room.exits.insert(dir, to.to_string());
            }
            
            if bidirectional {
                let opposite = dir.opposite();
                if let Some(to_room) = self.rooms.get_mut(to) {
                    to_room.exits.insert(opposite, from.to_string());
                }
            }
        }
    }
    
    /// Check if can go in a direction from a location
    pub fn can_go_from(&self, from: &str, direction: &str) -> bool {
        if let Some(room) = self.rooms.get(from) {
            if let Some(dir) = Direction::from_str(direction) {
                return room.exits.contains_key(&dir);
            }
        }
        false
    }
    
    /// Get destination from a location in a direction
    pub fn get_destination(&self, from: &str, direction: &str) -> Option<&str> {
        if let Some(room) = self.rooms.get(from) {
            if let Some(dir) = Direction::from_str(direction) {
                return room.exits.get(&dir).map(|s| s.as_str());
            }
        }
        None
    }
    
    /// Add an object (simplified item)
    pub fn add_object(&mut self, id: &str, name: &str, is_container: bool) {
        let item = Item {
            id: id.to_string(),
            name: Some(name.to_string()),
            is_container,
            ..Default::default()
        };
        self.add_item(id.to_string(), item).ok();
    }
    
    /// Put item in container
    pub fn put_in_container(&mut self, item_id: &str, container_id: &str) {
        if let Some(container) = self.items.get_mut(container_id) {
            if container.is_container && !container.contains.contains(&item_id.to_string()) {
                container.contains.push(item_id.to_string());
            }
        }
    }
    
    /// Get container contents
    pub fn get_container_contents(&self, container_id: &str) -> Vec<String> {
        self.items.get(container_id)
            .map(|c| c.contains.clone())
            .unwrap_or_default()
    }
    
    /// Set global state
    pub fn set_global_state(&mut self, key: &str, value: Value) {
        self.variables.insert(format!("global_{}", key), value);
    }
    
    /// Get global state
    pub fn get_global_state(&self, key: &str) -> Option<&Value> {
        self.variables.get(&format!("global_{}", key))
    }
    
    /// Set location state
    pub fn set_location_state(&mut self, location: &str, key: &str, value: Value) {
        self.variables.insert(format!("location_{}_{}", location, key), value);
    }
    
    /// Get location state
    pub fn get_location_state(&self, location: &str, key: &str) -> Option<&Value> {
        self.variables.get(&format!("location_{}_{}", location, key))
    }
    
    /// Add physical object
    pub fn add_physical_object(&mut self, id: &str, mass: f32, position: (f32, f32, f32)) {
        let mut item = Item {
            id: id.to_string(),
            name: Some(id.to_string()),
            weight: mass,
            ..Default::default()
        };
        
        item.properties.insert("pos_x".to_string(), Value::Float(position.0 as f64));
        item.properties.insert("pos_y".to_string(), Value::Float(position.1 as f64));
        item.properties.insert("pos_z".to_string(), Value::Float(position.2 as f64));
        item.properties.insert("mass".to_string(), Value::Float(mass as f64));
        
        self.add_item(id.to_string(), item).ok();
    }
    
    /// Apply gravity to object
    pub fn apply_gravity(&mut self, object_id: &str, gravity: f32) {
        if let Some(object) = self.items.get_mut(object_id) {
            let current_y = object.properties.get("pos_y")
                .and_then(|v| v.as_float())
                .unwrap_or(0.0);
            
            // Simple gravity: move down
            let new_y = current_y - (gravity as f64 * 0.016); // Assuming 60fps
            object.properties.insert("pos_y".to_string(), Value::Float(new_y));
        }
    }
    
    /// Get object position
    pub fn get_object_position(&self, object_id: &str) -> (f32, f32, f32) {
        if let Some(object) = self.items.get(object_id) {
            let x = object.properties.get("pos_x")
                .and_then(|v| v.as_float())
                .unwrap_or(0.0) as f32;
            let y = object.properties.get("pos_y")
                .and_then(|v| v.as_float())
                .unwrap_or(0.0) as f32;
            let z = object.properties.get("pos_z")
                .and_then(|v| v.as_float())
                .unwrap_or(0.0) as f32;
            (x, y, z)
        } else {
            (0.0, 0.0, 0.0)
        }
    }
    
    /// Set object position
    pub fn set_object_position(&mut self, object_id: &str, position: (f32, f32, f32)) {
        if let Some(object) = self.items.get_mut(object_id) {
            object.properties.insert("pos_x".to_string(), Value::Float(position.0 as f64));
            object.properties.insert("pos_y".to_string(), Value::Float(position.1 as f64));
            object.properties.insert("pos_z".to_string(), Value::Float(position.2 as f64));
        }
    }
    
    /// Check collision between two objects
    pub fn check_collision(&self, object1_id: &str, object2_id: &str) -> bool {
        let pos1 = self.get_object_position(object1_id);
        let pos2 = self.get_object_position(object2_id);
        
        // Simple distance check (assuming sphere collision)
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;
        let dz = pos1.2 - pos2.2;
        let distance_squared = dx * dx + dy * dy + dz * dz;
        
        // Collision if within 1 unit
        distance_squared < 1.0
    }
}

// Add helper trait for Value
impl Value {
    /// Get as float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
}