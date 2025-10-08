//! Script format definitions and parsing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{Error, Result};
use crate::types::{GameMode, Direction, Value};

/// Main game script structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameScript {
    /// Text adventure game script
    TextAdventure(TextAdventureScript),
    /// Visual novel game script
    VisualNovel(VisualNovelScript),
    /// Interactive fiction game script
    InteractiveFiction(InteractiveFictionScript),
}

/// Text adventure game script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAdventureScript {
    /// Game metadata
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub version: Option<String>,
    
    /// Game settings
    pub settings: TextAdventureSettings,
    
    /// Starting location
    pub starting_location: String,
    
    /// All locations in the game
    pub locations: HashMap<String, Location>,
    
    /// All items in the game
    pub items: HashMap<String, Item>,
    
    /// All characters in the game
    pub characters: HashMap<String, Character>,
    
    /// Vocabulary definitions
    pub vocabulary: Option<Vocabulary>,
    
    /// Global events
    pub events: Option<HashMap<String, Event>>,
}

/// Text adventure settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAdventureSettings {
    /// Parser mode (Natural, Simple, Strict)
    pub parser_mode: ParserMode,
    /// Enable command aliases
    pub command_aliases: bool,
    /// Enable darkness system
    pub darkness_system: bool,
    /// Enable inventory limits
    pub inventory_limits: bool,
    /// Max inventory size
    pub max_inventory: Option<usize>,
}

/// Parser modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ParserMode {
    /// Natural language parsing with articles
    Natural,
    /// Simple verb-noun parsing
    Simple,
    /// Strict exact matching
    Strict,
}

/// Location definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Exits to other locations
    pub exits: HashMap<Direction, String>,
    /// Items in this location
    pub items: Vec<String>,
    /// Characters in this location
    pub characters: Vec<String>,
    /// Is location dark?
    pub dark: Option<bool>,
    /// First visit description
    pub first_visit: Option<String>,
    /// Events for this location
    pub events: Option<LocationEvents>,
}

/// Location events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEvents {
    /// On enter event
    pub on_enter: Option<Vec<Action>>,
    /// On exit event
    pub on_exit: Option<Vec<Action>>,
    /// On look event
    pub on_look: Option<Vec<Action>>,
}

/// Item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Can be taken?
    pub takeable: bool,
    /// Weight (if inventory limits enabled)
    pub weight: Option<u32>,
    /// Is a container?
    pub container: Option<bool>,
    /// Items inside (if container)
    pub contains: Option<Vec<String>>,
    /// Can be opened?
    pub openable: Option<bool>,
    /// Is locked?
    pub locked: Option<bool>,
    /// Key required (if locked)
    pub key: Option<String>,
    /// Item events
    pub events: Option<ItemEvents>,
}

/// Item events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEvents {
    /// On take event
    pub on_take: Option<Vec<Action>>,
    /// On use event
    pub on_use: Option<Vec<Action>>,
    /// On examine event
    pub on_examine: Option<Vec<Action>>,
    /// On open event
    pub on_open: Option<Vec<Action>>,
}

/// Character definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Dialogue tree
    pub dialogue: Option<DialogueTree>,
    /// Character inventory
    pub inventory: Option<Vec<String>>,
    /// Character events
    pub events: Option<CharacterEvents>,
}

/// Character events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEvents {
    /// On talk event
    pub on_talk: Option<Vec<Action>>,
    /// On give event
    pub on_give: Option<Vec<Action>>,
    /// On examine event
    pub on_examine: Option<Vec<Action>>,
}

/// Dialogue tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    /// Starting node
    pub start: String,
    /// Dialogue nodes
    pub nodes: HashMap<String, DialogueNode>,
}

/// Dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    /// Character text
    pub text: String,
    /// Player responses
    pub responses: Option<Vec<DialogueResponse>>,
    /// Actions to execute
    pub actions: Option<Vec<Action>>,
    /// Next node (if no responses)
    pub next: Option<String>,
}

/// Dialogue response option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    /// Response text
    pub text: String,
    /// Next dialogue node
    pub next: String,
    /// Conditions for showing
    pub conditions: Option<Vec<Condition>>,
    /// Actions when selected
    pub actions: Option<Vec<Action>>,
}

