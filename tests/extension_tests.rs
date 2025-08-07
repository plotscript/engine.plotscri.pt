//! Tests for the extension system

use plotscript::{
    Engine, EngineConfig, GameMode, init,
    extensions::{Extension, FunctionCondition, FunctionAction},
    Response,
    types::{GameState, Value},
};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

/// Test extension that adds simple commands
struct TestExtension {
    call_count: RwLock<usize>,
}

impl TestExtension {
    fn new() -> Self {
        Self {
            call_count: RwLock::new(0),
        }
    }
}

impl Extension for TestExtension {
    fn name(&self) -> &str {
        "test"
    }
    
    fn get_verbs(&self) -> Vec<&str> {
        vec!["test", "count"]
    }
    
    fn process_command(&mut self, command: &str, args: &[&str], _state: &mut GameState) -> Option<Response> {
        *self.call_count.write().unwrap() += 1;
        
        match command {
            "test" => {
                Some(Response {
                    text: format!("Test command executed with args: {:?}", args),
                    success: true,
                    ..Default::default()
                })
            }
            "count" => {
                let count = self.call_count.read().unwrap();
                Some(Response {
                    text: format!("Extension called {} times", *count),
                    success: true,
                    ..Default::default()
                })
            }
            _ => None,
        }
    }
}

#[test]
fn test_extension_registration() {
    init();
    
    let mut engine = Engine::new();
    let extension = Box::new(TestExtension::new());
    
    // Register extension
    assert!(engine.register_extension(extension).is_ok());
    assert!(engine.has_extension("test"));
    
    // Check metadata
    let extensions = engine.list_extensions();
    assert_eq!(extensions.len(), 1);
    assert_eq!(extensions[0].name, "test");
}

#[test]
fn test_extension_commands() {
    init();
    
    let game_script = r#"
    TextAdventure((
        title: "Test Game",
        author: "Test",
        description: Some("Test"),
        version: Some("1.0.0"),
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
                description: "A test room.",
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
    
    let mut engine = Engine::new();
    engine.load_script(game_script).unwrap();
    engine.start().unwrap();
    
    // Register extension
    let extension = Box::new(TestExtension::new());
    engine.register_extension(extension).unwrap();
    
    // Test custom command
    let response = engine.process_input("test hello world").unwrap();
    assert!(response.success);
    assert!(response.text.contains("hello"));
    assert!(response.text.contains("world"));
    
    // Test count command
    let response = engine.process_input("count").unwrap();
    assert!(response.text.contains("2 times")); // test + count = 2
}

#[test]
fn test_custom_conditions() {
    init();
    
    let mut engine = Engine::new();
    
    // Register a custom condition
    let condition = Box::new(FunctionCondition::new(
        "has_item",
        1,
        |state: &GameState, args: &[Value]| {
            if let Some(Value::String(item)) = args.first() {
                Ok(state.has_flag(&format!("has_{}", item)))
            } else {
                Ok(false)
            }
        },
    ));
    
    assert!(engine.register_condition(condition).is_ok());
}

#[test]
fn test_custom_actions() {
    init();
    
    let mut engine = Engine::new();
    
    // Register a custom action
    let action = Box::new(FunctionAction::new(
        "give_gold",
        1,
        |state: &mut GameState, args: &[Value]| {
            if let Some(Value::Integer(amount)) = args.first() {
                let current = match state.get_variable("gold") {
                    Some(Value::Integer(g)) => *g,
                    _ => 0,
                };
                state.set_variable("gold", Value::Integer(current + amount));
            }
            Ok(())
        },
    ));
    
    assert!(engine.register_action(action).is_ok());
}

#[test]
fn test_extension_state_modification() {
    init();
    
    // Extension that modifies game state
    struct StateModifierExtension;
    
    impl Extension for StateModifierExtension {
        fn name(&self) -> &str {
            "state_modifier"
        }
        
        fn get_verbs(&self) -> Vec<&str> {
            vec!["modify"]
        }
        
        fn process_command(&mut self, command: &str, _args: &[&str], state: &mut GameState) -> Option<Response> {
            if command == "modify" {
                state.set_variable("modified", Value::Bool(true));
                state.set_flag("extension_used");
                
                Some(Response {
                    text: "State modified!".to_string(),
                    success: true,
                    ..Default::default()
                })
            } else {
                None
            }
        }
    }
    
    let game_script = r#"
    TextAdventure((
        title: "Test Game",
        author: "Test",
        description: Some("Test"),
        version: Some("1.0.0"),
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
                description: "A test room.",
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
    
    let mut engine = Engine::new();
    engine.load_script(game_script).unwrap();
    engine.start().unwrap();
    
    // Register extension
    let extension = Box::new(StateModifierExtension);
    engine.register_extension(extension).unwrap();
    
    // Execute command
    let response = engine.process_input("modify").unwrap();
    assert!(response.success);
    
    // Check state was modified
    assert_eq!(engine.state.get_variable("modified"), Some(&Value::Bool(true)));
    assert!(engine.state.has_flag("extension_used"));
}

#[test]
fn test_extension_unregistration() {
    init();
    
    let mut engine = Engine::new();
    let extension = Box::new(TestExtension::new());
    
    // Register and verify
    engine.register_extension(extension).unwrap();
    assert!(engine.has_extension("test"));
    
    // Unregister and verify
    engine.unregister_extension("test").unwrap();
    assert!(!engine.has_extension("test"));
    assert!(engine.list_extensions().is_empty());
}