//! Fuzzy matching for typo tolerance in natural language parsing

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;

/// Fuzzy matcher for commands and vocabulary
pub struct FuzzyCommandMatcher {
    /// The fuzzy matching algorithm
    matcher: SkimMatcherV2,
    /// Minimum score threshold (0-100)
    threshold: i64,
    /// Known verbs
    verbs: Vec<String>,
    /// Known objects (items, characters, etc.)
    objects: HashMap<String, Vec<String>>,
    /// Command aliases
    aliases: HashMap<String, String>,
}

impl FuzzyCommandMatcher {
    /// Create a new fuzzy matcher
    pub fn new(threshold: u8) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            threshold: threshold as i64,
            verbs: Vec::new(),
            objects: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
    
    /// Add a known verb
    pub fn add_verb(&mut self, verb: &str) {
        self.verbs.push(verb.to_lowercase());
    }
    
    /// Add multiple verbs
    pub fn add_verbs(&mut self, verbs: &[&str]) {
        for verb in verbs {
            self.add_verb(verb);
        }
    }
    
    /// Add an object with its synonyms
    pub fn add_object(&mut self, object: &str, synonyms: Vec<String>) {
        self.objects.insert(object.to_lowercase(), synonyms);
    }
    
    /// Add a command alias
    pub fn add_alias(&mut self, alias: &str, command: &str) {
        self.aliases.insert(alias.to_lowercase(), command.to_lowercase());
    }
    
    /// Find the best matching verb
    pub fn match_verb(&self, input: &str) -> Option<String> {
        let input_lower = input.to_lowercase();
        
        // Check exact match first
        if self.verbs.contains(&input_lower) {
            return Some(input_lower);
        }
        
        // Check aliases
        if let Some(command) = self.aliases.get(&input_lower) {
            return Some(command.clone());
        }
        
        // Fuzzy match - using Levenshtein distance-based scoring
        let mut best_match = None;
        let mut best_score = 0;
        
        for verb in &self.verbs {
            if let Some(score) = self.matcher.fuzzy_match(verb, &input_lower) {
                // The fuzzy_matcher crate uses different scoring, generally scores 30-100
                // For typos with 1-2 character differences, scores are typically 40-60
                if score > best_score {
                    best_score = score;
                    best_match = Some(verb.clone());
                }
            }
        }
        
        // Only return a match if the score is reasonable for a typo
        // Typically, a score of 30+ indicates a reasonable match
        if best_score >= 30 {
            best_match
        } else {
            None
        }
    }
    
    /// Find the best matching object
    pub fn match_object(&self, input: &str, context_objects: &[String]) -> Option<String> {
        let input_lower = input.to_lowercase();
        
        // First check context objects (items in room, inventory, etc.)
        for object in context_objects {
            let object_lower = object.to_lowercase();
            
            // Exact match
            if object_lower == input_lower {
                return Some(object.clone());
            }
            
            // Check if this object has synonyms
            if let Some(synonyms) = self.objects.get(&object_lower) {
                if synonyms.iter().any(|s| s.to_lowercase() == input_lower) {
                    return Some(object.clone());
                }
            }
        }
        
        // Fuzzy match against context objects
        let mut best_match = None;
        let mut best_score = 0;
        
        for object in context_objects {
            let object_lower = object.to_lowercase();
            
            // Match against object name
            if let Some(score) = self.matcher.fuzzy_match(&object_lower, &input_lower) {
                if score > best_score {
                    best_score = score;
                    best_match = Some(object.clone());
                }
            }
            
            // Match against synonyms
            if let Some(synonyms) = self.objects.get(&object_lower) {
                for synonym in synonyms {
                    if let Some(score) = self.matcher.fuzzy_match(&synonym.to_lowercase(), &input_lower) {
                        if score > best_score {
                            best_score = score;
                            best_match = Some(object.clone());
                        }
                    }
                }
            }
        }
        
        // Only return a match if the score is reasonable
        if best_score >= 30 {
            best_match
        } else {
            None
        }
    }
    
    /// Parse a command with fuzzy matching
    pub fn parse_command(&self, input: &str, context_objects: &[String]) -> Option<(String, Option<String>)> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            return None;
        }
        
        // First check if this is an alias that expands to multiple words
        if let Some(expanded) = self.aliases.get(tokens[0]) {
            // For aliases like "n" -> "go north", return the expanded form
            return Some((expanded.clone(), None));
        }
        
        // Try to match the verb
        let verb = self.match_verb(tokens[0])?;
        
        // If there's more tokens, try to match an object
        if tokens.len() > 1 {
            // Join remaining tokens as potential object name
            let object_input = tokens[1..].join(" ");
            let object = self.match_object(&object_input, context_objects);
            Some((verb, object))
        } else {
            Some((verb, None))
        }
    }
}

