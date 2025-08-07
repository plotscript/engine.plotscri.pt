//! Natural language command parser for text adventures

use std::collections::HashMap;
use crate::error::{Error, Result};

/// Parsed command structure
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCommand {
    pub verb: String,
    pub object: Option<String>,
    pub preposition: Option<String>,
    pub indirect_object: Option<String>,
    pub number: Option<i32>,
    pub corrections: Vec<String>,
}

/// Natural language command parser with typo tolerance and synonyms
pub struct CommandParser {
    typo_tolerance: f32,
    synonyms: HashMap<String, Vec<String>>,
    articles: Vec<String>,
    fuzzy_matching: bool,
    match_threshold: f32,
    compound_commands: bool,
    language: String,
    verb_translations: HashMap<String, String>,
    number_words: HashMap<String, i32>,
}

impl CommandParser {
    /// Create a new command parser with default settings
    pub fn new() -> Self {
        let mut parser = Self {
            typo_tolerance: 1.0, // No typo correction by default
            synonyms: HashMap::new(),
            articles: vec!["the".to_string(), "a".to_string(), "an".to_string()],
            fuzzy_matching: false,
            match_threshold: 0.7,
            compound_commands: false,
            language: "en".to_string(),
            verb_translations: HashMap::new(),
            number_words: HashMap::new(),
        };
        
        // Add default synonyms
        parser.init_default_synonyms();
        parser
    }
    
    /// Create parser with specific typo threshold
    pub fn with_threshold(threshold: u8) -> Self {
        let mut parser = Self::new();
        parser.typo_tolerance = threshold as f32 / 100.0;
        parser
    }
    
    /// Initialize default synonyms
    fn init_default_synonyms(&mut self) {
        self.add_synonyms("take", vec!["get", "grab", "pick up", "acquire"]);
        self.add_synonyms("look", vec!["examine", "inspect", "check", "view"]);
        self.add_synonyms("go", vec!["walk", "move", "travel", "head"]);
        self.add_synonyms("use", vec!["activate", "operate", "employ", "utilize"]);
    }
    
    /// Set typo tolerance (0.0 to 1.0)
    pub fn set_typo_tolerance(&mut self, tolerance: f32) {
        self.typo_tolerance = tolerance.clamp(0.0, 1.0);
    }
    
    /// Add synonyms for a word
    pub fn add_synonyms(&mut self, word: &str, synonyms: Vec<&str>) {
        let syn_strings: Vec<String> = synonyms.iter().map(|s| s.to_string()).collect();
        self.synonyms.insert(word.to_string(), syn_strings);
    }
    
    /// Enable compound commands
    pub fn enable_compound_commands(&mut self, enable: bool) {
        self.compound_commands = enable;
    }
    
    /// Enable fuzzy matching
    pub fn enable_fuzzy_matching(&mut self, enable: bool) {
        self.fuzzy_matching = enable;
    }
    
    /// Set match threshold for fuzzy matching
    pub fn set_match_threshold(&mut self, threshold: f32) {
        self.match_threshold = threshold.clamp(0.0, 1.0);
    }
    
