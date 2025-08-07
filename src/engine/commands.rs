//! Command parsing for text adventure mode

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::Result;
use crate::parser::{FuzzyCommandMatcher, VocabularyBuilder};
use crate::types::Direction;
use crate::world::World;

/// Parsed command types
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Look around
    Look,
    /// Look at something specific
    LookAt(String),
    /// Move in a direction
    Go(Direction),
    /// Take an item
    Take(String),
    /// Drop an item
    Drop(String),
    /// Use an item
    Use(String),
    /// Use item on target
    UseOn(String, String),
    /// Examine something
    Examine(String),
    /// Show inventory
    Inventory,
    /// Talk to character
    Talk(String),
    /// Say something
    Say(String),
    /// Give item to character
    Give(String, String),
    /// Open something
    Open(String),
    /// Close something
    Close(String),
    /// Unlock something
    Unlock(String),
    /// Lock something
    Lock(String),
    /// Push/pull/turn
    Manipulate(String, String),
    /// Wait
    Wait,
    /// Save game
    Save(Option<u8>),
    /// Load game
    Load(Option<u8>),
    /// Help
    Help,
    /// Quit
    Quit,
    /// Debug command
    Debug(String),
    /// Unknown command
    Unknown(String),
}

/// Command parser for natural language input
pub struct CommandParser {
    fuzzy_matcher: FuzzyCommandMatcher,
}

impl CommandParser {
    /// Create a new command parser
    pub fn new() -> Self {
        // Build vocabulary with default settings
        let matcher = VocabularyBuilder::new().build(70);
        
        Self {
            fuzzy_matcher: matcher,
        }
    }
    
    /// Create with custom threshold
    pub fn with_threshold(threshold: u8) -> Self {
        let matcher = VocabularyBuilder::new().build(threshold);
        
        Self {
            fuzzy_matcher: matcher,
        }
    }
    
    /// Parse a command string
    pub fn parse(&self, input: &str, world: &World) -> Result<Command> {
        let input = input.trim().to_lowercase();
        
        if input.is_empty() {
            return Ok(Command::Look);
        }
        
        // Try regex patterns
        if let Some(cmd) = self.parse_patterns(&input) {
            return Ok(cmd);
        }
        
        // Try simple commands
        if let Some(cmd) = self.parse_simple(&input) {
            return Ok(cmd);
        }
        
        // Try fuzzy matching
        if let Some(cmd) = self.parse_fuzzy(&input, world) {
            return Ok(cmd);
        }
        
        Ok(Command::Unknown(input))
    }
    
