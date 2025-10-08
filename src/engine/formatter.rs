//! Output formatting for different game modes

use crate::types::GameMode;

/// Output formatter for game responses
pub struct OutputFormatter {
    /// Maximum line width
    line_width: usize,
    /// Use color codes
    use_color: bool,
}

impl OutputFormatter {
    /// Create a new formatter
    pub fn new() -> Self {
        Self {
            line_width: 80,
            use_color: true,
        }
    }
    
    /// Format help text based on game mode
    pub fn format_help(&self, mode: GameMode) -> String {
        match mode {
            GameMode::TextAdventure => self.format_text_adventure_help(),
            GameMode::VisualNovel => self.format_visual_novel_help(),
            GameMode::InteractiveFiction => self.format_interactive_fiction_help(),
        }
    }
    
    fn format_text_adventure_help(&self) -> String {
        r#"# Text Adventure Commands

## Movement
- GO [direction] - Move in a direction (north, south, east, west, up, down)
- NORTH/N, SOUTH/S, EAST/E, WEST/W - Quick movement
- ENTER, EXIT - Enter or leave locations

## Objects
- LOOK/L - Describe current location
- EXAMINE/X [object] - Look at something closely
- TAKE/GET [item] - Pick up an item
- DROP [item] - Drop an item
- INVENTORY/I - List what you're carrying
- USE [item] - Use an item
- USE [item] ON [target] - Use item on something

## Interaction
- TALK TO [character] - Start conversation
- SAY [text] - Say something
- GIVE [item] TO [character] - Give an item

## System
- SAVE [slot] - Save your game
- LOAD [slot] - Load a saved game
- HELP - Show this help
- QUIT - Exit the game

Tips:
- Most commands can be abbreviated (X for EXAMINE)
- The parser understands variations ("pick up" = "take")
- Try EXAMINE on everything for clues!"#.to_string()
    }
    
    fn format_visual_novel_help(&self) -> String {
        r#"# Visual Novel Controls

## Navigation
- CLICK/SPACE - Advance text
- ENTER - Confirm choice
- ESC - Open menu

## Features
- SAVE - Save your progress
- LOAD - Load saved game
- SKIP - Skip seen text
- AUTO - Auto-advance text
- LOG - View text history

## Settings
- Text speed adjustment
- Volume controls
- Display options"#.to_string()
    }
    
    fn format_interactive_fiction_help(&self) -> String {
        r#"# Interactive Fiction

Click on choices to progress through the story.

## Controls
- Number keys (1-9) - Quick select choices
- SAVE - Save progress
- BACK - Undo last choice
- RESTART - Start over

## Features
- Your choices shape the story
- Multiple endings available
- Character relationships matter"#.to_string()
    }
    
    /// Wrap text to line width
    pub fn wrap_text(&self, text: &str) -> String {
        let mut result = String::new();
        
        for paragraph in text.split("\n\n") {
            if !result.is_empty() {
                result.push_str("\n\n");
            }
            
            let wrapped = self.wrap_paragraph(paragraph);
            result.push_str(&wrapped);
        }
        
        result
    }
    
    fn wrap_paragraph(&self, text: &str) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + word.len() + 1 <= self.line_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        lines.join("\n")
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}