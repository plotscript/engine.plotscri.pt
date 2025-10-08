//! Interactive Fiction (choice-based) game format support

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::error::{Error, Result};

/// Interactive fiction game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFiction {
    /// Current story node
    pub current_node: String,
    /// Story nodes
    pub nodes: HashMap<String, StoryNode>,
    /// Storylets (quality-based content)
    pub storylets: Vec<Storylet>,
    /// Player qualities/stats
    pub qualities: HashMap<String, Quality>,
    /// Inventory items
    pub inventory: HashSet<String>,
    /// Story history
    pub history: Vec<HistoryEntry>,
    /// Checkpoints for save states
    pub checkpoints: HashMap<String, GameCheckpoint>,
}

/// Story node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub choices: Vec<Choice>,
    pub conditions: Vec<Condition>,
    pub consequences: Vec<Consequence>,
    pub node_type: NodeType,
}

/// Node types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeType {
    Normal,
    Hub,      // Can return to this node
    Ending,   // Game ending
    Death,    // Death/failure ending
    Checkpoint, // Auto-save point
}

/// Player choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: String,
    pub text: String,
    pub target: String,
    pub conditions: Vec<Condition>,
    pub consequences: Vec<Consequence>,
    pub visible_if: Vec<Condition>,
    pub style: ChoiceStyle,
}

/// Choice presentation style
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChoiceStyle {
    Normal,
    Important,  // Highlighted
    Dangerous,  // Warning style
    Subtle,     // De-emphasized
    Timed(u32), // Seconds to choose
}

/// Conditions for choices/nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Always,
    Never,
    HasQuality(String, ComparisonOp, i32),
    HasItem(String),
    HasVisited(String),
    HasFlag(String),
    Not(Box<Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Random(f32), // Probability 0.0-1.0
}

/// Comparison operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

/// Consequences of choices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Consequence {
    SetQuality(String, QualityChange),
    GiveItem(String),
    RemoveItem(String),
    SetFlag(String),
    UnsetFlag(String),
    GoTo(String),
    EndGame(String),
}

/// Quality change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityChange {
    Set(i32),
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Divide(i32),
}

/// Player quality/stat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quality {
    pub name: String,
    pub value: i32,
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub hidden: bool,
    pub category: QualityCategory,
}

/// Quality categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QualityCategory {
    Attribute,    // Core stats
    Skill,        // Learned abilities
    Resource,     // Consumable resources
    Relationship, // NPC relationships
    Progress,     // Story progress trackers
}

/// Storylet for quality-based narratives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storylet {
    pub id: String,
    pub title: String,
    pub conditions: Vec<Condition>,
    pub content: StoryNode,
    pub priority: i32,
    pub repeatable: bool,
    pub max_uses: Option<u32>,
    pub uses: u32,
}

/// History entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub node_id: String,
    pub choice_made: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Game checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCheckpoint {
    pub node_id: String,
    pub qualities: HashMap<String, Quality>,
    pub inventory: HashSet<String>,
    pub flags: HashSet<String>,
}

impl InteractiveFiction {
    /// Create a new interactive fiction game
    pub fn new() -> Self {
        Self {
            current_node: String::new(),
            nodes: HashMap::new(),
            storylets: Vec::new(),
            qualities: HashMap::new(),
            inventory: HashSet::new(),
            history: Vec::new(),
            checkpoints: HashMap::new(),
        }
    }
    
