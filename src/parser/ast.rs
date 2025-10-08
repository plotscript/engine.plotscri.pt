//! Abstract Syntax Tree definitions for PlotScript

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{Direction, GameMode, Value};

/// Complete parsed script
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Script {
    /// Game metadata
    pub game: Option<GameDefinition>,
    /// All rooms/locations
    pub rooms: HashMap<String, Room>,
    /// All characters
    pub characters: HashMap<String, Character>,
    /// All items
    pub items: HashMap<String, Item>,
    /// All events
    pub events: Vec<Event>,
    /// All functions
    pub functions: HashMap<String, Function>,
    /// Imported files
    pub imports: Vec<String>,
}

/// Game definition and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDefinition {
    /// Game title
    pub title: String,
    /// Author name
    pub author: Option<String>,
    /// Version string
    pub version: Option<String>,
    /// Game mode
    pub mode: GameMode,
    /// Description
    pub description: Option<String>,
    /// Configuration options
    pub config: HashMap<String, Value>,
}

impl Default for GameDefinition {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: None,
            version: None,
            mode: GameMode::default(),
            description: None,
            config: HashMap::new(),
        }
    }
}

/// Room/Location definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    /// Unique identifier
    pub id: String,
    /// Display title
    pub title: Option<String>,
    /// Description text
    pub description: Option<String>,
    /// Available exits
    pub exits: HashMap<Direction, String>,
    /// Items in this room
    pub items: Vec<String>,
    /// Characters in this room
    pub characters: Vec<String>,
    /// Is the room dark?
    pub dark: bool,
    /// Has been visited?
    pub visited: bool,
    /// Scripts to run on enter
    pub on_enter: Vec<Statement>,
    /// Scripts to run on exit
    pub on_exit: Vec<Statement>,
    /// Custom properties
    pub properties: HashMap<String, Value>,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: None,
            description: None,
            exits: HashMap::new(),
            items: Vec::new(),
            characters: Vec::new(),
            dark: false,
            visited: false,
            on_enter: Vec::new(),
            on_exit: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

/// Character definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Current location
    pub location: Option<String>,
    /// Dialogue tree
    pub dialogue: DialogueTree,
    /// Current state
    pub state: String,
    /// Relationship value
    pub relationship: i32,
    /// Custom properties
    pub properties: HashMap<String, Value>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: None,
            description: None,
            location: None,
            dialogue: DialogueTree::default(),
            state: "default".to_string(),
            relationship: 0,
            properties: HashMap::new(),
        }
    }
}

/// Dialogue tree structure
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    /// Root dialogue nodes
    pub nodes: Vec<DialogueNode>,
}

/// Single dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    /// Node ID
    pub id: String,
    /// Speaker (character ID or "narrator")
    pub speaker: String,
    /// Dialogue text
    pub text: String,
    /// Possible responses
    pub responses: Vec<DialogueResponse>,
    /// Conditions to show this node
    pub conditions: Vec<Condition>,
}

/// Dialogue response option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    /// Response text
    pub text: String,
    /// Next node ID
    pub next: Option<String>,
    /// Actions to execute
    pub actions: Vec<Statement>,
    /// Conditions to show this response
    pub conditions: Vec<Condition>,
}

/// Item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Current location (room ID or character ID)
    pub location: Option<String>,
    /// Can be taken?
    pub takeable: bool,
    /// Weight (affects inventory)
    pub weight: f32,
    /// Is it a container?
    pub is_container: bool,
    /// Items contained (if container)
    pub contains: Vec<String>,
    /// Scripts for interactions
    pub on_take: Vec<Statement>,
    pub on_use: Vec<Statement>,
    pub on_examine: Vec<Statement>,
    /// Custom properties
    pub properties: HashMap<String, Value>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: None,
            description: None,
            location: None,
            takeable: true,
            weight: 1.0,
            is_container: false,
            contains: Vec::new(),
            on_take: Vec::new(),
            on_use: Vec::new(),
            on_examine: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

/// Event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event ID
    pub id: String,
    /// Trigger condition
    pub trigger: Trigger,
    /// Actions to execute
    pub actions: Vec<Statement>,
}

/// Event trigger types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    /// Always triggers
    Always,
    /// After N turns
    Turn(u32),
    /// When entering room
    EnterRoom(String),
    /// When taking item
    TakeItem(String),
    /// When variable condition met
    Condition(Condition),
    /// Custom trigger
    Custom(String),
}

/// Condition for branching logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Left side of comparison
    pub left: Expression,
    /// Operator
    pub operator: ComparisonOp,
    /// Right side of comparison
    pub right: Expression,
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Parameters
    pub parameters: Vec<String>,
    /// Function body
    pub body: Vec<Statement>,
}

/// Statement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// Variable assignment
    Assignment {
        /// Variable name
        variable: String,
        /// Value to assign
        value: Expression,
    },
    /// If statement
    If {
        /// Condition
        condition: Condition,
        /// Then block
        then_block: Vec<Statement>,
        /// Else block
        else_block: Option<Vec<Statement>>,
    },
    /// While loop
    While {
        /// Loop condition
        condition: Condition,
        /// Loop body
        body: Vec<Statement>,
    },
    /// For loop
    For {
        /// Iterator variable
        variable: String,
        /// Collection to iterate
        collection: Expression,
        /// Loop body
        body: Vec<Statement>,
    },
    /// Function call
    Call {
        /// Function name
        function: String,
        /// Arguments
        arguments: Vec<Expression>,
    },
    /// Print statement
    Print(Expression),
    /// Return statement
    Return(Option<Expression>),
    /// Game command
    Command(Command),
}

/// Game-specific commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Move object to location
    Move { object: String, location: String },
    /// Give item to character
    Give { item: String, character: String },
    /// Unlock something
    Unlock(String),
    /// Lock something
    Lock(String),
    /// Reveal hidden object
    Reveal(String),
    /// Hide object
    Hide(String),
    /// Play sound effect
    PlaySound(String),
    /// Show image
    ShowImage { path: String, position: String },
    /// Set variable
    SetVariable { name: String, value: Value },
}

/// Expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal(Value),
    /// Variable reference
    Variable(String),
    /// Binary operation
    Binary {
        /// Left operand
        left: Box<Expression>,
        /// Operator
        op: BinaryOp,
        /// Right operand
        right: Box<Expression>,
    },
    /// Unary operation
    Unary {
        /// Operator
        op: UnaryOp,
        /// Operand
        operand: Box<Expression>,
    },
    /// Function call
    Call {
        /// Function name
        function: String,
        /// Arguments
        arguments: Vec<Expression>,
    },
    /// List literal
    List(Vec<Expression>),
    /// Map literal
    Map(Vec<(String, Expression)>),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
    /// Logical AND
    And,
    /// Logical OR
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// Logical NOT
    Not,
    /// Negation
    Neg,
}