//! Query utilities for world state

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;

use crate::world::World;
use crate::parser::{Item, Character};

/// Query builder for finding objects in the world
pub struct WorldQuery<'a> {
    world: &'a World,
    matcher: SkimMatcherV2,
}

impl<'a> WorldQuery<'a> {
    /// Create a new query builder
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            matcher: SkimMatcherV2::default(),
        }
    }
    
    /// Find an item by name (fuzzy matching)
    pub fn find_item(&self, name: &str) -> Option<(&str, &Item)> {
        let name_lower = name.to_lowercase();
        
        // First try exact match
        for (id, item) in &self.world.items {
            if id.to_lowercase() == name_lower {
                return Some((id, item));
            }
            
            if let Some(item_name) = &item.name {
                if item_name.to_lowercase() == name_lower {
                    return Some((id, item));
                }
            }
        }
        
        // Then try fuzzy match
        let mut best_match = None;
        let mut best_score = 0;
        
        for (id, item) in &self.world.items {
            // Match against ID
            if let Some(score) = self.matcher.fuzzy_match(id, name) {
                if score > best_score {
                    best_score = score;
                    best_match = Some((id.as_str(), item));
                }
            }
            
            // Match against name
            if let Some(item_name) = &item.name {
                if let Some(score) = self.matcher.fuzzy_match(item_name, name) {
                    if score > best_score {
                        best_score = score;
                        best_match = Some((id.as_str(), item));
                    }
                }
            }
        }
        
        best_match
    }
    
    /// Find items in current room
    pub fn find_item_here(&self, name: &str) -> Option<(&str, &Item)> {
        if let Ok(room) = self.world.current_room() {
            // Search only items in this room
            for item_id in &room.items {
                if let Some(item) = self.world.items.get(item_id) {
                    if self.matches_item(item_id, item, name) {
                        return Some((item_id, item));
                    }
                }
            }
        }
        
        None
    }
    
    /// Find items in inventory
    pub fn find_item_inventory(&self, name: &str) -> Option<(&str, &Item)> {
        for item_id in &self.world.inventory {
            if let Some(item) = self.world.items.get(item_id) {
                if self.matches_item(item_id, item, name) {
                    return Some((item_id, item));
                }
            }
        }
        
        None
    }
    
    /// Find a character by name
    pub fn find_character(&self, name: &str) -> Option<(&str, &Character)> {
        let name_lower = name.to_lowercase();
        
        // First try exact match
        for (id, character) in &self.world.characters {
            if id.to_lowercase() == name_lower {
                return Some((id, character));
            }
            
            if let Some(char_name) = &character.name {
                if char_name.to_lowercase() == name_lower {
                    return Some((id, character));
                }
            }
        }
        
        // Then try fuzzy match
        let mut best_match = None;
        let mut best_score = 0;
        
        for (id, character) in &self.world.characters {
            // Match against ID
            if let Some(score) = self.matcher.fuzzy_match(id, name) {
                if score > best_score {
                    best_score = score;
                    best_match = Some((id.as_str(), character));
                }
            }
            
            // Match against name
            if let Some(char_name) = &character.name {
                if let Some(score) = self.matcher.fuzzy_match(char_name, name) {
                    if score > best_score {
                        best_score = score;
                        best_match = Some((id.as_str(), character));
                    }
                }
            }
        }
        
        best_match
    }
    
    /// Find characters in current room
    pub fn find_character_here(&self, name: &str) -> Option<(&str, &Character)> {
        if let Ok(room) = self.world.current_room() {
            for char_id in &room.characters {
                if let Some(character) = self.world.characters.get(char_id) {
                    if self.matches_character(char_id, character, name) {
                        return Some((char_id, character));
                    }
                }
            }
        }
        
        None
    }
    
    /// Check if an item matches a name
    fn matches_item(&self, id: &str, item: &Item, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        
        if id.to_lowercase() == name_lower {
            return true;
        }
        
        if let Some(item_name) = &item.name {
            if item_name.to_lowercase() == name_lower {
                return true;
            }
        }
        
        // Check fuzzy match with threshold
        if let Some(score) = self.matcher.fuzzy_match(id, name) {
            if score > 50 {
                return true;
            }
        }
        
        if let Some(item_name) = &item.name {
            if let Some(score) = self.matcher.fuzzy_match(item_name, name) {
                if score > 50 {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Check if a character matches a name
    fn matches_character(&self, id: &str, character: &Character, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        
        if id.to_lowercase() == name_lower {
            return true;
        }
        
        if let Some(char_name) = &character.name {
            if char_name.to_lowercase() == name_lower {
                return true;
            }
        }
        
        // Check fuzzy match with threshold
        if let Some(score) = self.matcher.fuzzy_match(id, name) {
            if score > 50 {
                return true;
            }
        }
        
        if let Some(char_name) = &character.name {
            if let Some(score) = self.matcher.fuzzy_match(char_name, name) {
                if score > 50 {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get all visible objects (items and characters in current room + inventory)
    pub fn visible_objects(&self) -> HashMap<String, ObjectType> {
        let mut objects = HashMap::new();
        
        // Add inventory items
        for item_id in &self.world.inventory {
            objects.insert(item_id.clone(), ObjectType::InventoryItem);
        }
        
        // Add room items and characters
        if let Ok(room) = self.world.current_room() {
            for item_id in &room.items {
                objects.insert(item_id.clone(), ObjectType::RoomItem);
            }
            
            for char_id in &room.characters {
                objects.insert(char_id.clone(), ObjectType::Character);
            }
        }
        
        objects
    }
}

/// Type of object in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    /// Item in current room
    RoomItem,
    /// Item in inventory
    InventoryItem,
    /// Character in current room
    Character,
}

/// Statistics about the world
pub struct WorldStats {
    /// Total number of rooms
    pub room_count: usize,
    /// Number of visited rooms
    pub visited_rooms: usize,
    /// Total number of items
    pub item_count: usize,
    /// Items in inventory
    pub inventory_count: usize,
    /// Total number of characters
    pub character_count: usize,
    /// Current turn
    pub turns: u32,
    /// Current score
    pub score: i32,
    /// Completion percentage
    pub completion: f32,
}

impl World {
    /// Get world statistics
    pub fn stats(&self) -> WorldStats {
        let visited_rooms = self.rooms.values()
            .filter(|room| room.visited)
            .count();
        
        let total_items = self.items.len();
        let collectible_items = self.items.values()
            .filter(|item| item.takeable)
            .count();
        
        let collected_items = self.inventory.len();
        
        let completion = if collectible_items > 0 {
            (collected_items as f32 / collectible_items as f32) * 100.0
        } else {
            0.0
        };
        
        WorldStats {
            room_count: self.rooms.len(),
            visited_rooms,
            item_count: total_items,
            inventory_count: self.inventory.len(),
            character_count: self.characters.len(),
            turns: self.turn_count,
            score: self.score,
            completion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Item;
    
    #[test]
    fn test_fuzzy_item_search() {
        let mut world = World::new();
        
        // Add some items
        world.items.insert("brass_key".to_string(), Item {
            id: "brass_key".to_string(),
            name: Some("Brass Key".to_string()),
            ..Default::default()
        });
        
        world.items.insert("rusty_key".to_string(), Item {
            id: "rusty_key".to_string(),
            name: Some("Rusty Old Key".to_string()),
            ..Default::default()
        });
        
        let query = WorldQuery::new(&world);
        
        // Test exact match
        assert!(query.find_item("brass_key").is_some());
        
        // Test fuzzy match
        assert!(query.find_item("brass ky").is_some());
        assert!(query.find_item("key").is_some());
        
        // Test name match
        assert!(query.find_item("rusty old key").is_some());
    }
}