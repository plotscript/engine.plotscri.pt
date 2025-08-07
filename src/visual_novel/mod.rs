//! Visual Novel game format support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{Error, Result};

/// Visual novel game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNovel {
    /// Current scene
    pub current_scene: String,
    /// Available scenes
    pub scenes: HashMap<String, Scene>,
    /// Character definitions
    pub characters: HashMap<String, Character>,
    /// Background assets
    pub backgrounds: HashMap<String, BackgroundAsset>,
    /// Audio tracks
    pub audio: HashMap<String, AudioAsset>,
    /// Current dialogue position
    pub dialogue_index: usize,
    /// Text display speed
    pub text_speed: TextSpeed,
    /// Auto-advance settings
    pub auto_mode: bool,
    /// Skip mode
    pub skip_mode: SkipMode,
}

/// Scene definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub background: Option<String>,
    pub music: Option<String>,
    pub dialogue: Vec<DialogueNode>,
    pub characters_on_screen: Vec<CharacterPosition>,
}

/// Character definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub sprites: HashMap<String, String>,  // expression -> asset path
    pub default_sprite: String,
}

/// Character position on screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPosition {
    pub character_id: String,
    pub position: ScreenPosition,
    pub sprite: String,
    pub opacity: f32,
}

/// Screen positions for characters
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ScreenPosition {
    Left,
    CenterLeft,
    Center,
    CenterRight,
    Right,
    Custom(f32, f32),  // x, y coordinates
}

/// Dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub speaker: Option<String>,
    pub text: String,
    pub voice: Option<String>,
    pub choices: Vec<Choice>,
    pub effects: Vec<VisualEffect>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}

/// Player choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub target: ChoiceTarget,
    pub conditions: Vec<String>,
    pub consequences: Vec<String>,
}

/// Choice target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChoiceTarget {
    Scene(String),
    DialogueIndex(usize),
    Ending(String),
}

/// Visual effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualEffect {
    Transition(TransitionType),
    CharacterAnimation(String, AnimationType),
    ScreenShake(f32),
    Flash(String),  // color
    Weather(WeatherType),
}

/// Transition types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    Dissolve,
    SlideLeft,
    SlideRight,
    Wipe,
    Zoom,
}

/// Character animations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimationType {
    SlideIn,
    SlideOut,
    FadeIn,
    FadeOut,
    Shake,
    Bounce,
}

/// Weather effects
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeatherType {
    None,
    Rain,
    Snow,
    Fog,
    Cherry,  // cherry blossoms
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
    Off,
    ReadOnly,
    All,
}

/// Background asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundAsset {
    pub id: String,
    pub path: String,
    pub variants: HashMap<String, String>,  // time of day variants
}

/// Audio asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAsset {
    pub id: String,
    pub path: String,
    pub volume: f32,
    pub loop_track: bool,
}

impl VisualNovel {
    /// Create a new visual novel
    pub fn new() -> Self {
        Self {
            current_scene: String::new(),
            scenes: HashMap::new(),
            characters: HashMap::new(),
            backgrounds: HashMap::new(),
            audio: HashMap::new(),
            dialogue_index: 0,
            text_speed: TextSpeed::Normal,
            auto_mode: false,
            skip_mode: SkipMode::Off,
        }
    }
    
    /// Load a scene
    pub fn load_scene(&mut self, scene_id: &str) -> Result<()> {
        if !self.scenes.contains_key(scene_id) {
            return Err(Error::RuntimeError(format!("Scene '{}' not found", scene_id)));
        }
        
        self.current_scene = scene_id.to_string();
        self.dialogue_index = 0;
        Ok(())
    }
    
    /// Get current scene
    pub fn current_scene(&self) -> Option<&Scene> {
        self.scenes.get(&self.current_scene)
    }
    
    /// Get current dialogue
    pub fn current_dialogue(&self) -> Option<&DialogueNode> {
        self.current_scene()
            .and_then(|scene| scene.dialogue.get(self.dialogue_index))
    }
    
    /// Advance dialogue
    pub fn advance(&mut self) -> Result<AdvanceResult> {
        let scene = self.scenes.get(&self.current_scene)
            .ok_or_else(|| Error::RuntimeError("No current scene".to_string()))?;
        
        if self.dialogue_index + 1 < scene.dialogue.len() {
            self.dialogue_index += 1;
            Ok(AdvanceResult::Continue)
        } else {
            Ok(AdvanceResult::SceneEnd)
        }
    }
    
    /// Make a choice
    pub fn make_choice(&mut self, choice_index: usize) -> Result<()> {
        let choice_target = {
            let dialogue = self.current_dialogue()
                .ok_or_else(|| Error::RuntimeError("No current dialogue".to_string()))?;
            
            let choice = dialogue.choices.get(choice_index)
                .ok_or_else(|| Error::RuntimeError("Invalid choice index".to_string()))?;
            
            choice.target.clone()
        };
        
        match choice_target {
            ChoiceTarget::Scene(scene_id) => {
                self.load_scene(&scene_id)?;
            }
            ChoiceTarget::DialogueIndex(index) => {
                self.dialogue_index = index;
            }
            ChoiceTarget::Ending(_ending_id) => {
                // Handle ending
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    /// Set text speed
    pub fn set_text_speed(&mut self, speed: TextSpeed) {
        self.text_speed = speed;
    }
    
    /// Toggle auto mode
    pub fn toggle_auto_mode(&mut self) {
        self.auto_mode = !self.auto_mode;
    }
    
    /// Set skip mode
    pub fn set_skip_mode(&mut self, mode: SkipMode) {
        self.skip_mode = mode;
    }
}

/// Result of advancing dialogue
#[derive(Debug, Clone, Copy)]
pub enum AdvanceResult {
    Continue,
    SceneEnd,
    GameEnd,
}

impl Default for VisualNovel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visual_novel_creation() {
        let vn = VisualNovel::new();
        assert_eq!(vn.dialogue_index, 0);
        assert!(!vn.auto_mode);
    }
    
    #[test]
    fn test_scene_loading() {
        let mut vn = VisualNovel::new();
        
        let scene = Scene {
            id: "intro".to_string(),
            background: Some("school.png".to_string()),
            music: Some("peaceful.ogg".to_string()),
            dialogue: vec![
                DialogueNode {
                    speaker: Some("Alice".to_string()),
                    text: "Hello!".to_string(),
                    voice: None,
                    choices: vec![],
                    effects: vec![],
                    conditions: vec![],
                    actions: vec![],
                },
            ],
            characters_on_screen: vec![],
        };
        
        vn.scenes.insert("intro".to_string(), scene);
        vn.load_scene("intro").unwrap();
        
        assert_eq!(vn.current_scene, "intro");
        assert_eq!(vn.dialogue_index, 0);
    }
}