/// Vocabulary definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    /// Recognized verbs
    pub verbs: Vec<String>,
    /// Verb synonyms
    pub synonyms: HashMap<String, String>,
    /// Custom patterns
    pub patterns: Option<Vec<Pattern>>,
}

/// Custom command pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern name
    pub name: String,
    /// Regex pattern
    pub pattern: String,
    /// Actions to execute
    pub actions: Vec<Action>,
}

/// Event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event trigger
    pub trigger: Trigger,
    /// Conditions
    pub conditions: Option<Vec<Condition>>,
    /// Actions
    pub actions: Vec<Action>,
    /// One-time only?
    pub once: Option<bool>,
}

/// Event triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    /// After N turns
    TurnCount(u32),
    /// When entering location
    EnterLocation(String),
    /// When taking item
    TakeItem(String),
    /// When talking to character
    TalkTo(String),
    /// When variable equals value
    Variable(String, Value),
    /// Custom trigger
    Custom(String),
}

/// Conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Variable equals value
    VarEquals(String, Value),
    /// Variable not equals
    VarNotEquals(String, Value),
    /// Has item
    HasItem(String),
    /// In location
    InLocation(String),
    /// Has visited location
    HasVisited(String),
    /// Has flag
    HasFlag(String),
    /// Not has flag
    NotHasFlag(String),
    /// Quality at least
    QualityAtLeast(String, i32),
    /// Quality at most
    QualityAtMost(String, i32),
    /// Quality between min and max
    QualityBetween(String, i32, i32),
    /// Not condition
    Not(Box<Condition>),
    /// All conditions must be true
    And(Vec<Condition>),
    /// Any condition must be true
    Or(Vec<Condition>),
}

/// Actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Print text
    Print(String),
    /// Set variable
    SetVar(String, Value),
    /// Add to variable
    AddVar(String, i32),
    /// Set flag
    SetFlag(String),
    /// Unset flag
    UnsetFlag(String),
    /// Change quality
    ChangeQuality(String, i32),
    /// Set quality
    SetQuality(String, i32),
    /// Go to node
    GoToNode(String),
    /// Give item
    GiveItem(String),
    /// Remove item
    RemoveItem(String),
    /// Move item to location
    MoveItem(String, String),
    /// Move player to location
    MovePlayer(String),
    /// Lock/unlock
    SetLocked(String, bool),
    /// Enable/disable exit
    SetExit(String, Direction, Option<String>),
    /// End game
    EndGame(EndType),
    /// Play sound
    PlaySound(String),
    /// Show image
    ShowImage(String),
}

/// Game ending types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndType {
    /// Victory
    Victory,
    /// Death
    Death,
    /// Custom ending
    Custom(String),
}

/// Visual novel game script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNovelScript {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub settings: VisualNovelSettings,
    pub starting_scene: String,
    pub scenes: HashMap<String, Scene>,
    pub characters: HashMap<String, VNCharacter>,
    pub assets: Assets,
}

/// Visual novel settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNovelSettings {
    pub resolution: (u32, u32),
    pub text_speed: TextSpeed,
    pub auto_save: bool,
    pub skip_mode: SkipMode,
}

impl Default for VisualNovelSettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            text_speed: TextSpeed::Normal,
            auto_save: true,
            skip_mode: SkipMode::None,
        }
    }
}

/// Text display speed
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextSpeed {
    Slow,
    Normal,
    Fast,
    Instant,
}

/// Skip mode settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkipMode {
    None,
    Read,
    All,
}

/// Scene definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub background: Option<String>,
    pub music: Option<String>,
    pub characters: Vec<CharacterPosition>,
    pub dialogue: Vec<DialogueLine>,
}

/// Character on screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPosition {
    pub id: String,
    pub sprite: String,
    pub position: Position,
}

/// Screen position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    Left,
    CenterLeft,
    Center,
    CenterRight,
    Right,
    Custom(f32, f32),
}

