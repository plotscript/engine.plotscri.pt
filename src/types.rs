//! Core types used throughout the PlotScript engine

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Game mode determines the type of interactive narrative
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    /// Classic text adventure with parser
    TextAdventure,
    /// Visual novel with graphics and characters
    VisualNovel,
    /// Choice-based interactive fiction
    InteractiveFiction,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::InteractiveFiction
    }
}

/// Response from the engine after processing input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Main text output
    pub text: String,
    /// Current location/scene
    pub location: Option<String>,
    /// Available choices (for IF mode)
    pub choices: Vec<Choice>,
    /// Current game state
    pub state: GameState,
    /// Media to display
    pub media: Vec<Media>,
    /// Sound effects to play
    pub sounds: Vec<Sound>,
    /// Achievement unlocked
    pub achievement: Option<String>,
    /// Whether the command was successful
    pub success: bool,
    /// Whether the game has ended
    pub ended: bool,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            text: String::new(),
            location: None,
            choices: Vec::new(),
            state: GameState::default(),
            media: Vec::new(),
            sounds: Vec::new(),
            achievement: None,
            success: true,
            ended: false,
        }
    }
}

/// A choice available to the player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Unique ID for this choice
    pub id: String,
    /// Display text
    pub text: String,
    /// Whether this choice is available
    pub enabled: bool,
    /// Tooltip or hint
    pub hint: Option<String>,
}

/// Current game state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Current mode
    pub mode: GameMode,
    /// Player stats/variables
    pub variables: HashMap<String, Value>,
    /// Inventory items
    pub inventory: Vec<String>,
    /// Current score
    pub score: i32,
    /// Turn/move counter
    pub turns: u32,
    /// Game flags
    pub flags: HashSet<String>,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self {
            mode: GameMode::default(),
            variables: HashMap::new(),
            inventory: Vec::new(),
            score: 0,
            turns: 0,
            flags: HashSet::new(),
        }
    }
    
    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Set a variable value
    pub fn set_variable(&mut self, name: impl Into<String>, value: Value) {
        self.variables.insert(name.into(), value);
    }
    
    /// Check if a flag is set
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(name)
    }
    
    /// Set a flag
    pub fn set_flag(&mut self, name: impl Into<String>) {
        self.flags.insert(name.into());
    }
    
    /// Clear a flag
    pub fn clear_flag(&mut self, name: &str) {
        self.flags.remove(name);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

/// Media element to display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    /// Type of media
    pub media_type: MediaType,
    /// Path or URL
    pub source: String,
    /// Display position
    pub position: MediaPosition,
    /// Alt text
    pub alt: Option<String>,
}

/// Type of media
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    /// Background image
    Background,
    /// Character sprite
    Character,
    /// UI element
    Interface,
    /// Cutscene/video
    Video,
}

/// Position for media display
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaPosition {
    /// Full background
    Background,
    /// Left side
    Left,
    /// Center
    Center,
    /// Right side
    Right,
    /// Custom position
    Custom { x: f32, y: f32 },
}

/// Sound effect or music
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    /// Sound file path
    pub source: String,
    /// Volume (0.0 - 1.0)
    pub volume: f32,
    /// Loop the sound
    pub looping: bool,
    /// Fade in duration (ms)
    pub fade_in: Option<u32>,
}

/// Variable value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// String value
    String(String),
    /// List of values
    List(Vec<Value>),
    /// Map of values
    Map(HashMap<String, Value>),
    /// Null value
    Null,
}

impl Value {
    /// Get as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    /// Get as integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            _ => None,
        }
    }
    
    /// Get as string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

/// Direction for movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// North
    North,
    /// Northeast
    Northeast,
    /// East
    East,
    /// Southeast
    Southeast,
    /// South
    South,
    /// Southwest
    Southwest,
    /// West
    West,
    /// Northwest
    Northwest,
    /// Up
    Up,
    /// Down
    Down,
    /// In
    In,
    /// Out
    Out,
}

impl Direction {
    /// Get opposite direction
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::Northeast => Direction::Southwest,
            Direction::East => Direction::West,
            Direction::Southeast => Direction::Northwest,
            Direction::South => Direction::North,
            Direction::Southwest => Direction::Northeast,
            Direction::West => Direction::East,
            Direction::Northwest => Direction::Southeast,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::In => Direction::Out,
            Direction::Out => Direction::In,
        }
    }
    
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Direction> {
        match s.to_lowercase().as_str() {
            "n" | "north" => Some(Direction::North),
            "ne" | "northeast" => Some(Direction::Northeast),
            "e" | "east" => Some(Direction::East),
            "se" | "southeast" => Some(Direction::Southeast),
            "s" | "south" => Some(Direction::South),
            "sw" | "southwest" => Some(Direction::Southwest),
            "w" | "west" => Some(Direction::West),
            "nw" | "northwest" => Some(Direction::Northwest),
            "u" | "up" => Some(Direction::Up),
            "d" | "down" => Some(Direction::Down),
            "in" | "enter" => Some(Direction::In),
            "out" | "exit" | "leave" => Some(Direction::Out),
            _ => None,
        }
    }
}

/// Game type (similar to GameMode but used in tests)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameType {
    /// Text adventure
    TextAdventure,
    /// Visual novel
    VisualNovel,
    /// Interactive fiction
    InteractiveFiction,
}

/// Conditions for game logic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    /// Always true
    Always,
    /// Check if player has item
    HasItem(String),
    /// Check if flag is set
    HasFlag(String),
    /// Check quality value
    QualityCheck(String, String, i32), // quality name, operator, value
    /// Check state
    StateCheck(String, Value),
    /// Check if location visited
    HasVisited(String),
    /// All conditions must be true
    And(Vec<Box<Condition>>),
    /// At least one condition must be true
    Or(Vec<Box<Condition>>),
    /// Condition must be false
    Not(Box<Condition>),
}

/// Actions that can be performed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Print text
    Print(String),
    /// Set variable
    SetVariable(String, Value),
    /// Modify quality
    ModifyQuality(String, i32),
    /// Add item to inventory
    AddItem(String),
    /// Remove item from inventory
    RemoveItem(String),
    /// Set flag
    SetFlag(String),
    /// Clear flag
    ClearFlag(String),
    /// Go to location/node
    GoTo(String),
    /// Unlock node
    UnlockNode(String),
    /// Play sound
    PlaySound(String),
    /// Show image
    ShowImage(String),
    /// Start combat
    StartCombat(String),
    /// Add score
    AddScore(i32),
    /// Screen shake
    ScreenShake(f32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::Up.opposite(), Direction::Down);
    }
    
    #[test]
    fn test_direction_from_str() {
        assert_eq!(Direction::from_str("n"), Some(Direction::North));
        assert_eq!(Direction::from_str("NORTH"), Some(Direction::North));
        assert_eq!(Direction::from_str("invalid"), None);
    }
    
    #[test]
    fn test_value_conversions() {
        let val = Value::Integer(42);
        assert_eq!(val.as_integer(), Some(42));
        assert_eq!(val.as_bool(), None);
        
        let val = Value::String("hello".to_string());
        assert_eq!(val.as_string(), Some("hello"));
    }
}