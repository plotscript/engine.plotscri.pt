//! Comprehensive integration tests for the PlotScript engine

use plotscript::{Engine, EngineConfig, GameMode, init, types::*};
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_engine_creation() {
    init();
    let engine = Engine::new();
    assert_eq!(engine.config.mode, GameMode::InteractiveFiction);
    assert!(!engine.config.debug);
    assert_eq!(engine.config.max_inventory, 20);
}

#[test]
fn test_engine_with_custom_config() {
    init();
    let config = EngineConfig {
        mode: GameMode::VisualNovel,
        debug: true,
        max_inventory: 10,
        auto_save: false,
        history_size: 50,
        typo_correction: true,
        typo_threshold: 80,
    };
    
    let engine = Engine::with_config(config.clone());
    assert_eq!(engine.config.mode, GameMode::VisualNovel);
    assert!(engine.config.debug);
    assert_eq!(engine.config.max_inventory, 10);
}

#[test]
fn test_load_invalid_script() {
    init();
    let mut engine = Engine::new();
    let result = engine.load_script("invalid script content");
    assert!(result.is_err());
}

#[test]
fn test_start_without_script() {
    init();
    let mut engine = Engine::new();
    let result = engine.start();
    assert!(result.is_err());
}

#[test]
fn test_simple_text_adventure() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    // Create a simple game script in RON format
    let script = r#"