/// Vocabulary builder for fuzzy matching
pub struct VocabularyBuilder {
    verbs: Vec<&'static str>,
    aliases: HashMap<&'static str, &'static str>,
    objects: HashMap<String, Vec<String>>,
}

impl VocabularyBuilder {
    /// Create a new vocabulary builder
    pub fn new() -> Self {
        Self {
            verbs: vec![
                // Movement
                "go", "walk", "run", "move", "travel", "head",
                // Object interaction
                "take", "get", "pick", "grab", "collect",
                "drop", "put", "place", "leave", "discard",
                "use", "activate", "apply", "employ",
                "examine", "look", "inspect", "check", "study",
                "open", "unlock", "close", "shut", "lock",
                // Character interaction
                "talk", "speak", "say", "ask", "tell", "chat",
                "give", "offer", "hand", "present",
                // System
                "inventory", "save", "load", "help", "quit", "exit",
            ],
            aliases: HashMap::from([
                ("n", "go north"),
                ("s", "go south"),
                ("e", "go east"),
                ("w", "go west"),
                ("ne", "go northeast"),
                ("nw", "go northwest"),
                ("se", "go southeast"),
                ("sw", "go southwest"),
                ("u", "go up"),
                ("d", "go down"),
                ("i", "inventory"),
                ("inv", "inventory"),
                ("l", "look"),
                ("x", "examine"),
                ("g", "get"),
                ("t", "take"),
                ("pickup", "take"),
                ("pic", "take"),
            ]),
            objects: HashMap::new(),
        }
    }
    
    /// Add object synonyms
    pub fn add_object_synonyms(&mut self, object: String, synonyms: Vec<String>) {
        self.objects.insert(object, synonyms);
    }
    
    /// Build a fuzzy matcher from this vocabulary
    pub fn build(self, threshold: u8) -> FuzzyCommandMatcher {
        let mut matcher = FuzzyCommandMatcher::new(threshold);
        
        // Add verbs
        for verb in self.verbs {
            matcher.add_verb(verb);
        }
        
        // Add aliases
        for (alias, command) in self.aliases {
            matcher.add_alias(alias, command);
        }
        
        // Add objects
        for (object, synonyms) in self.objects {
            matcher.add_object(&object, synonyms);
        }
        
        matcher
    }
}

impl Default for VocabularyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exact_verb_match() {
        let matcher = VocabularyBuilder::new().build(70);
        assert_eq!(matcher.match_verb("take"), Some("take".to_string()));
        assert_eq!(matcher.match_verb("TAKE"), Some("take".to_string()));
    }
    
    #[test]
    fn test_fuzzy_verb_match() {
        let matcher = VocabularyBuilder::new().build(70);
        // Common typos
        assert_eq!(matcher.match_verb("tkae"), Some("take".to_string()));
        assert_eq!(matcher.match_verb("exmaine"), Some("examine".to_string()));
        assert_eq!(matcher.match_verb("invntory"), Some("inventory".to_string()));
    }
    
    #[test]
    fn test_alias_match() {
        let matcher = VocabularyBuilder::new().build(70);
        assert_eq!(matcher.match_verb("n"), Some("go north".to_string()));
        assert_eq!(matcher.match_verb("i"), Some("inventory".to_string()));
        assert_eq!(matcher.match_verb("x"), Some("examine".to_string()));
    }
    
    #[test]
    fn test_object_match() {
        let mut builder = VocabularyBuilder::new();
        builder.add_object_synonyms(
            "brass key".to_string(),
            vec!["key".to_string(), "brass".to_string()]
        );
        let matcher = builder.build(70);
        
        let context = vec!["brass key".to_string(), "lantern".to_string()];
        
        // Exact match
        assert_eq!(matcher.match_object("brass key", &context), Some("brass key".to_string()));
        // Synonym match
        assert_eq!(matcher.match_object("key", &context), Some("brass key".to_string()));
        // Fuzzy match
        assert_eq!(matcher.match_object("braas key", &context), Some("brass key".to_string()));
        assert_eq!(matcher.match_object("lantrn", &context), Some("lantern".to_string()));
    }
    
    #[test]
    fn test_parse_command() {
        let matcher = VocabularyBuilder::new().build(70);
        let context = vec!["brass key".to_string(), "lantern".to_string()];
        
        // Simple verb
        assert_eq!(
            matcher.parse_command("look", &context),
            Some(("look".to_string(), None))
        );
        
        // Verb + object with typo
        assert_eq!(
            matcher.parse_command("tkae lantrn", &context),
            Some(("take".to_string(), Some("lantern".to_string())))
        );
        
        // Alias
        assert_eq!(
            matcher.parse_command("n", &context),
            Some(("go north".to_string(), None))
        );
    }
}