/// Dialogue line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: Option<String>,
    pub text: String,
    pub voice: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub effects: Option<Vec<Effect>>,
}

/// Visual novel character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNCharacter {
    pub name: String,
    pub color: Option<String>,
    pub sprites: HashMap<String, String>,
}

/// Visual/audio effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Transition(Transition),
    Sound(String),
    Shake,
    Flash(String),
}

/// Transition types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Transition {
    Fade,
    Dissolve,
    SlideLeft,
    SlideRight,
}

/// Choice in visual novel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub target: String,
    pub conditions: Option<Vec<Condition>>,
}

/// Asset definitions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Assets {
    pub backgrounds: HashMap<String, String>,
    pub sprites: HashMap<String, String>,
    pub music: HashMap<String, String>,
    pub sounds: HashMap<String, String>,
    pub voices: HashMap<String, String>,
}

/// Interactive fiction game script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFictionScript {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub settings: IFSettings,
    pub starting_node: String,
    pub nodes: HashMap<String, StoryNode>,
    pub qualities: HashMap<String, Quality>,
    pub storylets: Option<Vec<Storylet>>,
}

/// Interactive fiction settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IFSettings {
    pub show_stats: bool,
    pub checkpoint_saves: bool,
    pub timed_choices: bool,
    pub quality_caps: bool,
}

/// Story node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    pub content: String,
    pub choices: Vec<StoryChoice>,
    pub conditions: Option<Vec<Condition>>,
    pub consequences: Option<Vec<Action>>,
}

/// Story choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChoice {
    pub text: String,
    pub target: String,
    pub conditions: Option<Vec<Condition>>,
    pub consequences: Option<Vec<Action>>,
}

/// Quality (stat)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quality {
    pub initial: i32,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub hidden: bool,
}

/// Storylet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storylet {
    pub id: String,
    pub title: String,
    pub conditions: Vec<Condition>,
    pub content: StoryNode,
    pub priority: i32,
    pub repeatable: bool,
}

impl GameScript {
    /// Load from RON string
    pub fn from_ron(source: &str) -> Result<Self> {
        ron::from_str(source)
            .map_err(|e| Error::ParseError(format!("RON parse error: {}", e)))
    }
    
    /// Load from YAML string
    pub fn from_yaml(source: &str) -> Result<Self> {
        serde_yaml::from_str(source)
            .map_err(|e| Error::ParseError(format!("YAML parse error: {}", e)))
    }
    
    /// Save to RON string
    pub fn to_ron(&self) -> Result<String> {
        ron::to_string(self)
            .map_err(|e| Error::InvalidScript(format!("RON serialization error: {}", e)))
    }
    
    /// Convert to YAML format
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| Error::InvalidScript(format!("YAML serialization error: {}", e)))
    }
    
    /// Convert to JSON format
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| Error::InvalidScript(format!("JSON serialization error: {}", e)))
    }
    
    /// Get game mode
    pub fn game_mode(&self) -> GameMode {
        match self {
            GameScript::TextAdventure(_) => GameMode::TextAdventure,
            GameScript::VisualNovel(_) => GameMode::VisualNovel,
            GameScript::InteractiveFiction(_) => GameMode::InteractiveFiction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_adventure_script() {
        let script = r#"
TextAdventure((
    title: "Test Adventure",
    author: "Test Author",
    description: Some("A test game"),
    version: Some("1.0.0"),
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    starting_location: "entrance",
    locations: {
        "entrance": (
            name: "Entrance Hall",
            description: "A grand entrance hall.",
            exits: {
                north: "hallway",
            },
            items: ["key"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "hallway": (
            name: "Hallway",
            description: "A long hallway.",
            exits: {
                south: "entrance",
            },
            items: [],
            characters: [],
            dark: None,
            first_visit: None,
            events: None,
        ),
    },
    items: {
        "key": (
            name: "Brass Key",
            description: "A small brass key.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
    },
    characters: {},
    vocabulary: None,
    events: None,
))
        "#;
        
        let game = GameScript::from_ron(script).unwrap();
        assert!(matches!(game, GameScript::TextAdventure(_)));
    }
}