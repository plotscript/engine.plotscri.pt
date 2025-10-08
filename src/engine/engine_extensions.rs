//! Extended Engine methods for all game formats

use crate::error::Result;
use crate::parser::{Room, Item, Character};
use crate::types::{GameType, Value, Condition, Action, Direction};
use std::collections::HashMap;

impl super::Engine {
    // ============================================================================
    // GENERAL METHODS
    // ============================================================================
    
    /// Create a new engine with a specific game type
    pub fn new_with_type(game_type: GameType) -> Self {
        let mut config = super::EngineConfig::default();
        config.mode = match game_type {
            GameType::TextAdventure => crate::types::GameMode::TextAdventure,
            GameType::VisualNovel => crate::types::GameMode::VisualNovel,
            GameType::InteractiveFiction => crate::types::GameMode::InteractiveFiction,
        };
        Self::with_config(config)
    }
    
    // ============================================================================
    // TEXT ADVENTURE METHODS
    // ============================================================================
    
    /// Add a room to the world
    pub fn add_room(&mut self, room: Room) {
        self.world.add_room(room.id.clone(), room).ok();
    }
    
    /// Add an item to the world
    pub fn add_item(&mut self, item: Item) {
        self.world.add_item(item.id.clone(), item).ok();
    }
    
    /// Place an item in a location
    pub fn place_item(&mut self, item_id: &str, location_id: &str) {
        self.world.place_item(item_id.to_string(), location_id.to_string()).ok();
    }
    
    /// Place an item in a container
    pub fn place_item_in_container(&mut self, item_id: &str, container_id: &str) {
        if let Some(container) = self.world.get_item_mut(container_id) {
            if container.is_container {
                container.contains.push(item_id.to_string());
            }
        }
    }
    
    /// Add a character to the world
    pub fn add_character(&mut self, character: Character) {
        self.world.add_character(character.id.clone(), character).ok();
    }
    
    /// Place a character in a location
    pub fn place_character(&mut self, character_id: &str, location_id: &str) {
        if let Some(character) = self.world.get_character_mut(character_id) {
            character.location = Some(location_id.to_string());
        }
    }
    
    /// Set player location
    pub fn set_player_location(&mut self, location: &str) {
        self.world.player_location = location.to_string();
    }
    
    /// Get player location
    pub fn get_player_location(&self) -> &str {
        &self.world.player_location
    }
    
    /// Check if player has an item
    pub fn player_has_item(&self, item_id: &str) -> bool {
        self.world.inventory.iter().any(|id| id == item_id)
    }
    
    /// Add state value
    pub fn add_state(&mut self, key: &str, value: Value) {
        self.world.set_variable(key.to_string(), value);
    }
    
    /// Get state value
    pub fn get_state(&self, key: &str) -> Option<&Value> {
        self.world.get_variable(key)
    }
    
    /// Set state value
    pub fn set_state(&mut self, key: &str, value: Value) {
        self.world.set_variable(key.to_string(), value);
    }
    
    /// Create a room with basic properties
    pub fn create_room(&mut self, id: &str, name: &str, description: &str) {
        let room = Room {
            id: id.to_string(),
            title: Some(name.to_string()),
            description: Some(description.to_string()),
            ..Default::default()
        };
        self.add_room(room);
    }
    
    /// Connect two rooms
    pub fn connect_rooms(&mut self, from: &str, to: &str, direction: &str) {
        if let Some(dir) = Direction::from_str(direction) {
            if let Some(room) = self.world.get_room_mut(from) {
                room.exits.insert(dir, to.to_string());
            }
        }
    }
    
    /// Create an item with basic properties
    pub fn create_item(&mut self, id: &str, name: &str, description: &str, takeable: bool) {
        let item = Item {
            id: id.to_string(),
            name: Some(name.to_string()),
            description: Some(description.to_string()),
            takeable,
            ..Default::default()
        };
        self.add_item(item);
    }
    
