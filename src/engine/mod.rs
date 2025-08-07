//! Main game engine that coordinates all subsystems

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::parser::{self, DialogueTree};
use crate::runtime::Runtime;
use crate::types::{GameMode, Response, GameState, Value, Choice};
use crate::world::World;
use crate::extensions::ExtensionManager;

mod commands;
mod formatter;
mod save;
mod engine_extensions;

pub use commands::*;
pub use formatter::*;
pub use save::*;

/// Engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Game mode
    pub mode: GameMode,
    /// Enable debug output
    pub debug: bool,
    /// Maximum inventory size
    pub max_inventory: usize,
    /// Enable auto-save
    pub auto_save: bool,
    /// Command history size
    pub history_size: usize,
    /// Enable typo correction
    pub typo_correction: bool,
    /// Typo correction threshold (0-100)
    pub typo_threshold: u8,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            mode: GameMode::default(),
            debug: false,
            max_inventory: 20,
            auto_save: true,
            history_size: 100,
            typo_correction: true,
            typo_threshold: 70,
        }
    }
}

/// Main PlotScript engine
pub struct Engine {
    /// Engine configuration
    pub config: EngineConfig,
    /// Parsed script
    script: Option<parser::Script>,
    /// Game world state
    world: World,
    /// Script runtime
    runtime: Runtime,
    /// Command parser
    command_parser: commands::CommandParser,
    /// Output formatter
    formatter: OutputFormatter,
    /// Command history
    history: Vec<String>,
    /// Save game manager
    save_manager: SaveManager,
    /// Game started flag
    started: bool,
    /// Extension manager
    extension_manager: ExtensionManager,
    /// Game state (exposed for extensions)
    pub state: GameState,
    /// Active dialogue state (character name, current node index)
    active_dialogue: Option<(String, usize)>,
}