    /// Parse using regex patterns
    fn parse_patterns(&self, input: &str) -> Option<Command> {
        lazy_static! {
            // Movement patterns
            static ref GO_PATTERN: Regex = Regex::new(r"^(go|walk|move|head|travel)\s+(to\s+)?(north|south|east|west|up|down|in|out|n|s|e|w|u|d)$").unwrap();
            static ref DIRECTION_ONLY: Regex = Regex::new(r"^(north|south|east|west|up|down|in|out|n|s|e|w|u|d)$").unwrap();
            
            // Object interaction patterns
            static ref TAKE_PATTERN: Regex = Regex::new(r"^(take|get|pick up|grab|acquire)\s+(?:the\s+)?(.+)$").unwrap();
            static ref DROP_PATTERN: Regex = Regex::new(r"^(drop|put down|discard|leave)\s+(?:the\s+)?(.+)$").unwrap();
            static ref USE_PATTERN: Regex = Regex::new(r"^(use|apply|activate)\s+(?:the\s+)?(.+?)(?:\s+(?:on|with)\s+(?:the\s+)?(.+))?$").unwrap();
            static ref EXAMINE_PATTERN: Regex = Regex::new(r"^(examine|look at|inspect|check|x)\s+(?:the\s+)?(.+)$").unwrap();
            
            // Character interaction patterns
            static ref TALK_PATTERN: Regex = Regex::new(r"^(talk|speak|chat)\s+(?:to|with)\s+(?:the\s+)?(.+)$").unwrap();
            static ref SAY_PATTERN: Regex = Regex::new(r#"^(say|tell|ask)\s+["']?(.+?)["']?$"#).unwrap();
            static ref GIVE_PATTERN: Regex = Regex::new(r#"^(give|offer|hand)\s+(?:the\s+)?(.+?)\s+to\s+(?:the\s+)?(.+)$"#).unwrap();
            
            // Container patterns
            static ref OPEN_PATTERN: Regex = Regex::new(r#"^(open|unlock)\s+(?:the\s+)?(.+)$"#).unwrap();
            static ref CLOSE_PATTERN: Regex = Regex::new(r#"^(close|shut|lock)\s+(?:the\s+)?(.+)$"#).unwrap();
            
            // Save/load patterns
            static ref SAVE_PATTERN: Regex = Regex::new(r#"^save(?:\s+(?:game\s+)?(?:to\s+)?(?:slot\s+)?(\d+))?$"#).unwrap();
            static ref LOAD_PATTERN: Regex = Regex::new(r#"^load(?:\s+(?:game\s+)?(?:from\s+)?(?:slot\s+)?(\d+))?$"#).unwrap();
        }
        
        // Check movement
        if let Some(caps) = GO_PATTERN.captures(input) {
            if let Some(dir_match) = caps.get(3) {
                if let Some(dir) = Direction::from_str(dir_match.as_str()) {
                    return Some(Command::Go(dir));
                }
            }
        }
        
        if let Some(caps) = DIRECTION_ONLY.captures(input) {
            if let Some(dir) = Direction::from_str(caps.get(1).unwrap().as_str()) {
                return Some(Command::Go(dir));
            }
        }
        
        // Check take
        if let Some(caps) = TAKE_PATTERN.captures(input) {
            if let Some(item) = caps.get(2) {
                return Some(Command::Take(item.as_str().to_string()));
            }
        }
        
        // Check drop
        if let Some(caps) = DROP_PATTERN.captures(input) {
            if let Some(item) = caps.get(2) {
                return Some(Command::Drop(item.as_str().to_string()));
            }
        }
        
        // Check use
        if let Some(caps) = USE_PATTERN.captures(input) {
            if let Some(item) = caps.get(2) {
                if let Some(target) = caps.get(3) {
                    return Some(Command::UseOn(
                        item.as_str().to_string(),
                        target.as_str().to_string()
                    ));
                } else {
                    return Some(Command::Use(item.as_str().to_string()));
                }
            }
        }
        
        // Check examine
        if let Some(caps) = EXAMINE_PATTERN.captures(input) {
            if let Some(target) = caps.get(2) {
                return Some(Command::Examine(target.as_str().to_string()));
            }
        }
        
        // Check talk
        if let Some(caps) = TALK_PATTERN.captures(input) {
            if let Some(character) = caps.get(2) {
                return Some(Command::Talk(character.as_str().to_string()));
            }
        }
        
        // Check give
        if let Some(caps) = GIVE_PATTERN.captures(input) {
            if let (Some(item), Some(character)) = (caps.get(2), caps.get(3)) {
                return Some(Command::Give(
                    item.as_str().to_string(),
                    character.as_str().to_string()
                ));
            }
        }
        
        // Check save/load
        if let Some(caps) = SAVE_PATTERN.captures(input) {
            let slot = caps.get(1).and_then(|m| m.as_str().parse().ok());
            return Some(Command::Save(slot));
        }
        
        if let Some(caps) = LOAD_PATTERN.captures(input) {
            let slot = caps.get(1).and_then(|m| m.as_str().parse().ok());
            return Some(Command::Load(slot));
        }
        
        None
    }
    
    /// Parse simple single-word commands
    fn parse_simple(&self, input: &str) -> Option<Command> {
        match input {
            "look" | "l" => Some(Command::Look),
            "inventory" | "i" | "inv" => Some(Command::Inventory),
            "wait" | "z" => Some(Command::Wait),
            "help" | "h" | "?" => Some(Command::Help),
            "quit" | "q" | "exit" => Some(Command::Quit),
            _ => None,
        }
    }
    
    /// Parse using fuzzy matching against known objects
    fn parse_fuzzy(&self, input: &str, world: &World) -> Option<Command> {
        // Get context objects (items in room + inventory)
        let mut context_objects = Vec::new();
        
        // Add items in current room
        if let Ok(room_items) = world.room_items() {
            for item in room_items {
                if let Some(name) = &item.name {
                    context_objects.push(name.clone());
                } else {
                    context_objects.push(item.id.clone());
                }
            }
        }
        
        // Add items in inventory
        for item_id in &world.inventory {
            if let Some(item) = world.items.get(item_id) {
                if let Some(name) = &item.name {
                    context_objects.push(name.clone());
                } else {
                    context_objects.push(item.id.clone());
                }
            }
        }
        
        // Add characters in room
        if let Ok(room_chars) = world.room_characters() {
            for character in room_chars {
                if let Some(name) = &character.name {
                    context_objects.push(name.clone());
                } else {
                    context_objects.push(character.id.clone());
                }
            }
        }
        
        // Try to parse the command with fuzzy matching
        if let Some((verb, object)) = self.fuzzy_matcher.parse_command(input, &context_objects) {
            // Handle multi-word verbs (like "go north")
            if verb.starts_with("go ") {
                if let Some(dir_str) = verb.strip_prefix("go ") {
                    if let Some(dir) = Direction::from_str(dir_str) {
                        return Some(Command::Go(dir));
                    }
                }
            }
            
            // Handle regular verbs
            match verb.as_str() {
                "look" => {
                    if let Some(obj) = object {
                        Some(Command::Examine(obj))
                    } else {
                        Some(Command::Look)
                    }
                }
                "take" | "get" | "pick" | "grab" => {
                    object.map(Command::Take)
                }
                "drop" | "put" | "leave" => {
                    object.map(Command::Drop)
                }
                "use" | "activate" | "apply" => {
                    object.map(Command::Use)
                }
                "examine" | "inspect" | "check" => {
                    object.map(Command::Examine)
                }
                "talk" | "speak" | "chat" => {
                    object.map(Command::Talk)
                }
                "open" | "unlock" => {
                    object.map(Command::Open)
                }
                "close" | "shut" | "lock" => {
                    object.map(Command::Close)
                }
                "inventory" => Some(Command::Inventory),
                "help" => Some(Command::Help),
                "quit" | "exit" => Some(Command::Quit),
                _ => None
            }
        } else {
            None
        }
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_movement_commands() {
        let parser = CommandParser::new();
        let world = World::new();
        
        assert_eq!(
            parser.parse("go north", &world).unwrap(),
            Command::Go(Direction::North)
        );
        
        assert_eq!(
            parser.parse("n", &world).unwrap(),
            Command::Go(Direction::North)
        );
        
        assert_eq!(
            parser.parse("walk to the east", &world).unwrap(),
            Command::Go(Direction::East)
        );
    }
    
    #[test]
    fn test_object_commands() {
        let parser = CommandParser::new();
        let world = World::new();
        
        assert_eq!(
            parser.parse("take the brass key", &world).unwrap(),
            Command::Take("brass key".to_string())
        );
        
        assert_eq!(
            parser.parse("examine lamp", &world).unwrap(),
            Command::Examine("lamp".to_string())
        );
        
        assert_eq!(
            parser.parse("use key on door", &world).unwrap(),
            Command::UseOn("key".to_string(), "door".to_string())
        );
    }
    
    #[test]
    fn test_simple_commands() {
        let parser = CommandParser::new();
        let world = World::new();
        
        assert_eq!(parser.parse("look", &world).unwrap(), Command::Look);
        assert_eq!(parser.parse("i", &world).unwrap(), Command::Inventory);
        assert_eq!(parser.parse("help", &world).unwrap(), Command::Help);
        assert_eq!(parser.parse("quit", &world).unwrap(), Command::Quit);
    }
}