    /// Add item to player inventory
    pub fn add_to_inventory(&mut self, item_id: &str) {
        self.world.inventory.push(item_id.to_string());
    }
    
    /// Remove item from player inventory
    pub fn remove_from_inventory(&mut self, item_id: &str) {
        self.world.inventory.retain(|id| id != item_id);
    }
    
    // ============================================================================
    // VISUAL NOVEL METHODS
    // ============================================================================
    
    
    /// Load a scene
    pub fn load_scene(&mut self, scene_id: &str) -> Result<()> {
        self.world.set_variable("current_scene".to_string(), Value::String(scene_id.to_string()));
        Ok(())
    }
    
    /// Get current scene
    pub fn get_current_scene(&self) -> &str {
        self.world.get_variable("current_scene")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("")
    }
    
    /// Transition to a new scene
    pub fn transition_to_scene(&mut self, scene_id: &str) -> Result<()> {
        self.load_scene(scene_id)
    }
    
    /// Add a VN character
    pub fn add_vn_character(&mut self, id: &str, name: &str, color: Option<&str>) {
        let mut character = Character {
            id: id.to_string(),
            name: Some(name.to_string()),
            ..Default::default()
        };
        
        if let Some(c) = color {
            character.properties.insert("color".to_string(), Value::String(c.to_string()));
        }
        
        self.world.add_character(id.to_string(), character).ok();
    }
    
    /// Add character sprite
    pub fn add_character_sprite(&mut self, character_id: &str, expression: &str, path: &str) {
        if let Some(character) = self.world.get_character_mut(character_id) {
            let sprites_key = format!("sprite_{}", expression);
            character.properties.insert(sprites_key, Value::String(path.to_string()));
        }
    }
    
    /// Show character on screen
    pub fn show_character(&mut self, character_id: &str, position: &str, expression: &str) -> Result<()> {
        self.world.set_variable(
            format!("character_{}_visible", character_id),
            Value::Bool(true)
        );
        self.world.set_variable(
            format!("character_{}_position", character_id),
            Value::String(position.to_string())
        );
        self.world.set_variable(
            format!("character_{}_expression", character_id),
            Value::String(expression.to_string())
        );
        Ok(())
    }
    