impl Engine {
    /// Create a new engine with default config
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }
    
    /// Create a new engine with custom config
    pub fn with_config(config: EngineConfig) -> Self {
        let command_parser = if config.typo_correction {
            commands::CommandParser::with_threshold(config.typo_threshold)
        } else {
            commands::CommandParser::with_threshold(100) // Require exact match
        };
        
        Self {
            config,
            script: None,
            world: World::new(),
            runtime: Runtime::new(),
            command_parser,
            formatter: OutputFormatter::new(),
            history: Vec::new(),
            save_manager: SaveManager::new(),
            started: false,
            extension_manager: ExtensionManager::new(),
            state: GameState::new(),
            active_dialogue: None,
        }
    }
    
    /// Load a script from source
    pub fn load_script(&mut self, source: &str) -> Result<()> {
        // Try to parse as RON/YAML first (new format)
        match crate::script::GameScript::from_ron(source) {
            Ok(game_script) => return self.load_game_script(game_script),
            Err(_) => {
                // Try YAML if RON failed
                match crate::script::GameScript::from_yaml(source) {
                    Ok(game_script) => return self.load_game_script(game_script),
                    Err(_) => {
                        // Both failed, try old parser format
                    }
                }
            }
        }
        
        // Fall back to old parser format
        let script = parser::parse_script(source)?;
        
        // Initialize world from script
        self.world = World::from_script(&script)?;
        
        // Initialize runtime with script
        self.runtime.load_script(&script)?;
        
        // Set game mode from script
        if let Some(game) = &script.game {
            self.config.mode = game.mode;
        }
        
        self.script = Some(script);
        self.started = false;
        
        Ok(())
    }
    
    /// Load a game script (new format)
    fn load_game_script(&mut self, game_script: crate::script::GameScript) -> Result<()> {
        use crate::script::GameScript;
        
        // Update config based on script
        self.config.mode = game_script.game_mode();
        
        // Initialize based on game type
        match game_script {
            GameScript::TextAdventure(ta_script) => {
                self.load_text_adventure(ta_script)?;
            }
            GameScript::VisualNovel(vn_script) => {
                self.load_visual_novel(vn_script)?;
            }
            GameScript::InteractiveFiction(if_script) => {
                self.load_interactive_fiction(if_script)?;
            }
        }
        
        self.started = false;
        Ok(())
    }
    
    /// Load a text adventure script
    fn load_text_adventure(&mut self, script: crate::script::TextAdventureScript) -> Result<()> {
        use std::collections::HashMap;
        
        // Initialize world
        self.world = World::new();
        self.world.player_location = script.starting_location.clone();
        
        // Create a dummy parser script for compatibility
        self.script = Some(crate::parser::Script {
            game: Some(crate::parser::GameDefinition {
                title: script.title.clone(),
                author: Some(script.author.clone()),
                description: script.description.clone(),
                version: script.version.clone(),
                mode: GameMode::TextAdventure,
                config: HashMap::new(),
            }),
            rooms: HashMap::new(),
            items: HashMap::new(),
            characters: HashMap::new(),
            events: Vec::new(),
            functions: HashMap::new(),
            imports: Vec::new(),
        });
        
        // Add locations
        for (id, location) in script.locations {
            let mut room = crate::parser::Room {
                id: id.clone(),
                title: Some(location.name),
                description: Some(location.description.clone()),
                exits: location.exits,
                items: location.items,
                characters: location.characters,
                dark: location.dark.unwrap_or(false),
                visited: false,
                on_enter: Vec::new(),
                on_exit: Vec::new(),
                properties: HashMap::new(),
            };
            
            // Add properties
            room.properties.insert("description".to_string(), Value::String(location.description));
            if let Some(dark) = location.dark {
                room.properties.insert("dark".to_string(), Value::Bool(dark));
            }
            if let Some(first_visit) = location.first_visit {
                room.properties.insert("first_visit".to_string(), Value::String(first_visit));
            }
            
            self.world.add_room(id, room)?;
        }
        
        // Add items
        for (id, item) in script.items {
            let mut game_item = crate::parser::Item {
                id: id.clone(),
                name: Some(item.name),
                description: Some(item.description),
                location: None,
                takeable: item.takeable,
                weight: item.weight.unwrap_or(1) as f32,
                is_container: item.container.unwrap_or(false),
                contains: item.contains.unwrap_or_default(),
                on_take: vec![],
                on_use: vec![],
                on_examine: vec![],
                properties: HashMap::new(),
            };
            
            // Add extra properties
            if let Some(openable) = item.openable {
                game_item.properties.insert("openable".to_string(), Value::Bool(openable));
            }
            if let Some(locked) = item.locked {
                game_item.properties.insert("locked".to_string(), Value::Bool(locked));
            }
            if let Some(key) = item.key {
                game_item.properties.insert("key".to_string(), Value::String(key));
            }
            
            self.world.add_item(id, game_item)?;
        }
        
        // Add characters
        for (id, character) in script.characters {
            let mut game_char = crate::parser::Character {
                id: id.clone(),
                name: Some(character.name),
                description: Some(character.description),
                location: None,
                dialogue: DialogueTree::default(),
                state: "default".to_string(),
                relationship: 0,
                properties: HashMap::new(),
            };
            
            // Convert dialogue if present
            if let Some(dialogue) = character.dialogue {
                // Convert our script dialogue to AST dialogue
                let mut nodes = Vec::new();
                for (node_id, node) in dialogue.nodes {
                    let mut responses = Vec::new();
                    if let Some(resp_list) = node.responses {
                        for resp in resp_list {
                            responses.push(crate::parser::DialogueResponse {
                                text: resp.text,
                                next: Some(resp.next),
                                actions: vec![],
                                conditions: vec![],
                            });
                        }
                    }
                    
                    nodes.push(crate::parser::DialogueNode {
                        id: node_id,
                        speaker: id.clone(),
                        text: node.text,
                        responses,
                        conditions: vec![],
                    });
                }
                game_char.dialogue.nodes = nodes;
            }
            
            self.world.add_character(id, game_char)?;
        }
        
        Ok(())
    }
    
    /// Load a visual novel script
    fn load_visual_novel(&mut self, script: crate::script::VisualNovelScript) -> Result<()> {
        use std::collections::HashMap;
        
        // Initialize visual novel state
        self.world = World::new();
        self.world.player_location = "_vn_mode".to_string();
        
        // Store VN-specific data in world variables
        self.world.set_variable("vn_current_scene".to_string(), Value::String(script.starting_scene.clone()));
        self.world.set_variable("vn_dialogue_index".to_string(), Value::Float(0.0));
        self.world.set_variable("vn_auto_mode".to_string(), Value::Bool(false));
        self.world.set_variable("vn_skip_mode".to_string(), Value::Bool(false));
        
        // Store scenes as JSON in variables for retrieval
        let scenes_json = serde_json::to_string(&script.scenes)
            .map_err(|e| Error::RuntimeError(format!("Failed to serialize scenes: {}", e)))?;
        self.world.set_variable("vn_scenes".to_string(), Value::String(scenes_json));
        
        // Store characters
        let chars_json = serde_json::to_string(&script.characters)
            .map_err(|e| Error::RuntimeError(format!("Failed to serialize characters: {}", e)))?;
        self.world.set_variable("vn_characters".to_string(), Value::String(chars_json));
        
        // Store settings
        self.world.set_variable("vn_text_speed".to_string(), Value::String(format!("{:?}", script.settings.text_speed)));
        self.world.set_variable("vn_auto_save".to_string(), Value::Bool(script.settings.auto_save));
        
        // Create a dummy script for compatibility
        self.script = Some(crate::parser::Script {
            game: Some(crate::parser::GameDefinition {
                title: script.title.clone(),
                author: Some(script.author.clone()),
                description: script.description.clone(),
                version: None,
                mode: GameMode::VisualNovel,
                config: HashMap::new(),
            }),
            rooms: HashMap::new(),
            items: HashMap::new(),
            characters: HashMap::new(),
            events: Vec::new(),
            functions: HashMap::new(),
            imports: Vec::new(),
        });
        
        Ok(())
    }
    
    /// Load an interactive fiction script
    fn load_interactive_fiction(&mut self, script: crate::script::InteractiveFictionScript) -> Result<()> {
        use std::collections::HashMap;
        
        // Initialize IF state
        self.world = World::new();
        self.world.player_location = "_if_mode".to_string();
        
        // Store IF-specific data
        self.world.set_variable("if_current_node".to_string(), Value::String(script.starting_node.clone()));
        self.world.set_variable("if_show_stats".to_string(), Value::Bool(script.settings.show_stats));
        self.world.set_variable("if_timed_choices".to_string(), Value::Bool(script.settings.timed_choices));
        
        // Initialize qualities
        for (name, quality) in &script.qualities {
            let qual_name = format!("quality_{}", name);
            self.world.set_variable(qual_name, Value::Float(quality.initial as f64));
            
            // Store min/max if present
            if let Some(min) = quality.min {
                self.world.set_variable(format!("quality_{}_min", name), Value::Float(min as f64));
            }
            if let Some(max) = quality.max {
                self.world.set_variable(format!("quality_{}_max", name), Value::Float(max as f64));
            }
        }
        
        // Store nodes as JSON
        let nodes_json = serde_json::to_string(&script.nodes)
            .map_err(|e| Error::RuntimeError(format!("Failed to serialize nodes: {}", e)))?;
        self.world.set_variable("if_nodes".to_string(), Value::String(nodes_json));
        
        // Store storylets if present
        if let Some(storylets) = &script.storylets {
            let storylets_json = serde_json::to_string(storylets)
                .map_err(|e| Error::RuntimeError(format!("Failed to serialize storylets: {}", e)))?;
            self.world.set_variable("if_storylets".to_string(), Value::String(storylets_json));
        }
        
        // Create dummy script for compatibility
        self.script = Some(crate::parser::Script {
            game: Some(crate::parser::GameDefinition {
                title: script.title.clone(),
                author: Some(script.author.clone()),
                description: script.description.clone(),
                version: None,
                mode: GameMode::InteractiveFiction,
                config: HashMap::new(),
            }),
            rooms: HashMap::new(),
            items: HashMap::new(),
            characters: HashMap::new(),
            events: Vec::new(),
            functions: HashMap::new(),
            imports: Vec::new(),
        });
        
        Ok(())
    }
    
    /// Start the game
    pub fn start(&mut self) -> Result<Response> {
        if self.script.is_none() {
            return Err(Error::InvalidState("No script loaded".to_string()));
        }
        
        self.started = true;
        
        // Run startup events
        self.runtime.run_event("game_start", &mut self.world)?;
        
        // Get initial response based on game mode
        match self.config.mode {
            GameMode::TextAdventure => self.look(),
            GameMode::VisualNovel => self.get_vn_scene(),
            GameMode::InteractiveFiction => self.get_if_node(),
        }
    }
    
    /// Process player input
    pub fn process_input(&mut self, input: &str) -> Result<Response> {
        if !self.started {
            return Err(Error::InvalidState("Game not started".to_string()));
        }
        
        // Add to history
        if self.history.len() >= self.config.history_size {
            self.history.remove(0);
        }
        self.history.push(input.to_string());
        
        // Process based on game mode
        match self.config.mode {
            GameMode::TextAdventure => self.process_command(input),
            GameMode::VisualNovel => self.process_vn_input(input),
            GameMode::InteractiveFiction => self.process_if_choice(input),
        }
    }
    
    /// Process a text adventure command
    fn process_command(&mut self, input: &str) -> Result<Response> {
        // Check if we're in an active dialogue
        if let Some((char_id, node_idx)) = &self.active_dialogue.clone() {
            // Try to parse input as a dialogue choice number
            if let Ok(choice_num) = input.trim().parse::<usize>() {
                return self.select_dialogue_response(char_id, *node_idx, choice_num - 1);
            }
            
            // Allow "exit" or "leave" to end dialogue
            if input.trim().eq_ignore_ascii_case("exit") || input.trim().eq_ignore_ascii_case("leave") {
                self.active_dialogue = None;
                return Ok(Response {
                    text: "You end the conversation.".to_string(),
                    location: Some(self.world.player_location.clone()),
                    choices: Vec::new(),
                    state: self.get_game_state(),
                    media: Vec::new(),
                    sounds: Vec::new(),
                    achievement: None,
                    success: true,
                    ended: false,
                });
            }
        }
        
        // First try extensions
        let parts: Vec<&str> = input.split_whitespace().collect();
        if !parts.is_empty() {
            let command = parts[0];
            let args = &parts[1..];
            
            // Update game state for extensions
            self.state = self.get_game_state();
            
            if let Some(response) = self.extension_manager.process_command(command, args, &mut self.state) {
                // Apply state changes back to the engine
                self.apply_state_changes();
                return Ok(response);
            }
        }
        
        // Parse the command normally
        let command = self.command_parser.parse(input, &self.world)?;
        
        // Execute the command
        match command {
            Command::Look => self.look(),
            Command::Go(direction) => self.go(direction),
            Command::Take(item) => self.take(&item),
            Command::Drop(item) => self.drop(&item),
            Command::Use(item) => self.use_item(&item),
            Command::Examine(target) => self.examine(&target),
            Command::Inventory => self.inventory(),
            Command::Talk(character) => self.talk_to(&character),
            Command::Save(slot) => self.save_game(slot),
            Command::Load(slot) => self.load_game(slot),
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Debug(cmd) => self.debug_command(&cmd),
            _ => Err(Error::RuntimeError("Unknown command".to_string())),
        }
    }
    
    /// Look around the current location
    fn look(&mut self) -> Result<Response> {
        let room = self.world.current_room()?;
        let mut text = String::new();
        
        // Room title
        if let Some(title) = &room.title {
            text.push_str(&format!("## {}\n\n", title));
        }
        
        // Room description
        if let Some(desc) = &room.description {
            text.push_str(desc);
            text.push_str("\n\n");
        }
        
        // List exits
        if !room.exits.is_empty() {
            text.push_str("Exits: ");
            let exits: Vec<String> = room.exits.keys()
                .map(|dir| format!("{:?}", dir).to_lowercase())
                .collect();
            text.push_str(&exits.join(", "));
            text.push_str("\n\n");
        }
        
        // List items
        let items = self.world.room_items()?;
        if !items.is_empty() {
            text.push_str("You can see: ");
            let item_names: Vec<String> = items.iter()
                .map(|item| item.name.as_deref().unwrap_or(&item.id))
                .map(|s| s.to_string())
                .collect();
            text.push_str(&item_names.join(", "));
            text.push_str("\n");
        }
        
        // List characters
        let characters = self.world.room_characters()?;
        if !characters.is_empty() {
            text.push_str("Also here: ");
            let char_names: Vec<String> = characters.iter()
                .map(|chr| chr.name.as_deref().unwrap_or(&chr.id))
                .map(|s| s.to_string())
                .collect();
            text.push_str(&char_names.join(", "));
            text.push_str("\n");
        }
        
        Ok(Response {
            text,
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Move in a direction
    fn go(&mut self, direction: crate::types::Direction) -> Result<Response> {
        let old_location = self.world.player_location.clone();
        let new_location = self.world.move_player(direction)?;
        
        // Run exit events for old room
        self.runtime.run_room_event(&old_location, "on_exit", &mut self.world)?;
        
        // Run enter events for new room
        self.runtime.run_room_event(&new_location, "on_enter", &mut self.world)?;
        
        // Auto-save if enabled
        if self.config.auto_save {
            self.auto_save()?;
        }
        
        self.look()
    }
    
    /// Take an item
    fn take(&mut self, item_name: &str) -> Result<Response> {
        // Check inventory limit
        if self.world.inventory.len() >= self.config.max_inventory {
            return Ok(Response {
                text: "You're carrying too much already.".to_string(),
                location: Some(self.world.player_location.clone()),
                choices: Vec::new(),
                state: self.get_game_state(),
                media: Vec::new(),
                sounds: Vec::new(),
                achievement: None,
                success: false,
                ended: false,
            });
        }
        
        // Find the item
        let query = crate::world::WorldQuery::new(&self.world);
        let (item_id, _) = query.find_item_here(item_name)
            .ok_or_else(|| Error::NotFound(format!("item '{}'", item_name)))?;
        
        let item_id = item_id.to_string();
        
        // Take it
        self.world.take_item(&item_id)?;
        
        // Run on_take events
        self.runtime.run_item_event(&item_id, "on_take", &mut self.world)?;
        
        Ok(Response {
            text: format!("You take the {}.", item_name),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Drop an item
    fn drop(&mut self, item_name: &str) -> Result<Response> {
        // Find the item in inventory
        let query = crate::world::WorldQuery::new(&self.world);
        let (item_id, _) = query.find_item_inventory(item_name)
            .ok_or_else(|| Error::NotFound(format!("item '{}' in inventory", item_name)))?;
        
        let item_id = item_id.to_string();
        
        // Drop it
        self.world.drop_item(&item_id)?;
        
        Ok(Response {
            text: format!("You drop the {}.", item_name),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Use an item
    fn use_item(&mut self, item_name: &str) -> Result<Response> {
        // Find the item (in inventory or room)
        let query = crate::world::WorldQuery::new(&self.world);
        let (item_id, _) = query.find_item_inventory(item_name)
            .or_else(|| query.find_item_here(item_name))
            .ok_or_else(|| Error::NotFound(format!("item '{}'", item_name)))?;
        
        let item_id = item_id.to_string();
        
        // Run on_use events
        self.runtime.run_item_event(&item_id, "on_use", &mut self.world)?;
        
        Ok(Response {
            text: format!("You use the {}.", item_name),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Examine something
    fn examine(&mut self, target: &str) -> Result<Response> {
        let text;
        let mut found_item_id = None;
        
        // Try to find as item
        {
            let query = crate::world::WorldQuery::new(&self.world);
            if let Some((item_id, item)) = query.find_item_inventory(target)
                .or_else(|| query.find_item_here(target)) {
                
                if let Some(desc) = &item.description {
                    text = desc.clone();
                } else {
                    text = format!("It's a {}.", item.name.as_deref().unwrap_or(item_id));
                }
                found_item_id = Some(item_id.to_string());
                
            } else if let Some((char_id, character)) = query.find_character_here(target) {
                // Found as character
                if let Some(desc) = &character.description {
                    text = desc.clone();
                } else {
                    text = format!("It's {}.", character.name.as_deref().unwrap_or(char_id));
                }
            } else {
                return Err(Error::NotFound(format!("'{}'", target)));
            }
        }
        
        // Run on_examine events if we found an item
        if let Some(item_id) = found_item_id {
            self.runtime.run_item_event(&item_id, "on_examine", &mut self.world)?;
        }
        
        Ok(Response {
            text,
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Show inventory
    fn inventory(&mut self) -> Result<Response> {
        let mut text = String::new();
        
        if self.world.inventory.is_empty() {
            text.push_str("You're not carrying anything.");
        } else {
            text.push_str("You are carrying:\n");
            for item_id in &self.world.inventory {
                if let Some(item) = self.world.items.get(item_id) {
                    text.push_str(&format!("- {}\n", 
                        item.name.as_deref().unwrap_or(item_id)));
                }
            }
        }
        
        Ok(Response {
            text,
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Talk to a character
    fn talk_to(&mut self, character_name: &str) -> Result<Response> {
        // Find the character
        let query = crate::world::WorldQuery::new(&self.world);
        let (char_id, character) = query.find_character_here(character_name)
            .ok_or_else(|| Error::NotFound(format!("character '{}'", character_name)))?;
        
        // Check if character has dialogue nodes
        if !character.dialogue.nodes.is_empty() {
            // Start dialogue at first node
            self.active_dialogue = Some((char_id.to_string(), 0));
            
            // Get the first dialogue node
            let node = &character.dialogue.nodes[0];
            
            // Build response with dialogue choices
            let mut choices = Vec::new();
            for (idx, response) in node.responses.iter().enumerate() {
                // Check conditions
                let enabled = response.conditions.iter().all(|cond| {
                    self.runtime.check_condition(cond, &self.world).unwrap_or(false)
                });
                
                choices.push(Choice {
                    id: idx.to_string(),
                    text: response.text.clone(),
                    enabled,
                    hint: None,
                });
            }
            
            Ok(Response {
                text: format!("{}: {}", node.speaker, node.text),
                location: Some(self.world.player_location.clone()),
                choices,
                state: self.get_game_state(),
                media: Vec::new(),
                sounds: Vec::new(),
                achievement: None,
                success: true,
                ended: false,
            })
        } else {
            let text = format!("{} doesn't seem to have anything to say right now.", character_name);
            
            Ok(Response {
                text,
                location: Some(self.world.player_location.clone()),
                choices: Vec::new(),
                state: self.get_game_state(),
                media: Vec::new(),
                sounds: Vec::new(),
                achievement: None,
                success: true,
                ended: false,
            })
        }
    }
    
    /// Select a dialogue response and continue the dialogue
    fn select_dialogue_response(&mut self, char_id: &str, current_node_idx: usize, choice_idx: usize) -> Result<Response> {
        // Extract the data we need before mutating world
        let (selected_response_data, dialogue_nodes) = {
            let character = self.world.characters.get(char_id)
                .ok_or_else(|| Error::NotFound(format!("character '{}'", char_id)))?;
            
            // Check bounds
            if current_node_idx >= character.dialogue.nodes.len() {
                return Err(Error::InvalidState("Invalid dialogue state".to_string()));
            }
            
            // Get the current node
            let current_node = &character.dialogue.nodes[current_node_idx];
            
            // Get the selected response
            if choice_idx >= current_node.responses.len() {
                return Err(Error::InvalidInput("Invalid choice number".to_string()));
            }
            
            // Clone the response data and dialogue nodes before we mutate world
            (current_node.responses[choice_idx].clone(), character.dialogue.nodes.clone())
        };
        
        // Check if the choice is enabled
        let enabled = selected_response_data.conditions.iter().all(|cond| {
            self.runtime.check_condition(cond, &self.world).unwrap_or(false)
        });
        
        if !enabled {
            return Err(Error::InvalidInput("That choice is not available".to_string()));
        }
        
        // Execute response actions
        for action in &selected_response_data.actions {
            self.runtime.execute_action(action, &mut self.world)?;
        }
        
        // Find the next node by ID
        let next_node_id = &selected_response_data.next;
        
        // Check if the next node is "end" to exit dialogue
        if next_node_id.as_deref() == Some("end") || next_node_id.as_deref() == Some("exit") {
            self.active_dialogue = None;
            return Ok(Response {
                text: "The conversation ends.".to_string(),
                location: Some(self.world.player_location.clone()),
                choices: Vec::new(),
                state: self.get_game_state(),
                media: Vec::new(),
                sounds: Vec::new(),
                achievement: None,
                success: true,
                ended: false,
            });
        }
        
        // Find the next node index by its ID
        let next_node_id_str = next_node_id.as_ref()
            .ok_or_else(|| Error::InvalidScript("No next node specified".to_string()))?;
        let next_node_idx = dialogue_nodes.iter()
            .position(|n| n.id == *next_node_id_str)
            .ok_or_else(|| Error::InvalidScript(format!("Missing dialogue node: {}", next_node_id_str)))?;
        
        // Update dialogue state
        self.active_dialogue = Some((char_id.to_string(), next_node_idx));
        
        // Get the next node
        let next_node = &dialogue_nodes[next_node_idx];
        
        // Build response with new choices
        let mut choices = Vec::new();
        for (idx, response) in next_node.responses.iter().enumerate() {
            let enabled = response.conditions.iter().all(|cond| {
                self.runtime.check_condition(cond, &self.world).unwrap_or(false)
            });
            
            choices.push(Choice {
                id: idx.to_string(),
                text: response.text.clone(),
                enabled,
                hint: None,
            });
        }
        
        // If no responses, end dialogue
        if choices.is_empty() {
            self.active_dialogue = None;
        }
        
        Ok(Response {
            text: format!("{}: {}", next_node.speaker, next_node.text),
            location: Some(self.world.player_location.clone()),
            choices,
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Save the game
    pub fn save_game(&mut self, slot: Option<u8>) -> Result<Response> {
        let slot = slot.unwrap_or(1);
        self.save_manager.save(&self.world, &self.runtime, slot)?;
        
        Ok(Response {
            text: format!("Game saved to slot {}.", slot),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Load a saved game
    pub fn load_game(&mut self, slot: Option<u8>) -> Result<Response> {
        let slot = slot.unwrap_or(1);
        let (world, runtime) = self.save_manager.load(slot)?;
        
        self.world = world;
        self.runtime = runtime;
        
        Ok(Response {
            text: format!("Game loaded from slot {}.", slot),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Show help
    fn help(&mut self) -> Result<Response> {
        let text = self.formatter.format_help(self.config.mode);
        
        Ok(Response {
            text,
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Quit the game
    fn quit(&mut self) -> Result<Response> {
        Ok(Response {
            text: "Thanks for playing!".to_string(),
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: true,  // Game is ending
        })
    }
    
    /// Debug command
    fn debug_command(&mut self, cmd: &str) -> Result<Response> {
        if !self.config.debug {
            return Err(Error::PermissionDenied("Debug mode not enabled".to_string()));
        }
        
        // TODO: Implement debug commands
        let text = format!("Debug: {}", cmd);
        
        Ok(Response {
            text,
            location: Some(self.world.player_location.clone()),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Process visual novel input
    fn process_vn_input(&mut self, input: &str) -> Result<Response> {
        // Handle special commands
        match input.trim().to_lowercase().as_str() {
            "next" | "" | "continue" => self.advance_vn_dialogue(),
            "back" | "previous" => self.previous_vn_dialogue(),
            "auto" => self.toggle_vn_auto_mode(),
            "skip" => self.toggle_vn_skip_mode(),
            "save" => self.save_game(None),
            "load" => self.load_game(None),
            "menu" | "quit" => self.quit(),
            choice if choice.starts_with("choice:") => {
                let choice_idx = choice.trim_start_matches("choice:").trim();
                if let Ok(idx) = choice_idx.parse::<usize>() {
                    self.select_vn_choice(idx)
                } else {
                    self.advance_vn_dialogue()
                }
            }
            _ => {
                // Try to parse as a choice number
                if let Ok(choice_num) = input.trim().parse::<usize>() {
                    self.select_vn_choice(choice_num)
                } else {
                    self.advance_vn_dialogue()
                }
            }
        }
    }
    
    /// Process interactive fiction choice
    fn process_if_choice(&mut self, choice_id: &str) -> Result<Response> {
        // Try to parse as choice index
        if let Ok(idx) = choice_id.trim().parse::<usize>() {
            self.select_if_choice(idx)
        } else {
            // Try to match choice text
            self.select_if_choice_by_text(choice_id)
        }
    }
    
    /// Get current game state
    fn get_game_state(&self) -> GameState {
        GameState {
            mode: self.config.mode,
            variables: self.world.variables.clone(),
            inventory: self.world.inventory.clone(),
            score: self.world.score,
            turns: self.world.turn_count,
            flags: self.world.flags.clone(),
        }
    }
    
    /// Auto-save the game
    fn auto_save(&mut self) -> Result<()> {
        self.save_manager.auto_save(&self.world, &self.runtime)
    }
    
    // Extension management methods
    
    /// Register an extension
    pub fn register_extension(&mut self, mut extension: Box<dyn crate::extensions::Extension>) -> Result<()> {
        // Call on_load before registering
        extension.on_load(self)?;
        self.extension_manager.register_extension(extension)
    }
    
    /// Unregister an extension
    pub fn unregister_extension(&mut self, name: &str) -> Result<()> {
        // Note: We can't call on_unload here due to borrowing rules
        // Extensions should clean up in their Drop implementation if needed
        self.extension_manager.unregister_extension(name)
    }
    
    /// Register a custom condition
    pub fn register_condition(&mut self, condition: Box<dyn crate::extensions::Condition>) -> Result<()> {
        self.extension_manager.register_condition(condition)
    }
    
    /// Register a custom action
    pub fn register_action(&mut self, action: Box<dyn crate::extensions::Action>) -> Result<()> {
        self.extension_manager.register_action(action)
    }
    
    /// List all loaded extensions
    pub fn list_extensions(&self) -> Vec<crate::extensions::ExtensionMetadata> {
        self.extension_manager.list_extensions()
    }
    
    /// Check if an extension is loaded
    pub fn has_extension(&self, name: &str) -> bool {
        self.extension_manager.has_extension(name)
    }
    
    /// Apply state changes from extensions back to the engine
    fn apply_state_changes(&mut self) {
        // Apply variable changes
        for (key, value) in &self.state.variables {
            self.world.set_variable(key.clone(), value.clone());
        }
        
        // Apply flags
        self.world.flags = self.state.flags.clone();
        
        // Apply inventory changes if needed
        // This is a simplified version - a full implementation would handle more complex state synchronization
    }
    
    // ============================================================================
    // VISUAL NOVEL MODE IMPLEMENTATION
    // ============================================================================
    
    /// Get current VN scene
    fn get_vn_scene(&mut self) -> Result<Response> {
        let current_scene = self.world.get_variable("vn_current_scene")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current scene".to_string()))?;
        
        let dialogue_idx = self.world.get_variable("vn_dialogue_index")
            .and_then(|v| if let Value::Float(n) = v { Some(*n as usize) } else { None })
            .unwrap_or(0);
        
        // Deserialize scenes
        let scenes_json = self.world.get_variable("vn_scenes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No scenes data".to_string()))?;
        
        let scenes: HashMap<String, crate::script::Scene> = serde_json::from_str(&scenes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse scenes: {}", e)))?;
        
        let scene = scenes.get(&current_scene)
            .ok_or_else(|| Error::NotFound(format!("Scene '{}'", current_scene)))?;
        
        // Get current dialogue line
        if dialogue_idx >= scene.dialogue.len() {
            // Scene ended
            return Ok(Response {
                text: "[Scene Complete]".to_string(),
                location: Some(current_scene),
                choices: vec![crate::types::Choice {
                    id: "continue".to_string(),
                    text: "Continue to next scene".to_string(),
                    enabled: true,
                    hint: None,
                }],
                state: self.get_game_state(),
                media: scene.background.clone().map(|bg| vec![crate::types::Media {
                    media_type: crate::types::MediaType::Background,
                    source: bg,
                    position: crate::types::MediaPosition::Background,
                    alt: None,
                }]).unwrap_or_default(),
                sounds: scene.music.clone().map(|m| vec![crate::types::Sound {
                    source: m,
                    volume: 1.0,
                    looping: true,
                    fade_in: Some(1000),
                }]).unwrap_or_default(),
                achievement: None,
                success: true,
                ended: false,
            });
        }
        
        let dialogue = &scene.dialogue[dialogue_idx];
        let mut text = String::new();
        
        // Add speaker if present
        if let Some(speaker) = &dialogue.speaker {
            text.push_str(&format!("**{}**: ", speaker));
        }
        text.push_str(&dialogue.text);
        
        // Get choices if present
        let choices = if let Some(choices) = &dialogue.choices {
            choices.iter().enumerate().map(|(i, c)| crate::types::Choice {
                id: format!("choice_{}", i),
                text: c.text.clone(),
                enabled: true,
                hint: None,
            }).collect()
        } else {
            Vec::new()
        };
        
        // Build media list
        let mut media = Vec::new();
        if let Some(bg) = &scene.background {
            media.push(crate::types::Media {
                media_type: crate::types::MediaType::Background,
                source: bg.clone(),
                position: crate::types::MediaPosition::Background,
                alt: None,
            });
        }
        
        // Add character sprites
        for char_pos in &scene.characters {
            let position = match &char_pos.position {
                crate::script::Position::Left => crate::types::MediaPosition::Left,
                crate::script::Position::Center => crate::types::MediaPosition::Center,
                crate::script::Position::Right => crate::types::MediaPosition::Right,
                crate::script::Position::CenterLeft => crate::types::MediaPosition::Left,
                crate::script::Position::CenterRight => crate::types::MediaPosition::Right,
                crate::script::Position::Custom(x, y) => crate::types::MediaPosition::Custom { x: *x, y: *y },
            };
            media.push(crate::types::Media {
                media_type: crate::types::MediaType::Character,
                source: format!("{}:{}", char_pos.id, char_pos.sprite),
                position,
                alt: Some(char_pos.id.clone()),
            });
        }
        
        Ok(Response {
            text,
            location: Some(current_scene),
            choices,
            state: self.get_game_state(),
            media,
            sounds: scene.music.clone().map(|m| vec![crate::types::Sound {
                source: m,
                volume: 1.0,
                looping: true,
                fade_in: Some(1000),
            }]).unwrap_or_default(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Advance VN dialogue
    fn advance_vn_dialogue(&mut self) -> Result<Response> {
        let dialogue_idx = self.world.get_variable("vn_dialogue_index")
            .and_then(|v| if let Value::Float(n) = v { Some(*n as usize) } else { None })
            .unwrap_or(0);
        
        // Increment dialogue index
        self.world.set_variable("vn_dialogue_index".to_string(), Value::Float((dialogue_idx + 1) as f64));
        
        // Check if we need to change scenes
        let current_scene = self.world.get_variable("vn_current_scene")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current scene".to_string()))?;
        
        let scenes_json = self.world.get_variable("vn_scenes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No scenes data".to_string()))?;
        
        let scenes: HashMap<String, crate::script::Scene> = serde_json::from_str(&scenes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse scenes: {}", e)))?;
        
        if let Some(scene) = scenes.get(&current_scene) {
            if dialogue_idx + 1 >= scene.dialogue.len() {
                // Try to find next scene (simple progression)
                // In a full implementation, this would follow the story structure
                self.world.set_variable("vn_dialogue_index".to_string(), Value::Float(0.0));
            }
        }
        
        self.get_vn_scene()
    }
    
    /// Go to previous VN dialogue
    fn previous_vn_dialogue(&mut self) -> Result<Response> {
        let dialogue_idx = self.world.get_variable("vn_dialogue_index")
            .and_then(|v| if let Value::Float(n) = v { Some(*n as usize) } else { None })
            .unwrap_or(0);
        
        if dialogue_idx > 0 {
            self.world.set_variable("vn_dialogue_index".to_string(), Value::Float((dialogue_idx - 1) as f64));
        }
        
        self.get_vn_scene()
    }
    
    /// Select a VN choice
    fn select_vn_choice(&mut self, choice_idx: usize) -> Result<Response> {
        let current_scene = self.world.get_variable("vn_current_scene")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current scene".to_string()))?;
        
        let dialogue_idx = self.world.get_variable("vn_dialogue_index")
            .and_then(|v| if let Value::Float(n) = v { Some(*n as usize) } else { None })
            .unwrap_or(0);
        
        let scenes_json = self.world.get_variable("vn_scenes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No scenes data".to_string()))?;
        
        let scenes: HashMap<String, crate::script::Scene> = serde_json::from_str(&scenes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse scenes: {}", e)))?;
        
        let scene = scenes.get(&current_scene)
            .ok_or_else(|| Error::NotFound(format!("Scene '{}'", current_scene)))?;
        
        if dialogue_idx < scene.dialogue.len() {
            if let Some(choices) = &scene.dialogue[dialogue_idx].choices {
                if choice_idx < choices.len() {
                    let choice = &choices[choice_idx];
                    // Navigate to target scene
                    self.world.set_variable("vn_current_scene".to_string(), Value::String(choice.target.clone()));
                    self.world.set_variable("vn_dialogue_index".to_string(), Value::Float(0.0));
                    
                    // Apply any consequences
                    // In a full implementation, this would process conditions and actions
                    
                    return self.get_vn_scene();
                }
            }
        }
        
        Err(Error::InvalidInput(format!("Invalid choice index: {}", choice_idx)))
    }
    
    /// Toggle VN auto mode
    fn toggle_vn_auto_mode(&mut self) -> Result<Response> {
        let auto_mode = self.world.get_variable("vn_auto_mode")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false);
        
        self.world.set_variable("vn_auto_mode".to_string(), Value::Bool(!auto_mode));
        
        Ok(Response {
            text: format!("Auto mode: {}", if !auto_mode { "ON" } else { "OFF" }),
            location: self.world.get_variable("vn_current_scene")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None }),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    /// Toggle VN skip mode
    fn toggle_vn_skip_mode(&mut self) -> Result<Response> {
        let skip_mode = self.world.get_variable("vn_skip_mode")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false);
        
        self.world.set_variable("vn_skip_mode".to_string(), Value::Bool(!skip_mode));
        
        Ok(Response {
            text: format!("Skip mode: {}", if !skip_mode { "ON" } else { "OFF" }),
            location: self.world.get_variable("vn_current_scene")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None }),
            choices: Vec::new(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
    
    // ============================================================================
    // INTERACTIVE FICTION MODE IMPLEMENTATION
    // ============================================================================
    
    /// Get current IF node
    fn get_if_node(&mut self) -> Result<Response> {
        let current_node = self.world.get_variable("if_current_node")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current node".to_string()))?;
        
        // Check for available storylets first
        if let Some(storylet) = self.get_available_storylet()? {
            return self.display_storylet(storylet);
        }
        
        // Get nodes
        let nodes_json = self.world.get_variable("if_nodes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No nodes data".to_string()))?;
        
        let nodes: HashMap<String, crate::script::StoryNode> = serde_json::from_str(&nodes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse nodes: {}", e)))?;
        
        let node = nodes.get(&current_node)
            .ok_or_else(|| Error::NotFound(format!("Node '{}'", current_node)))?;
        
        // Apply consequences
        if let Some(consequences) = &node.consequences {
            self.apply_if_actions(consequences)?;
        }
        
        // Build text with stats if enabled
        let mut text = String::new();
        
        if self.world.get_variable("if_show_stats")
            .and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false) {
            text.push_str(&self.format_if_stats());
            text.push_str("\n---\n\n");
        }
        
        text.push_str(&node.content);
        
        // Filter available choices based on conditions
        let mut available_choices = Vec::new();
        for choice in &node.choices {
            if self.check_if_conditions(&choice.conditions)? {
                available_choices.push(choice.text.clone());
            }
        }
        
        Ok(Response {
            text,
            location: Some(current_node),
            choices: available_choices.into_iter().enumerate().map(|(i, text)| crate::types::Choice {
                id: format!("choice_{}", i),
                text,
                enabled: true,
                hint: None,
            }).collect(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: node.choices.is_empty(), // End if no choices
        })
    }
    
    /// Select an IF choice by index
    fn select_if_choice(&mut self, choice_idx: usize) -> Result<Response> {
        let current_node = self.world.get_variable("if_current_node")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current node".to_string()))?;
        
        let nodes_json = self.world.get_variable("if_nodes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No nodes data".to_string()))?;
        
        let nodes: HashMap<String, crate::script::StoryNode> = serde_json::from_str(&nodes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse nodes: {}", e)))?;
        
        let node = nodes.get(&current_node)
            .ok_or_else(|| Error::NotFound(format!("Node '{}'", current_node)))?;
        
        // Find the actual choice (accounting for filtered choices)
        let mut available_idx = 0;
        for choice in &node.choices {
            if self.check_if_conditions(&choice.conditions)? {
                if available_idx == choice_idx {
                    // Apply consequences
                    if let Some(consequences) = &choice.consequences {
                        self.apply_if_actions(consequences)?;
                    }
                    
                    // Navigate to target node
                    self.world.set_variable("if_current_node".to_string(), Value::String(choice.target.clone()));
                    self.world.turn_count += 1;
                    
                    return self.get_if_node();
                }
                available_idx += 1;
            }
        }
        
        Err(Error::InvalidInput(format!("Invalid choice index: {}", choice_idx)))
    }
    
    /// Select IF choice by matching text
    fn select_if_choice_by_text(&mut self, choice_text: &str) -> Result<Response> {
        let current_node = self.world.get_variable("if_current_node")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No current node".to_string()))?;
        
        let nodes_json = self.world.get_variable("if_nodes")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .ok_or_else(|| Error::InvalidState("No nodes data".to_string()))?;
        
        let nodes: HashMap<String, crate::script::StoryNode> = serde_json::from_str(&nodes_json)
            .map_err(|e| Error::RuntimeError(format!("Failed to parse nodes: {}", e)))?;
        
        let node = nodes.get(&current_node)
            .ok_or_else(|| Error::NotFound(format!("Node '{}'", current_node)))?;
        
        // Find matching choice
        for choice in &node.choices {
            if choice.text.to_lowercase().contains(&choice_text.to_lowercase()) {
                if self.check_if_conditions(&choice.conditions)? {
                    // Apply consequences
                    if let Some(consequences) = &choice.consequences {
                        self.apply_if_actions(consequences)?;
                    }
                    
                    // Navigate to target node
                    self.world.set_variable("if_current_node".to_string(), Value::String(choice.target.clone()));
                    self.world.turn_count += 1;
                    
                    return self.get_if_node();
                }
            }
        }
        
        Err(Error::InvalidInput(format!("No matching choice for: {}", choice_text)))
    }
    
    /// Check IF conditions
    fn check_if_conditions(&self, conditions: &Option<Vec<crate::script::Condition>>) -> Result<bool> {
        if let Some(conds) = conditions {
            for cond in conds {
                match cond {
                    crate::script::Condition::QualityAtLeast(qual, value) => {
                        let qual_value = self.world.get_variable(&format!("quality_{}", qual))
                            .and_then(|v| if let Value::Float(n) = v { Some(*n) } else { None })
                            .unwrap_or(0.0);
                        if qual_value < *value as f64 {
                            return Ok(false);
                        }
                    }
                    crate::script::Condition::QualityAtMost(qual, value) => {
                        let qual_value = self.world.get_variable(&format!("quality_{}", qual))
                            .and_then(|v| if let Value::Float(n) = v { Some(*n) } else { None })
                            .unwrap_or(0.0);
                        if qual_value > *value as f64 {
                            return Ok(false);
                        }
                    }
                    crate::script::Condition::QualityBetween(qual, min, max) => {
                        let qual_value = self.world.get_variable(&format!("quality_{}", qual))
                            .and_then(|v| if let Value::Float(n) = v { Some(*n) } else { None })
                            .unwrap_or(0.0);
                        if qual_value < *min as f64 || qual_value > *max as f64 {
                            return Ok(false);
                        }
                    }
                    crate::script::Condition::HasFlag(flag) => {
                        if !self.world.flags.contains(flag) {
                            return Ok(false);
                        }
                    }
                    crate::script::Condition::NotHasFlag(flag) => {
                        if self.world.flags.contains(flag) {
                            return Ok(false);
                        }
                    }
                    _ => {
                        // For conditions we don't handle in IF mode, default to true
                        // This includes VarEquals, VarNotEquals, HasItem, InLocation, HasVisited, Not, And, Or
                    }
                }
            }
        }
        Ok(true)
    }
    
    /// Apply IF actions
    fn apply_if_actions(&mut self, actions: &[crate::script::Action]) -> Result<()> {
        for action in actions {
            match action {
                crate::script::Action::ChangeQuality(qual, amount) => {
                    let qual_name = format!("quality_{}", qual);
                    let current = self.world.get_variable(&qual_name)
                        .and_then(|v| if let Value::Float(n) = v { Some(*n) } else { None })
                        .unwrap_or(0.0);
                    
                    let mut new_value = current + *amount as f64;
                    
                    // Apply caps if they exist
                    if let Some(Value::Float(min)) = self.world.get_variable(&format!("quality_{}_min", qual)) {
                        new_value = new_value.max(*min);
                    }
                    if let Some(Value::Float(max)) = self.world.get_variable(&format!("quality_{}_max", qual)) {
                        new_value = new_value.min(*max);
                    }
                    
                    self.world.set_variable(qual_name, Value::Float(new_value));
                }
                crate::script::Action::SetQuality(qual, value) => {
                    let qual_name = format!("quality_{}", qual);
                    let mut new_value = *value as f64;
                    
                    // Apply caps if they exist
                    if let Some(Value::Float(min)) = self.world.get_variable(&format!("quality_{}_min", qual)) {
                        new_value = new_value.max(*min);
                    }
                    if let Some(Value::Float(max)) = self.world.get_variable(&format!("quality_{}_max", qual)) {
                        new_value = new_value.min(*max);
                    }
                    
                    self.world.set_variable(qual_name, Value::Float(new_value));
                }
                crate::script::Action::SetFlag(flag) => {
                    self.world.flags.insert(flag.clone());
                }
                crate::script::Action::UnsetFlag(flag) => {
                    self.world.flags.remove(flag);
                }
                crate::script::Action::GoToNode(node) => {
                    self.world.set_variable("if_current_node".to_string(), Value::String(node.clone()));
                }
                _ => {
                    // For actions we don't handle in IF mode, skip
                    // This includes Print, SetVar, AddVar, GiveItem, RemoveItem, MoveItem, MovePlayer, SetLocked, SetExit, EndGame, PlaySound, ShowImage
                }
            }
        }
        Ok(())
    }
    
    /// Format IF stats display
    fn format_if_stats(&self) -> String {
        let mut stats = String::new();
        stats.push_str("**Stats:**\n");
        
        // Display all qualities
        for (key, value) in &self.world.variables {
            if key.starts_with("quality_") && !key.ends_with("_min") && !key.ends_with("_max") {
                let qual_name = key.trim_start_matches("quality_");
                if let Value::Float(n) = value {
                    stats.push_str(&format!("- {}: {}\n", qual_name, n));
                }
            }
        }
        
        stats
    }
    
    /// Get available storylet
    fn get_available_storylet(&self) -> Result<Option<crate::script::Storylet>> {
        if let Some(Value::String(storylets_json)) = self.world.get_variable("if_storylets") {
            let storylets: Vec<crate::script::Storylet> = serde_json::from_str(storylets_json)
                .map_err(|e| Error::RuntimeError(format!("Failed to parse storylets: {}", e)))?;
            
            // Find highest priority available storylet
            let mut best_storylet = None;
            let mut best_priority = i32::MIN;
            
            for storylet in storylets {
                if self.check_if_conditions(&Some(storylet.conditions.clone()))? {
                    if storylet.priority > best_priority {
                        best_priority = storylet.priority;
                        best_storylet = Some(storylet);
                    }
                }
            }
            
            Ok(best_storylet)
        } else {
            Ok(None)
        }
    }
    
    /// Display a storylet
    fn display_storylet(&mut self, storylet: crate::script::Storylet) -> Result<Response> {
        // Apply storylet's consequences
        if let Some(consequences) = &storylet.content.consequences {
            self.apply_if_actions(consequences)?;
        }
        
        let mut text = String::new();
        text.push_str(&format!("**{}**\n\n", storylet.title));
        text.push_str(&storylet.content.content);
        
        // Filter available choices
        let mut available_choices = Vec::new();
        for choice in &storylet.content.choices {
            if self.check_if_conditions(&choice.conditions)? {
                available_choices.push(choice.text.clone());
            }
        }
        
        Ok(Response {
            text,
            location: Some(format!("storylet:{}", storylet.id)),
            choices: available_choices.into_iter().enumerate().map(|(i, text)| crate::types::Choice {
                id: format!("choice_{}", i),
                text,
                enabled: true,
                hint: None,
            }).collect(),
            state: self.get_game_state(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        })
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}