    /// Load a story node
    pub fn load_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(Error::RuntimeError(format!("Node '{}' not found", node_id)));
        }
        
        // Record in history
        self.history.push(HistoryEntry {
            node_id: node_id.to_string(),
            choice_made: None,
            timestamp: chrono::Utc::now(),
        });
        
        self.current_node = node_id.to_string();
        
        // Execute node consequences
        if let Some(node) = self.nodes.get(node_id).cloned() {
            for consequence in &node.consequences {
                self.apply_consequence(consequence)?;
            }
            
            // Auto-save at checkpoints
            if matches!(node.node_type, NodeType::Checkpoint) {
                self.create_checkpoint(node_id);
            }
        }
        
        Ok(())
    }
    
    /// Get current node
    pub fn current_node(&self) -> Option<&StoryNode> {
        self.nodes.get(&self.current_node)
    }
    
    /// Get available choices
    pub fn get_available_choices(&self) -> Vec<&Choice> {
        if let Some(node) = self.current_node() {
            node.choices.iter()
                .filter(|choice| self.check_conditions(&choice.visible_if))
                .collect()
        } else {
            vec![]
        }
    }
    
    /// Make a choice
    pub fn make_choice(&mut self, choice_id: &str) -> Result<()> {
        let node = self.current_node()
            .ok_or_else(|| Error::RuntimeError("No current node".to_string()))?
            .clone();
        
        let choice = node.choices.iter()
            .find(|c| c.id == choice_id)
            .ok_or_else(|| Error::RuntimeError("Invalid choice".to_string()))?
            .clone();
        
        // Check conditions
        if !self.check_conditions(&choice.conditions) {
            return Err(Error::RuntimeError("Choice conditions not met".to_string()));
        }
        
        // Apply consequences
        for consequence in &choice.consequences {
            self.apply_consequence(consequence)?;
        }
        
        // Record choice in history
        if let Some(last) = self.history.last_mut() {
            last.choice_made = Some(choice_id.to_string());
        }
        
        // Navigate to target
        self.load_node(&choice.target)?;
        
        Ok(())
    }
    
    /// Check if conditions are met
    pub fn check_conditions(&self, conditions: &[Condition]) -> bool {
        conditions.iter().all(|cond| self.check_condition(cond))
    }
    
    /// Check a single condition
    pub fn check_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Always => true,
            Condition::Never => false,
            Condition::HasQuality(name, op, value) => {
                if let Some(quality) = self.qualities.get(name) {
                    match op {
                        ComparisonOp::Equal => quality.value == *value,
                        ComparisonOp::NotEqual => quality.value != *value,
                        ComparisonOp::Greater => quality.value > *value,
                        ComparisonOp::GreaterEqual => quality.value >= *value,
                        ComparisonOp::Less => quality.value < *value,
                        ComparisonOp::LessEqual => quality.value <= *value,
                    }
                } else {
                    false
                }
            }
            Condition::HasItem(item) => self.inventory.contains(item),
            Condition::HasVisited(node) => {
                self.history.iter().any(|h| h.node_id == *node)
            }
            Condition::HasFlag(flag) => {
                // Flags stored as qualities with value 1
                self.qualities.get(flag)
                    .map(|q| q.value > 0)
                    .unwrap_or(false)
            }
            Condition::Not(cond) => !self.check_condition(cond),
            Condition::And(conds) => conds.iter().all(|c| self.check_condition(c)),
            Condition::Or(conds) => conds.iter().any(|c| self.check_condition(c)),
            Condition::Random(chance) => {
                rand::random::<f32>() < *chance
            }
        }
    }
    
    /// Apply a consequence
    pub fn apply_consequence(&mut self, consequence: &Consequence) -> Result<()> {
        match consequence {
            Consequence::SetQuality(name, change) => {
                let quality = self.qualities.entry(name.clone())
                    .or_insert_with(|| Quality {
                        name: name.clone(),
                        value: 0,
                        min: None,
                        max: None,
                        hidden: false,
                        category: QualityCategory::Attribute,
                    });
                
                match change {
                    QualityChange::Set(v) => quality.value = *v,
                    QualityChange::Add(v) => quality.value += v,
                    QualityChange::Subtract(v) => quality.value -= v,
                    QualityChange::Multiply(v) => quality.value *= v,
                    QualityChange::Divide(v) => {
                        if *v != 0 {
                            quality.value /= v;
                        }
                    }
                }
                
                // Clamp to min/max
                if let Some(min) = quality.min {
                    quality.value = quality.value.max(min);
                }
                if let Some(max) = quality.max {
                    quality.value = quality.value.min(max);
                }
            }
            Consequence::GiveItem(item) => {
                self.inventory.insert(item.clone());
            }
            Consequence::RemoveItem(item) => {
                self.inventory.remove(item);
            }
            Consequence::SetFlag(flag) => {
                self.qualities.insert(flag.clone(), Quality {
                    name: flag.clone(),
                    value: 1,
                    min: Some(0),
                    max: Some(1),
                    hidden: true,
                    category: QualityCategory::Progress,
                });
            }
            Consequence::UnsetFlag(flag) => {
                if let Some(quality) = self.qualities.get_mut(flag) {
                    quality.value = 0;
                }
            }
            Consequence::GoTo(node) => {
                self.load_node(node)?;
            }
            Consequence::EndGame(ending) => {
                // Handle game ending
                self.current_node = ending.clone();
            }
        }
        
        Ok(())
    }
    
    /// Get available storylets
    pub fn get_available_storylets(&self) -> Vec<&Storylet> {
        self.storylets.iter()
            .filter(|storylet| {
                // Check if usable
                if !storylet.repeatable && storylet.uses > 0 {
                    return false;
                }
                if let Some(max) = storylet.max_uses {
                    if storylet.uses >= max {
                        return false;
                    }
                }
                
                // Check conditions
                self.check_conditions(&storylet.conditions)
            })
            .collect()
    }
    
    /// Create a checkpoint
    pub fn create_checkpoint(&mut self, name: &str) {
        let checkpoint = GameCheckpoint {
            node_id: self.current_node.clone(),
            qualities: self.qualities.clone(),
            inventory: self.inventory.clone(),
            flags: self.qualities.iter()
                .filter(|(_, q)| q.hidden && q.value > 0)
                .map(|(k, _)| k.clone())
                .collect(),
        };
        
        self.checkpoints.insert(name.to_string(), checkpoint);
    }
    
    /// Restore from checkpoint
    pub fn restore_checkpoint(&mut self, name: &str) -> Result<()> {
        let checkpoint = self.checkpoints.get(name)
            .ok_or_else(|| Error::RuntimeError("Checkpoint not found".to_string()))?
            .clone();
        
        self.current_node = checkpoint.node_id;
        self.qualities = checkpoint.qualities;
        self.inventory = checkpoint.inventory;
        
        Ok(())
    }
}

impl Default for InteractiveFiction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interactive_fiction_creation() {
        let game = InteractiveFiction::new();
        assert!(game.nodes.is_empty());
        assert!(game.qualities.is_empty());
    }
    
    #[test]
    fn test_condition_checking() {
        let mut game = InteractiveFiction::new();
        
        // Add a quality
        game.qualities.insert("courage".to_string(), Quality {
            name: "courage".to_string(),
            value: 5,
            min: Some(0),
            max: Some(10),
            hidden: false,
            category: QualityCategory::Attribute,
        });
        
        // Test conditions
        assert!(game.check_condition(&Condition::Always));
        assert!(!game.check_condition(&Condition::Never));
        assert!(game.check_condition(&Condition::HasQuality(
            "courage".to_string(),
            ComparisonOp::Equal,
            5
        )));
        assert!(!game.check_condition(&Condition::HasQuality(
            "courage".to_string(),
            ComparisonOp::Greater,
            5
        )));
    }
}