TextAdventure((
    title: "Test Adventure",
    author: "Test",
    description: Some("A test game"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "start",
    
    locations: {
        "start": (
            name: "Starting Room",
            description: "You are in a simple room.",
            exits: {
                north: "end_room",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "end_room": (
            name: "End Room",
            description: "You have reached the end.",
            exits: {},
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {},
    characters: {},
    vocabulary: None,
    events: None,
))
    "#;
    
    // Load and start the game
    engine.load_script(script).expect("Failed to load script");
    let response = engine.start().expect("Failed to start game");
    
    assert!(response.text.contains("Starting Room"));
    assert_eq!(response.location, Some("start".to_string()));
}

#[test]
fn test_movement_commands() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    let script = r#"
TextAdventure((
    title: "Movement Test",
    author: "Test",
    description: Some("Testing movement"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "room1",
    
    locations: {
        "room1": (
            name: "Room 1",
            description: "First room.",
            exits: {
                east: "room2",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "room2": (
            name: "Room 2",
            description: "Second room.",
            exits: {
                west: "room1",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {},
    characters: {},
    vocabulary: None,
    events: None,
))
    "#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Test movement
    let response = engine.process_input("go east").expect("Failed to process command");
    assert!(response.text.contains("Room 2"));
    assert_eq!(response.location, Some("room2".to_string()));
    
    // Test going back
    let response = engine.process_input("west").expect("Failed to process command");
    assert!(response.text.contains("Room 1"));
    assert_eq!(response.location, Some("room1".to_string()));
}

#[test]
fn test_inventory_commands() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    let script = r#"
TextAdventure((
    title: "Inventory Test",
    author: "Test",
    description: Some("Testing inventory"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "start",
    
    locations: {
        "start": (
            name: "Start Room",
            description: "A room with items.",
            exits: {},
            items: ["key", "lamp"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "key": (
            id: "key",
            name: Some("brass key"),
            description: Some("A shiny brass key."),
            location: Some("start"),
            takeable: true,
            weight: 1.0,
            is_container: false,
            contains: [],
            on_take: [],
            on_use: [],
            on_examine: [],
            properties: {},
        ),
        "lamp": (
            id: "lamp",
            name: None,
            description: Some("An old oil lamp."),
            location: Some("start"),
            takeable: true,
            weight: 5.0,
            is_container: false,
            contains: [],
            on_take: [],
            on_use: [],
            on_examine: [],
            properties: {},
        ),
    },
    characters: {},
    vocabulary: None,
    events: None,
))
    "#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Check inventory is empty
    let response = engine.process_input("inventory").expect("Failed to process command");
    assert!(response.text.contains("not carrying anything"));
    
    // Take an item
    let response = engine.process_input("take key").expect("Failed to process command");
    assert!(response.text.contains("take the"));
    
    // Check inventory again
    let response = engine.process_input("i").expect("Failed to process command");
    assert!(response.text.contains("brass key"));
}

#[test]
fn test_comprehensive_fuzzy_matching() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 70,
        ..Default::default()
    });
    
    let script = r#"
TextAdventure((
    title: "Fuzzy Test",
    author: "Test",
    description: Some("Testing fuzzy matching"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "start",
    
    locations: {
        "start": (
            name: "Start Room",
            description: "Test room.",
            exits: {},
            items: ["lantern"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "lantern": (
            id: "lantern",
            name: None,
            description: Some("A bright lantern."),
            location: Some("start"),
            takeable: true,
            weight: 3.0,
            is_container: false,
            contains: [],
            on_take: [],
            on_use: [],
            on_examine: [],
            properties: {},
        ),
    },
    characters: {},
    vocabulary: None,
    events: None,
))
    "#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Test fuzzy matching with typos
    let response = engine.process_input("tkae lantrn").expect("Failed to process command");
    assert!(response.text.contains("take the"));
    
    // Test examine with typo
    let response = engine.process_input("exmaine lantern").expect("Failed to process command");
    assert!(response.text.contains("bright lantern"));
    
    // Test more complex fuzzy matching scenarios
    let response = engine.process_input("pikc up lantren").expect("Failed");
    assert!(response.success);
    
    // Test command aliases with typos
    let response = engine.process_input("inv").expect("Failed");
    assert!(response.text.contains("lantern"));
    
    // Test threshold behavior
    let response = engine.process_input("xyz123"); // Should not match anything
    assert!(response.is_err() || !response.unwrap().success);
}

#[test]
fn test_save_and_load() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    let script = r#"
TextAdventure((
    title: "Save Test",
    author: "Test",
    description: Some("Testing save/load"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "room1",
    
    locations: {
        "room1": (
            name: "Room 1",
            description: "First room.",
            exits: {
                north: "room2",
            },
            items: ["coin"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "room2": (
            name: "Room 2",
            description: "Second room.",
            exits: {
                south: "room1",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "coin": (
            id: "coin",
            name: None,
            description: Some("A gold coin."),
            location: Some("room1"),
            takeable: true,
            weight: 0.1,
            is_container: false,
            contains: [],
            on_take: [],
            on_use: [],
            on_examine: [],
            properties: {},
        ),
    },
    characters: {},
    vocabulary: None,
    events: None,
))
    "#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Move to second room and take coin
    engine.process_input("take coin").expect("Failed");
    engine.process_input("go north").expect("Failed");
    
    // Save the game
    let response = engine.save_game(Some(1)).expect("Failed to save");
    assert!(response.text.contains("saved"));
    
    // Go back to first room
    engine.process_input("go south").expect("Failed");
    engine.process_input("drop coin").expect("Failed");
    
    // Load the save
    let response = engine.load_game(Some(1)).expect("Failed to load");
    assert!(response.text.contains("loaded"));
    
    // Check we're back in room2 with the coin
    let response = engine.process_input("look").expect("Failed");
    assert!(response.text.contains("Second room"));
    
    let response = engine.process_input("inventory").expect("Failed");
    assert!(response.text.contains("coin"));
}

// COMPREHENSIVE CORE ENGINE TESTS

#[test]
fn test_engine_configuration_defaults() {
    init();
    let config = EngineConfig::default();
    
    assert_eq!(config.mode, GameMode::InteractiveFiction);
    assert!(!config.debug);
    assert_eq!(config.max_inventory, 20);
    assert!(config.auto_save);
    assert_eq!(config.history_size, 100);
    assert!(config.typo_correction);
    assert_eq!(config.typo_threshold, 70);
}

#[test]
fn test_engine_configuration_custom() {
    init();
    
    // Test all possible configurations
    let config = EngineConfig {
        mode: GameMode::TextAdventure,
        debug: true,
        max_inventory: 50,
        auto_save: false,
        history_size: 200,
        typo_correction: false,
        typo_threshold: 90,
    };
    
    let engine = Engine::with_config(config.clone());
    
    assert_eq!(engine.config.mode, GameMode::TextAdventure);
    assert!(engine.config.debug);
    assert_eq!(engine.config.max_inventory, 50);
    assert!(!engine.config.auto_save);
    assert_eq!(engine.config.history_size, 200);
    assert!(!engine.config.typo_correction);
    assert_eq!(engine.config.typo_threshold, 90);
}

#[test]
fn test_game_state_management() {
    init();
    let mut state = GameState::new();
    
    // Test variable operations
    assert!(state.get_variable("nonexistent").is_none());
    
    state.set_variable("health", Value::Integer(100));
    assert_eq!(state.get_variable("health"), Some(&Value::Integer(100)));
    
    state.set_variable("name", Value::String("Hero".to_string()));
    assert_eq!(state.get_variable("name"), Some(&Value::String("Hero".to_string())));
    
    // Test flag operations
    assert!(!state.has_flag("has_key"));
    
    state.set_flag("has_key");
    assert!(state.has_flag("has_key"));
    
    state.clear_flag("has_key");
    assert!(!state.has_flag("has_key"));
    
    // Test inventory
    assert!(state.inventory.is_empty());
    state.inventory.push("sword".to_string());
    assert_eq!(state.inventory.len(), 1);
    
    // Test counters
    assert_eq!(state.score, 0);
    assert_eq!(state.turns, 0);
    
    state.score = 100;
    state.turns = 5;
    assert_eq!(state.score, 100);
    assert_eq!(state.turns, 5);
}

#[test]
fn test_response_structure() {
    init();
    let response = Response::default();
    
    assert!(response.text.is_empty());
    assert!(response.location.is_none());
    assert!(response.choices.is_empty());
    assert!(response.media.is_empty());
    assert!(response.sounds.is_empty());
    assert!(response.achievement.is_none());
    assert!(response.success);
    assert!(!response.ended);
    
    // Test custom response
    let custom_response = Response {
        text: "Game over!".to_string(),
        location: Some("ending".to_string()),
        ended: true,
        success: false,
        achievement: Some("Failure".to_string()),
        ..Default::default()
    };
    
    assert_eq!(custom_response.text, "Game over!");
    assert_eq!(custom_response.location, Some("ending".to_string()));
    assert!(custom_response.ended);
    assert!(!custom_response.success);
    assert_eq!(custom_response.achievement, Some("Failure".to_string()));
}

#[test]
fn test_engine_lifecycle() {
    init();
    let mut engine = Engine::new();
    
    // Test initial state
    assert_eq!(engine.config.mode, GameMode::InteractiveFiction);
    
    // Test attempting to start without a script
    let result = engine.start();
    assert!(result.is_err());
    
    // Test processing input without starting
    let result = engine.process_input("look");
    assert!(result.is_err());
}

#[test] 
fn test_inventory_limits() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        max_inventory: 2, // Small inventory for testing
        ..Default::default()
    });
    
    let script = create_test_script_with_items(&[
        ("item1", "First Item", "A test item"),
        ("item2", "Second Item", "Another test item"),
        ("item3", "Third Item", "Yet another test item"),
    ]);
    
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Take first item - should work
    let response = engine.process_input("take item1").expect("Failed");
    assert!(response.success);
    
    // Take second item - should work
    let response = engine.process_input("take item2").expect("Failed");
    assert!(response.success);
    
    // Try to take third item - should fail due to inventory limit
    let response = engine.process_input("take item3").expect("Failed");
    assert!(!response.success);
    assert!(response.text.contains("carrying too much"));
    
    // Drop one item and try again
    engine.process_input("drop item1").expect("Failed");
    let response = engine.process_input("take item3").expect("Failed");
    assert!(response.success);
}

#[test]
fn test_command_history() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        history_size: 3, // Small history for testing
        ..Default::default()
    });
    
    let script = create_simple_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Add commands to history
    engine.process_input("look").expect("Failed");
    engine.process_input("inventory").expect("Failed");
    engine.process_input("help").expect("Failed");
    
    // History should have 3 items
    // TODO: Add public method to access history
    // assert_eq!(engine.history.len(), 3);
    
    // Add another command - oldest should be removed
    engine.process_input("look around").expect("Failed");
    
    // TODO: Add public method to access history
    // assert_eq!(engine.history.len(), 3);
    // assert_eq!(engine.history[0], "inventory");
    // assert_eq!(engine.history[1], "help");
    // assert_eq!(engine.history[2], "look around");
}

#[test]
fn test_debug_mode() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        debug: true,
        ..Default::default()
    });
    
    let script = create_simple_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Debug commands should work when debug mode is enabled
    let response = engine.process_input("debug test").expect("Failed");
    assert!(response.success);
    assert!(response.text.contains("Debug"));
    
    // Test with debug disabled
    let mut engine_no_debug = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        debug: false,
        ..Default::default()
    });
    
    engine_no_debug.load_script(&script).expect("Failed to load script");
    engine_no_debug.start().expect("Failed to start game");
    
    // Debug commands should fail when debug mode is disabled
    let result = engine_no_debug.process_input("debug test");
    assert!(result.is_err());
}

// Helper function to create test scripts
fn create_simple_test_script() -> String {
    r#"
TextAdventure((
    title: "Test Game",
    author: "Test",
    description: Some("A simple test game"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "start",
    
    locations: {
        "start": (
            name: "Test Room",
            description: "A simple test room.",
            exits: {},
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {},
    characters: {},
    vocabulary: None,
    events: None,
))
    "#.to_string()
}

fn create_test_script_with_items(items: &[(&str, &str, &str)]) -> String {
    let mut script = String::from(r#"
TextAdventure((
    title: "Test Game",
    author: "Test",
    description: Some("A test game with items"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "start",
    
    locations: {
        "start": (
            name: "Test Room",
            description: "A room with test items.",
            exits: {},
            items: ["#);
    
    // Add item IDs to location
    for (i, (id, _, _)) in items.iter().enumerate() {
        if i > 0 { script.push_str(", "); }
        script.push_str(&format!("\"{}\"", id));
    }
    
    script.push_str(r#"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
"#);
    
    // Add item definitions
    for (id, name, description) in items {
        script.push_str(&format!(r#"        "{}": (
            id: "{}",
            name: Some("{}"),
            description: Some("{}"),
            location: Some("start"),
            takeable: true,
            weight: 1.0,
            is_container: false,
            contains: [],
            on_take: [],
            on_use: [],
            on_examine: [],
            properties: {{}},
        ),
"#, id, id, name, description));
    }
    
    script.push_str(r#"},
    characters: {},
    vocabulary: None,
    events: None,
))
    "#);
    
    script
}