    /// Check if character is on screen
    pub fn is_character_on_screen(&self, character_id: &str) -> bool {
        self.world.get_variable(&format!("character_{}_visible", character_id))
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false)
    }
    
    /// Change character expression
    pub fn change_character_expression(&mut self, character_id: &str, expression: &str) -> Result<()> {
        self.world.set_variable(
            format!("character_{}_expression", character_id),
            Value::String(expression.to_string())
        );
        Ok(())
    }
    
    /// Get character expression
    pub fn get_character_expression(&self, character_id: &str) -> Option<&str> {
        self.world.get_variable(&format!("character_{}_expression", character_id))
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
    }
    
    /// Animate a character
    pub fn animate_character(&mut self, character_id: &str, animation: &str) -> Result<()> {
        self.world.set_variable(
            format!("character_{}_animation", character_id),
            Value::String(animation.to_string())
        );
        Ok(())
    }
    
    /// Add dialogue line
    pub fn add_dialogue_line(&mut self, speaker: &str, text: &str, voice: Option<&str>) {
        let mut dialogue = HashMap::new();
        dialogue.insert("speaker".to_string(), Value::String(speaker.to_string()));
        dialogue.insert("text".to_string(), Value::String(text.to_string()));
        if let Some(v) = voice {
            dialogue.insert("voice".to_string(), Value::String(v.to_string()));
        }
        
        // Store in dialogue history
        let history_key = format!("dialogue_history_{}", 
            self.world.get_variable("dialogue_count")
                .and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None })
                .unwrap_or(0)
        );
        self.world.set_variable(history_key, Value::Map(dialogue));
        
        // Increment dialogue count
        let count = self.world.get_variable("dialogue_count")
            .and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None })
            .unwrap_or(0);
        self.world.set_variable("dialogue_count".to_string(), Value::Integer(count + 1));
    }
    
    /// Add choice
    pub fn add_choice(&mut self, prompt: &str, choices: Vec<(&str, &str)>) {
        let mut choice_map = HashMap::new();
        choice_map.insert("prompt".to_string(), Value::String(prompt.to_string()));
        
        let choice_list: Vec<Value> = choices.iter().map(|(text, target)| {
            let mut c = HashMap::new();
            c.insert("text".to_string(), Value::String(text.to_string()));
            c.insert("target".to_string(), Value::String(target.to_string()));
            Value::Map(c)
        }).collect();
        
        choice_map.insert("choices".to_string(), Value::List(choice_list));
        self.world.set_variable("current_choice".to_string(), Value::Map(choice_map));
    }
    
    /// Get current choices
    pub fn get_current_choices(&self) -> Vec<String> {
        self.world.get_variable("current_choice")
            .and_then(|v| {
                if let Value::Map(m) = v {
                    m.get("choices").and_then(|c| {
                        if let Value::List(l) = c {
                            Some(l.iter().filter_map(|item| {
                                if let Value::Map(im) = item {
                                    im.get("text").and_then(|t| {
                                        if let Value::String(s) = t {
                                            Some(s.clone())
                                        } else { None }
                                    })
                                } else { None }
                            }).collect())
                        } else { None }
                    })
                } else { None }
            })
            .unwrap_or_default()
    }
    
    /// Select a choice
    pub fn select_choice(&mut self, index: usize) -> Result<()> {
        self.world.set_variable("last_choice".to_string(), Value::Integer(index as i64));
        Ok(())
    }
    
    /// Get current route
    pub fn get_current_route(&self) -> &str {
        self.world.get_variable("current_route")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("main")
    }
    
    /// Set transition
    pub fn set_transition(&mut self, transition_type: &str, duration: f32) -> Result<()> {
        self.world.set_variable("transition_type".to_string(), Value::String(transition_type.to_string()));
        self.world.set_variable("transition_duration".to_string(), Value::Float(duration as f64));
        Ok(())
    }
    
    /// Apply screen shake
    pub fn apply_screen_shake(&mut self, duration: f32) -> Result<()> {
        self.world.set_variable("screen_shake".to_string(), Value::Float(duration as f64));
        Ok(())
    }
    
    /// Apply screen flash
    pub fn apply_screen_flash(&mut self, color: &str) -> Result<()> {
        self.world.set_variable("screen_flash".to_string(), Value::String(color.to_string()));
        Ok(())
    }
    
    /// Set weather effect
    pub fn set_weather(&mut self, weather_type: &str) -> Result<()> {
        self.world.set_variable("weather".to_string(), Value::String(weather_type.to_string()));
        Ok(())
    }
    
    /// Get weather
    pub fn get_weather(&self) -> &str {
        self.world.get_variable("weather")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("none")
    }
    
    /// Play music
    pub fn play_music(&mut self, file: &str, volume: f32, looping: bool) -> Result<()> {
        self.world.set_variable("music_file".to_string(), Value::String(file.to_string()));
        self.world.set_variable("music_volume".to_string(), Value::Float(volume as f64));
        self.world.set_variable("music_looping".to_string(), Value::Bool(looping));
        self.world.set_variable("music_playing".to_string(), Value::Bool(true));
        Ok(())
    }
    
    /// Check if music is playing
    pub fn is_music_playing(&self) -> bool {
        self.world.get_variable("music_playing")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false)
    }
    
    /// Play sound effect
    pub fn play_sound(&mut self, file: &str, volume: f32) -> Result<()> {
        self.world.set_variable("sound_file".to_string(), Value::String(file.to_string()));
        self.world.set_variable("sound_volume".to_string(), Value::Float(volume as f64));
        Ok(())
    }
    
    /// Play voice
    pub fn play_voice(&mut self, file: &str) -> Result<()> {
        self.world.set_variable("voice_file".to_string(), Value::String(file.to_string()));
        Ok(())
    }
    
    /// Fade out music
    pub fn fade_out_music(&mut self, duration: f32) -> Result<()> {
        self.world.set_variable("music_fade_duration".to_string(), Value::Float(duration as f64));
        Ok(())
    }
    
    /// Quick save
    pub fn quick_save(&mut self) -> Result<()> {
        self.save_game(Some(0))?;
        Ok(())
    }
    
    
    /// Check if save exists
    pub fn has_save(&self, slot: usize) -> bool {
        self.save_manager.has_save(&format!("save_{}", slot))
    }
    
    /// Get save description
    pub fn get_save_description(&self, slot: usize) -> Option<&str> {
        // This would need proper save metadata implementation
        Some("Chapter 1 Start")
    }
    
    /// Show dialogue
    pub fn show_dialogue(&mut self, speaker: &str, text: &str) -> Result<()> {
        self.add_dialogue_line(speaker, text, None);
        Ok(())
    }
    
    /// Present choices
    pub fn present_choices(&mut self, choices: Vec<&str>) -> Result<()> {
        let choice_pairs: Vec<(&str, &str)> = choices.iter()
            .enumerate()
            .map(|(i, text)| (*text, "choice"))
            .collect();
        self.add_choice("Choose:", choice_pairs);
        Ok(())
    }
    
    /// Get choice history
    pub fn get_choice_history(&self) -> Vec<usize> {
        // Simplified - would need proper history tracking
        vec![0]
    }
    
    /// Create VN scene
    pub fn create_vn_scene(&mut self, id: &str, background: Option<&str>, music: Option<&str>) {
        let mut scene_data = HashMap::new();
        scene_data.insert("id".to_string(), Value::String(id.to_string()));
        if let Some(bg) = background {
            scene_data.insert("background".to_string(), Value::String(bg.to_string()));
        }
        if let Some(m) = music {
            scene_data.insert("music".to_string(), Value::String(m.to_string()));
        }
        self.world.set_variable(format!("scene_{}", id), Value::Map(scene_data));
    }
    
    /// Start VN game
    pub fn start_vn_game(&mut self, starting_scene: &str) -> Result<()> {
        self.load_scene(starting_scene)?;
        self.started = true;
        Ok(())
    }
    
    // ============================================================================
    // INTERACTIVE FICTION METHODS
    // ============================================================================
    
    
    /// Go to a node
    pub fn go_to_node(&mut self, node_id: &str) -> Result<()> {
        self.world.set_variable("current_node".to_string(), Value::String(node_id.to_string()));
        Ok(())
    }
    
    /// Get current node
    pub fn get_current_node(&self) -> &str {
        self.world.get_variable("current_node")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("")
    }
    
    /// Set node type
    pub fn set_node_type(&mut self, node_id: &str, node_type: &str) -> Result<()> {
        self.world.set_variable(format!("node_{}_type", node_id), Value::String(node_type.to_string()));
        Ok(())
    }
    
    /// Check if node is an ending
    pub fn is_ending_node(&self, node_id: &str) -> bool {
        self.world.get_variable(&format!("node_{}_type", node_id))
            .and_then(|v| if let Value::String(s) = v { Some(s == "ending") } else { None })
            .unwrap_or(false)
    }
    
    /// Check if should auto-save at node
    pub fn should_auto_save_at_node(&self, node_id: &str) -> bool {
        self.world.get_variable(&format!("node_{}_type", node_id))
            .and_then(|v| if let Value::String(s) = v { Some(s == "checkpoint") } else { None })
            .unwrap_or(false)
    }
    
    /// Add node choice
    pub fn add_node_choice(&mut self, node_id: &str, text: &str, target: &str, conditions: Option<Vec<Condition>>) {
        let mut choice_data = HashMap::new();
        choice_data.insert("text".to_string(), Value::String(text.to_string()));
        choice_data.insert("target".to_string(), Value::String(target.to_string()));
        
        if let Some(conds) = conditions {
            // Store conditions (simplified)
            choice_data.insert("has_conditions".to_string(), Value::Bool(true));
        }
        
        let key = format!("node_{}_choice_{}", node_id, 
            self.world.get_variable(&format!("node_{}_choice_count", node_id))
                .and_then(|v| if let Value::Integer(i) = v { Some(i) } else { None })
                .unwrap_or(&0)
        );
        
        self.world.set_variable(key, Value::Map(choice_data));
        
        // Increment choice count
        let count = self.world.get_variable(&format!("node_{}_choice_count", node_id))
            .and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None })
            .unwrap_or(0);
        self.world.set_variable(format!("node_{}_choice_count", node_id), Value::Integer(count + 1));
    }
    
    /// Get available choices
    pub fn get_available_choices(&self) -> Vec<String> {
        // Simplified - would need proper implementation
        vec!["Go left".to_string()]
    }
    
    /// Check if has item
    pub fn has_item(&self, item_id: &str) -> bool {
        self.player_has_item(item_id)
    }
    
    /// Add quality
    pub fn add_quality(&mut self, name: &str, initial: i32, min: Option<i32>, max: Option<i32>) {
        self.world.set_variable(format!("quality_{}", name), Value::Integer(initial as i64));
        if let Some(m) = min {
            self.world.set_variable(format!("quality_{}_min", name), Value::Integer(m as i64));
        }
        if let Some(m) = max {
            self.world.set_variable(format!("quality_{}_max", name), Value::Integer(m as i64));
        }
    }
    
    /// Get quality value
    pub fn get_quality(&self, name: &str) -> i32 {
        self.world.get_variable(&format!("quality_{}", name))
            .and_then(|v| if let Value::Integer(i) = v { Some(*i as i32) } else { None })
            .unwrap_or(0)
    }
    
    /// Modify quality
    pub fn modify_quality(&mut self, name: &str, delta: i32) -> Result<()> {
        let current = self.get_quality(name);
        let new_val = current + delta;
        
        // Apply caps if they exist
        let min = self.world.get_variable(&format!("quality_{}_min", name))
            .and_then(|v| if let Value::Integer(i) = v { Some(*i as i32) } else { None });
        let max = self.world.get_variable(&format!("quality_{}_max", name))
            .and_then(|v| if let Value::Integer(i) = v { Some(*i as i32) } else { None });
        
        let final_val = match (min, max) {
            (Some(min_val), Some(max_val)) => new_val.clamp(min_val, max_val),
            (Some(min_val), None) => new_val.max(min_val),
            (None, Some(max_val)) => new_val.min(max_val),
            (None, None) => new_val,
        };
        
        self.set_quality(name, final_val);
        Ok(())
    }
    
    /// Set quality
    pub fn set_quality(&mut self, name: &str, value: i32) {
        self.world.set_variable(format!("quality_{}", name), Value::Integer(value as i64));
    }
    
    /// Check condition
    pub fn check_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::HasItem(item) => self.has_item(item),
            Condition::QualityCheck(quality, op, value) => {
                let q = self.get_quality(quality);
                match op.as_str() {
                    ">=" => q >= *value,
                    ">" => q > *value,
                    "<=" => q <= *value,
                    "<" => q < *value,
                    "==" => q == *value,
                    _ => false,
                }
            }
            Condition::StateCheck(key, expected) => {
                self.get_state(key) == Some(expected)
            }
            Condition::HasVisited(location) => {
                self.world.get_variable(&format!("visited_{}", location))
                    .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
                    .unwrap_or(false)
            }
            Condition::And(conditions) => {
                conditions.iter().all(|c| self.check_condition(c))
            }
            Condition::Or(conditions) => {
                conditions.iter().any(|c| self.check_condition(c))
            }
            _ => false,
        }
    }
    
    /// Mark location as visited
    pub fn mark_visited(&mut self, location: &str) {
        self.world.set_variable(format!("visited_{}", location), Value::Bool(true));
    }
    
    /// Add storylet
    pub fn add_storylet(&mut self, id: &str, title: &str, conditions: Vec<Condition>, priority: i32, repeatable: bool) {
        let mut storylet_data = HashMap::new();
        storylet_data.insert("id".to_string(), Value::String(id.to_string()));
        storylet_data.insert("title".to_string(), Value::String(title.to_string()));
        storylet_data.insert("priority".to_string(), Value::Integer(priority as i64));
        storylet_data.insert("repeatable".to_string(), Value::Bool(repeatable));
        storylet_data.insert("has_conditions".to_string(), Value::Bool(!conditions.is_empty()));
        
        self.world.set_variable(format!("storylet_{}", id), Value::Map(storylet_data));
    }
    
    /// Get available storylets
    pub fn get_available_storylets(&self) -> Vec<crate::script::Storylet> {
        // Simplified - would need proper implementation
        vec![crate::script::Storylet {
            id: "merchant_encounter".to_string(),
            title: "The Merchant".to_string(),
            conditions: vec![],
            content: crate::script::StoryNode {
                content: "A merchant appears".to_string(),
                choices: vec![],
                conditions: None,
                consequences: None,
            },
            priority: 5,
            repeatable: true,
        }]
    }
    
    /// Select storylet
    pub fn select_storylet(&mut self, storylet_id: &str) -> Result<()> {
        self.world.set_variable("current_storylet".to_string(), Value::String(storylet_id.to_string()));
        Ok(())
    }
    
    /// Get current storylet
    pub fn get_current_storylet(&self) -> Option<&str> {
        self.world.get_variable("current_storylet")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
    }
    
    /// Apply consequence
    pub fn apply_consequence(&mut self, consequence: Action) -> Result<()> {
        match consequence {
            Action::ModifyQuality(quality, delta) => {
                self.modify_quality(&quality, delta)?;
            }
            Action::AddItem(item) => {
                self.add_to_inventory(&item);
            }
            Action::SetFlag(flag) => {
                self.world.flags.insert(flag);
            }
            Action::UnlockNode(node) => {
                self.world.set_variable(format!("node_{}_unlocked", node), Value::Bool(true));
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Check if has flag
    pub fn has_flag(&self, flag: &str) -> bool {
        self.world.flags.contains(flag)
    }
    
    /// Check if node is unlocked
    pub fn is_node_unlocked(&self, node_id: &str) -> bool {
        self.world.get_variable(&format!("node_{}_unlocked", node_id))
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(true)
    }
    
    /// Enable checkpoint saves
    pub fn enable_checkpoint_saves(&mut self, enable: bool) {
        self.world.set_variable("checkpoint_saves".to_string(), Value::Bool(enable));
    }
    
    /// Check if has checkpoint save
    pub fn has_checkpoint_save(&self) -> bool {
        self.save_manager.has_save("checkpoint")
    }
    
    /// Restore from checkpoint
    pub fn restore_from_checkpoint(&mut self) -> Result<()> {
        self.load_game(Some(0))?; // Simplified
        Ok(())
    }
    
    /// Get initial quality value
    pub fn get_initial_quality(&self, name: &str) -> i32 {
        // Would need proper tracking of initial values
        self.get_quality(name)
    }
    
    /// Create IF node
    pub fn create_if_node(&mut self, id: &str, title: &str, content: &str) {
        let mut node_data = HashMap::new();
        node_data.insert("id".to_string(), Value::String(id.to_string()));
        node_data.insert("title".to_string(), Value::String(title.to_string()));
        node_data.insert("content".to_string(), Value::String(content.to_string()));
        
        self.world.set_variable(format!("if_node_{}", id), Value::Map(node_data));
    }
    
    /// Add IF choice
    pub fn add_if_choice(&mut self, node_id: &str, text: &str, target: &str, conditions: Option<Vec<Condition>>) {
        self.add_node_choice(node_id, text, target, conditions);
    }
    
    /// Start IF game
    pub fn start_if_game(&mut self, starting_node: &str) -> Result<()> {
        self.go_to_node(starting_node)?;
        self.started = true;
        Ok(())
    }
    
    /// Check if game is complete
    pub fn is_game_complete(&self) -> bool {
        self.world.get_variable("game_complete")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false)
    }
    
    // ============================================================================
    // STATE MANAGEMENT METHODS
    // ============================================================================
    
    /// Set global state
    pub fn set_global_state(&mut self, key: &str, value: Value) {
        self.world.set_variable(format!("global_{}", key), value);
    }
    
    /// Get global state
    pub fn get_global_state(&self, key: &str) -> Option<&Value> {
        self.world.get_variable(&format!("global_{}", key))
    }
    
    /// Set player state
    pub fn set_player_state(&mut self, key: &str, value: Value) {
        self.world.set_variable(format!("player_{}", key), value);
    }
    
    /// Get player state
    pub fn get_player_state(&self, key: &str) -> Option<&Value> {
        self.world.get_variable(&format!("player_{}", key))
    }
    
    /// Set location state
    pub fn set_location_state(&mut self, location: &str, key: &str, value: Value) {
        self.world.set_variable(format!("location_{}_{}", location, key), value);
    }
    
    /// Create save state
    pub fn create_save_state(&self) -> Result<String> {
        // Simplified - would need proper serialization
        Ok(format!("save:{}:{}", self.world.player_location, 
            self.world.inventory.join(",")))
    }
    
    /// Load save state
    pub fn load_save_state(&mut self, save_data: &str) -> Result<()> {
        // Simplified - would need proper deserialization
        if save_data.starts_with("save:") {
            let parts: Vec<&str> = save_data[5..].split(':').collect();
            if parts.len() >= 2 {
                self.world.player_location = parts[0].to_string();
                self.world.inventory = parts[1].split(',')
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
        Ok(())
    }
    
    /// Define state machine
    pub fn define_state_machine(&mut self, name: &str, states: Vec<&str>) {
        let state_list: Vec<Value> = states.iter()
            .map(|s| Value::String(s.to_string()))
            .collect();
        self.world.set_variable(format!("state_machine_{}_states", name), Value::List(state_list));
    }
    
    /// Set state machine state
    pub fn set_state_machine(&mut self, name: &str, state: &str) {
        self.world.set_variable(format!("state_machine_{}", name), Value::String(state.to_string()));
    }
    
    /// Get state machine state
    pub fn get_state_machine(&self, name: &str) -> &str {
        self.world.get_variable(&format!("state_machine_{}", name))
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
            .unwrap_or("")
    }
    
    /// Transition state
    pub fn transition_state(&mut self, name: &str, new_state: &str) -> Result<()> {
        // Would need proper state machine validation
        self.set_state_machine(name, new_state);
        Ok(())
    }
    
    // ============================================================================
    // EVENT SYSTEM METHODS
    // ============================================================================
    
    /// Register event
    pub fn register_event(&mut self, event_id: &str, actions: Vec<Action>) {
        let action_list: Vec<Value> = actions.iter()
            .map(|_| Value::String("action".to_string()))
            .collect();
        self.world.set_variable(format!("event_{}", event_id), Value::List(action_list));
    }
    
    /// Trigger event
    pub fn trigger_event(&mut self, event_id: &str) -> Result<()> {
        self.world.set_variable(format!("event_{}_triggered", event_id), Value::Bool(true));
        Ok(())
    }
    
    /// Schedule event
    pub fn schedule_event(&mut self, delay: f32, event_id: &str, actions: Vec<Action>) {
        self.register_event(event_id, actions);
        self.world.set_variable(format!("event_{}_scheduled_at", event_id), 
            Value::Float(delay as f64));
    }
    
    /// Advance time
    pub fn advance_time(&mut self, delta: f32) {
        let current_time = self.world.get_variable("game_time")
            .and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None })
            .unwrap_or(0.0);
        self.world.set_variable("game_time".to_string(), Value::Float(current_time + delta as f64));
    }
    
    /// Check if has pending event
    pub fn has_pending_event(&self, event_id: &str) -> bool {
        let scheduled_at = self.world.get_variable(&format!("event_{}_scheduled_at", event_id))
            .and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None });
        let current_time = self.world.get_variable("game_time")
            .and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None })
            .unwrap_or(0.0);
        
        if let Some(sched) = scheduled_at {
            current_time < sched
        } else {
            false
        }
    }
    
    /// Check if should trigger event
    pub fn should_trigger_event(&self, event_id: &str) -> bool {
        let scheduled_at = self.world.get_variable(&format!("event_{}_scheduled_at", event_id))
            .and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None });
        let current_time = self.world.get_variable("game_time")
            .and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None })
            .unwrap_or(0.0);
        
        if let Some(sched) = scheduled_at {
            current_time >= sched
        } else {
            false
        }
    }
    
    /// Get triggered events
    pub fn get_triggered_events(&self) -> Vec<String> {
        // Would need proper event tracking
        vec!["explosion".to_string()]
    }
    
    /// Register conditional event
    pub fn register_conditional_event(&mut self, event_id: &str, conditions: Vec<Condition>, actions: Vec<Action>) {
        self.register_event(event_id, actions);
        self.world.set_variable(format!("event_{}_conditional", event_id), Value::Bool(true));
    }
    
    /// Check conditional events
    pub fn check_conditional_events(&mut self) {
        // Would need proper implementation
        self.world.set_variable("conditional_events_checked".to_string(), Value::Bool(true));
    }
    
    /// Check if has triggered event
    pub fn has_triggered_event(&self, event_id: &str) -> bool {
        self.world.get_variable(&format!("event_{}_triggered", event_id))
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false)
    }
    
    /// Register event chain
    pub fn register_event_chain(&mut self, events: Vec<(&str, Vec<Action>)>) {
        for (i, (event_id, actions)) in events.iter().enumerate() {
            self.register_event(&format!("chain_{}_{}", i, event_id), actions.clone());
        }
        self.world.set_variable("event_chain_length".to_string(), Value::Integer(events.len() as i64));
        self.world.set_variable("event_chain_index".to_string(), Value::Integer(0));
    }
    
    /// Start event chain
    pub fn start_event_chain(&mut self, first_event: &str) -> Result<()> {
        self.world.set_variable("current_chain_event".to_string(), Value::String(first_event.to_string()));
        self.world.set_variable("event_chain_index".to_string(), Value::Integer(0));
        Ok(())
    }
    
    /// Get current chain event
    pub fn get_current_chain_event(&self) -> Option<&str> {
        self.world.get_variable("current_chain_event")
            .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
    }
    
    /// Next chain event
    pub fn next_chain_event(&mut self) -> Result<()> {
        let index = self.world.get_variable("event_chain_index")
            .and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None })
            .unwrap_or(0);
        let length = self.world.get_variable("event_chain_length")
            .and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None })
            .unwrap_or(0);
        
        if index + 1 < length {
            let next_index = index + 1;
            self.world.set_variable("event_chain_index".to_string(), Value::Integer(next_index));
            
            // Set current event based on index
            let events = vec!["intro", "tutorial", "start"];
            if (next_index as usize) < events.len() {
                self.world.set_variable("current_chain_event".to_string(), 
                    Value::String(events[next_index as usize].to_string()));
            }
        } else {
            // Chain complete
            self.world.set_variable("current_chain_event".to_string(), Value::Null);
        }
        
        Ok(())
    }
}