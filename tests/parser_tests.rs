//! Tests for command parsing and fuzzy matching

use plotscript::parser::VocabularyBuilder;
use plotscript::engine::{CommandParser, Command};
use plotscript::world::World;
use plotscript::types::Direction;
use std::time::Instant;

#[test]
fn test_fuzzy_matcher_exact_match() {
    let matcher = VocabularyBuilder::new().build(70);
    
    // Test exact verb matches
    assert_eq!(matcher.match_verb("take"), Some("take".to_string()));
    assert_eq!(matcher.match_verb("examine"), Some("examine".to_string()));
    assert_eq!(matcher.match_verb("inventory"), Some("inventory".to_string()));
}

#[test]
fn test_fuzzy_matcher_typos() {
    let matcher = VocabularyBuilder::new().build(70);
    
    // Test common typos
    assert_eq!(matcher.match_verb("tkae"), Some("take".to_string()));
    assert_eq!(matcher.match_verb("exmaine"), Some("examine".to_string()));
    assert_eq!(matcher.match_verb("invntory"), Some("inventory".to_string()));
    assert_eq!(matcher.match_verb("tlak"), Some("talk".to_string()));
}

#[test]
fn test_fuzzy_matcher_aliases() {
    let matcher = VocabularyBuilder::new().build(70);
    
    // Test aliases
    assert_eq!(matcher.match_verb("n"), Some("go north".to_string()));
    assert_eq!(matcher.match_verb("i"), Some("inventory".to_string()));
    assert_eq!(matcher.match_verb("x"), Some("examine".to_string()));
    assert_eq!(matcher.match_verb("l"), Some("look".to_string()));
}

#[test]
fn test_fuzzy_object_matching() {
    let mut builder = VocabularyBuilder::new();
    builder.add_object_synonyms(
        "brass key".to_string(),
        vec!["key".to_string(), "brass".to_string()]
    );
    builder.add_object_synonyms(
        "oil lamp".to_string(),
        vec!["lamp".to_string(), "lantern".to_string()]
    );
    
    let matcher = builder.build(70);
    let context = vec!["brass key".to_string(), "oil lamp".to_string()];
    
    // Test exact matches
    assert_eq!(matcher.match_object("brass key", &context), Some("brass key".to_string()));
    assert_eq!(matcher.match_object("oil lamp", &context), Some("oil lamp".to_string()));
    
    // Test synonyms
    assert_eq!(matcher.match_object("key", &context), Some("brass key".to_string()));
    assert_eq!(matcher.match_object("lantern", &context), Some("oil lamp".to_string()));
    
    // Test fuzzy matching
    assert_eq!(matcher.match_object("braas key", &context), Some("brass key".to_string()));
    assert_eq!(matcher.match_object("oill lamp", &context), Some("oil lamp".to_string()));
}

#[test]
fn test_fuzzy_parse_command() {
    let matcher = VocabularyBuilder::new().build(70);
    let context = vec!["sword".to_string(), "shield".to_string()];
    
    // Test simple commands
    assert_eq!(
        matcher.parse_command("look", &context),
        Some(("look".to_string(), None))
    );
    
    // Test commands with objects
    assert_eq!(
        matcher.parse_command("take sword", &context),
        Some(("take".to_string(), Some("sword".to_string())))
    );
    
    // Test with typos
    assert_eq!(
        matcher.parse_command("tkae sheild", &context),
        Some(("take".to_string(), Some("shield".to_string())))
    );
    
    // Test aliases
    assert_eq!(
        matcher.parse_command("x sword", &context),
        Some(("examine".to_string(), Some("sword".to_string())))
    );
}

#[test]
fn test_command_parser_movement() {
    use plotscript::engine::Command;
    
    let parser = CommandParser::new();
    let world = World::new();
    
    // Test various movement commands
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
    
    assert_eq!(
        parser.parse("head south", &world).unwrap(),
        Command::Go(Direction::South)
    );
}

#[test]
fn test_command_parser_objects() {
    use plotscript::engine::Command;
    
    let parser = CommandParser::new();
    let world = World::new();
    
    // Test take commands
    assert_eq!(
        parser.parse("take the brass key", &world).unwrap(),
        Command::Take("brass key".to_string())
    );
    
    assert_eq!(
        parser.parse("get lamp", &world).unwrap(),
        Command::Take("lamp".to_string())
    );
    
    assert_eq!(
        parser.parse("pick up sword", &world).unwrap(),
        Command::Take("sword".to_string())
    );
}

#[test]
fn test_command_parser_examine() {
    use plotscript::engine::Command;
    
    let parser = CommandParser::new();
    let world = World::new();
    
    // Test examine commands
    assert_eq!(
        parser.parse("examine the chest", &world).unwrap(),
        Command::Examine("chest".to_string())
    );
    
    assert_eq!(
        parser.parse("look at painting", &world).unwrap(),
        Command::Examine("painting".to_string())
    );
    
    assert_eq!(
        parser.parse("x mirror", &world).unwrap(),
        Command::Examine("mirror".to_string())
    );
}

#[test]
fn test_command_parser_use() {
    use plotscript::engine::Command;
    
    let parser = CommandParser::new();
    let world = World::new();
    
    // Test use commands
    assert_eq!(
        parser.parse("use key", &world).unwrap(),
        Command::Use("key".to_string())
    );
    
    assert_eq!(
        parser.parse("use key on door", &world).unwrap(),
        Command::UseOn("key".to_string(), "door".to_string())
    );
    
    assert_eq!(
        parser.parse("apply potion", &world).unwrap(),
        Command::Use("potion".to_string())
    );
}

#[test]
fn test_command_parser_system() {
    use plotscript::engine::Command;
    
    let parser = CommandParser::new();
    let world = World::new();
    
    // Test system commands
    assert_eq!(
        parser.parse("inventory", &world).unwrap(),
        Command::Inventory
    );
    
    assert_eq!(
        parser.parse("i", &world).unwrap(),
        Command::Inventory
    );
    
    assert_eq!(
        parser.parse("help", &world).unwrap(),
        Command::Help
    );
    
    assert_eq!(
        parser.parse("save", &world).unwrap(),
        Command::Save(None)
    );
    
    assert_eq!(
        parser.parse("save game to slot 3", &world).unwrap(),
        Command::Save(Some(3))
    );
}

#[test]
fn test_threshold_settings() {
    // Test strict matching (threshold 100)
    let strict_matcher = VocabularyBuilder::new().build(100);
    assert_eq!(strict_matcher.match_verb("take"), Some("take".to_string()));
    assert_eq!(strict_matcher.match_verb("tkae"), None); // No fuzzy match
    
    // Test lenient matching (threshold 50)
    let lenient_matcher = VocabularyBuilder::new().build(50);
    assert_eq!(lenient_matcher.match_verb("take"), Some("take".to_string()));
    assert_eq!(lenient_matcher.match_verb("tkae"), Some("take".to_string())); // Fuzzy match works
}