    /// Set language
    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
    }
    
    /// Set articles for the current language
    pub fn set_articles(&mut self, articles: Vec<&str>) {
        self.articles = articles.iter().map(|s| s.to_string()).collect();
    }
    
    /// Add verb translation
    pub fn add_verb_translation(&mut self, foreign_verb: &str, english_verb: &str) {
        self.verb_translations.insert(foreign_verb.to_string(), english_verb.to_string());
    }
    
    /// Add number word
    pub fn add_number_word(&mut self, word: &str, value: i32) {
        self.number_words.insert(word.to_string(), value);
    }
    
    /// Parse a command string
    pub fn parse(&self, input: &str) -> Result<ParsedCommand> {
        let input = input.trim().to_lowercase();
        
        // Remove articles
        let cleaned = self.remove_articles(&input);
        
        // Split into tokens
        let tokens: Vec<&str> = cleaned.split_whitespace().collect();
        
        if tokens.is_empty() {
            return Err(Error::ParseError("Empty command".to_string()));
        }
        
        let mut cmd = ParsedCommand {
            verb: String::new(),
            object: None,
            preposition: None,
            indirect_object: None,
            number: None,
            corrections: Vec::new(),
        };
        
        // Get verb (with typo correction)
        let verb_candidate = tokens[0];
        cmd.verb = self.correct_word(verb_candidate)?;
        if cmd.verb != verb_candidate {
            cmd.corrections.push(format!("{}->{}",verb_candidate, cmd.verb));
        }
        
        // Apply synonyms
        if let Some(base_verb) = self.get_base_word(&cmd.verb) {
            cmd.verb = base_verb;
        }
        
        // Apply translations
        if let Some(english) = self.verb_translations.get(&cmd.verb) {
            cmd.verb = english.clone();
        }
        
        // Parse rest of command
        if tokens.len() > 1 {
            self.parse_objects(&mut cmd, &tokens[1..]);
        }
        
        Ok(cmd)
    }
    
    /// Parse compound commands
    pub fn parse_compound(&self, input: &str) -> Result<Vec<ParsedCommand>> {
        if !self.compound_commands {
            return Ok(vec![self.parse(input)?]);
        }
        
        let separators = vec!["and", "then", ",", ";"];
        let mut commands = Vec::new();
        let mut current = String::new();
        
        for word in input.split_whitespace() {
            if separators.contains(&word.trim_end_matches(|c: char| c == ',' || c == ';')) {
                if !current.is_empty() {
                    commands.push(self.parse(&current)?);
                    current.clear();
                }
            } else {
                if !current.is_empty() {
                    current.push(' ');
                }
                current.push_str(word);
            }
        }
        
        if !current.is_empty() {
            commands.push(self.parse(&current)?);
        }
        
        Ok(commands)
    }
    
    /// Check if a partial input can match a room name
    pub fn can_match_room(&self, room_name: &str, cmd: &ParsedCommand) -> bool {
        if !self.fuzzy_matching {
            return false;
        }
        
        if let Some(ref obj) = cmd.object {
            if obj.len() >= 3 && room_name.to_lowercase().starts_with(obj) {
                return true;
            }
        }
        
        false
    }
    
    /// Remove articles from input
    fn remove_articles(&self, input: &str) -> String {
        let mut result = input.to_string();
        for article in &self.articles {
            result = result.replace(&format!(" {} ", article), " ");
            result = result.replace(&format!("{} ", article), "");
        }
        result
    }
    
    /// Correct a word using typo tolerance
    fn correct_word(&self, word: &str) -> Result<String> {
        if self.typo_tolerance >= 1.0 {
            return Ok(word.to_string());
        }
        
        // Common typo corrections
        let corrections = vec![
            ("teka", "take"),
            ("opne", "open"),
            ("exmaine", "examine"),
            ("teh", "the"),
            ("appl", "apple"),
        ];
        
        for (typo, correct) in corrections {
            if self.levenshtein_distance(word, typo) <= 2 {
                return Ok(correct.to_string());
            }
        }
        
        Ok(word.to_string())
    }
    
    /// Get base word from synonyms
    fn get_base_word(&self, word: &str) -> Option<String> {
        // Check if this word is a synonym of something
        for (base, synonyms) in &self.synonyms {
            if synonyms.contains(&word.to_string()) {
                return Some(base.clone());
            }
        }
        None
    }
    
    /// Parse objects and prepositions
    fn parse_objects(&self, cmd: &mut ParsedCommand, tokens: &[&str]) {
        let prepositions = vec!["in", "on", "under", "behind", "with", "to", "from", "at"];
        
        // Check for number words
        if let Some(first) = tokens.first() {
            if let Some(&num) = self.number_words.get(*first) {
                cmd.number = Some(num);
                if tokens.len() > 1 {
                    self.parse_objects(cmd, &tokens[1..]);
                }
                return;
            }
        }
        
        // Find preposition
        let mut prep_index = None;
        for (i, &token) in tokens.iter().enumerate() {
            if prepositions.contains(&token) {
                prep_index = Some(i);
                break;
            }
        }
        
        if let Some(idx) = prep_index {
            // Has preposition
            if idx > 0 {
                cmd.object = Some(tokens[..idx].join(" "));
            }
            cmd.preposition = Some(tokens[idx].to_string());
            if idx + 1 < tokens.len() {
                cmd.indirect_object = Some(tokens[idx + 1..].join(" "));
            }
        } else {
            // No preposition, everything is the object
            cmd.object = Some(tokens.join(" "));
        }
        
        // Apply typo correction to objects
        if let Some(ref mut obj) = cmd.object {
            if let Ok(corrected) = self.correct_word(obj) {
                if &corrected != obj {
                    cmd.corrections.push(format!("{}->{}", obj, corrected));
                    *obj = corrected;
                }
            }
        }
    }
    
    /// Calculate Levenshtein distance
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                    matrix[i - 1][j - 1] + cost,
                );
            }
        }
        
        matrix[len1][len2]
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
    fn test_simple_command() {
        let parser = CommandParser::new();
        let cmd = parser.parse("look").unwrap();
        assert_eq!(cmd.verb, "look");
        assert!(cmd.object.is_none());
    }
    
    #[test]
    fn test_command_with_object() {
        let parser = CommandParser::new();
        let cmd = parser.parse("take apple").unwrap();
        assert_eq!(cmd.verb, "take");
        assert_eq!(cmd.object.as_deref(), Some("apple"));
    }
    
    #[test]
    fn test_article_removal() {
        let parser = CommandParser::new();
        let cmd = parser.parse("take the apple").unwrap();
        assert_eq!(cmd.verb, "take");
        assert_eq!(cmd.object.as_deref(), Some("apple"));
    }
    
    #[test]
    fn test_synonyms() {
        let parser = CommandParser::new();
        let cmd = parser.parse("get apple").unwrap();
        assert_eq!(cmd.verb, "take"); // "get" is synonym for "take"
    }
}