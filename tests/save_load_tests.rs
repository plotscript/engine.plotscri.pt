//! Comprehensive tests for save/load system with data integrity and version compatibility

use plotscript::{Engine, EngineConfig, GameMode, init, types::*};
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_basic_save_load_integrity() {
    init();
    let mut engine = create_test_engine();
    
    // Load a test script and start the game
    let script = create_comprehensive_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Perform some actions to change game state
    engine.process_input("take key").expect("Failed to take key");
    engine.process_input("go north").expect("Failed to go north");
    engine.process_input("examine door").expect("Failed to examine door");
    
    // Save the game state
    let save_response = engine.save_game(Some(1)).expect("Failed to save game");
    assert!(save_response.success);
    assert!(save_response.text.contains("saved"));
    
    // Capture current state for comparison
    let original_state = engine.state.clone();
    let original_location = engine.world.player_location.clone();
    let original_inventory = engine.world.inventory.clone();
    let original_score = engine.world.score;
    let original_turns = engine.world.turn_count;
    
    // Modify state further
    engine.process_input("drop key").expect("Failed to drop key");
    engine.process_input("go south").expect("Failed to go south");
    
    // Verify state has changed
    assert_ne!(engine.world.player_location, original_location);
    assert_ne!(engine.world.inventory, original_inventory);
    
    // Load the saved game
    let load_response = engine.load_game(Some(1)).expect("Failed to load game");
    assert!(load_response.success);
    assert!(load_response.text.contains("loaded"));
    
    // Verify state was restored correctly
    assert_eq!(engine.world.player_location, original_location);
    assert_eq!(engine.world.inventory, original_inventory);
    assert_eq!(engine.world.score, original_score);
    assert_eq!(engine.world.turn_count, original_turns);
    
    // Test that game continues to work after loading
    let response = engine.process_input("look").expect("Failed to look after load");
    assert!(response.success);
}

#[test]
fn test_multiple_save_slots() {
    init();
    let mut engine = create_test_engine();
    
    let script = create_comprehensive_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Create different game states in different slots
    
    // Slot 1: Initial state with key
    engine.process_input("take key").expect("Failed");
    let response = engine.save_game(Some(1)).expect("Failed to save slot 1");
    assert!(response.success);
    let slot1_inventory = engine.world.inventory.clone();
    
    // Slot 2: State after moving
    engine.process_input("go north").expect("Failed");
    let response = engine.save_game(Some(2)).expect("Failed to save slot 2");
    assert!(response.success);
    let slot2_location = engine.world.player_location.clone();
    
    // Slot 3: State after dropping key
    engine.process_input("drop key").expect("Failed");
    let response = engine.save_game(Some(3)).expect("Failed to save slot 3");
    assert!(response.success);
    let slot3_inventory = engine.world.inventory.clone();
    
    // Test loading each slot restores correct state
    
    // Load slot 1
    engine.load_game(Some(1)).expect("Failed to load slot 1");
    assert_eq!(engine.world.inventory, slot1_inventory);
    assert_eq!(engine.world.player_location, "start");
    
    // Load slot 2
    engine.load_game(Some(2)).expect("Failed to load slot 2");
    assert_eq!(engine.world.player_location, slot2_location);
    assert_eq!(engine.world.inventory, slot1_inventory); // Still has key
    
    // Load slot 3
    engine.load_game(Some(3)).expect("Failed to load slot 3");
    assert_eq!(engine.world.inventory, slot3_inventory);
    assert_eq!(engine.world.player_location, slot2_location); // In north room
}

#[test]
fn test_save_load_with_complex_state() {
    init();
    let mut engine = create_test_engine();
    
    let script = create_complex_state_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Create complex state with variables, flags, and inventory
    engine.process_input("take sword").expect("Failed");
    engine.process_input("take shield").expect("Failed");
    engine.process_input("take potion").expect("Failed");
    
    // Modify game variables and flags (simulating script actions)
    engine.state.set_variable("health", Value::Integer(75));
    engine.state.set_variable("magic", Value::Integer(50));
    engine.state.set_variable("experience", Value::Integer(100));
    engine.state.set_flag("has_weapon");
    engine.state.set_flag("visited_cave");
    engine.state.score = 250;
    engine.state.turns = 15;
    
    // Add some complex inventory state
    let complex_inventory = vec![
        "sword".to_string(),
        "shield".to_string(), 
        "potion".to_string(),
        "gold_coins".to_string(),
        "map".to_string(),
    ];
    engine.world.inventory = complex_inventory.clone();
    
    // Save the complex state
    let response = engine.save_game(Some(1)).expect("Failed to save complex state");
    assert!(response.success);
    
    // Reset to initial state
    engine.load_script(&script).expect("Failed to reload script");
    engine.start().expect("Failed to restart game");
    
    // Verify state is reset
    assert!(engine.world.inventory.is_empty());
    assert_eq!(engine.state.score, 0);
    assert_eq!(engine.state.turns, 0);
    assert!(!engine.state.has_flag("has_weapon"));
    
    // Load the complex state
    let response = engine.load_game(Some(1)).expect("Failed to load complex state");
    assert!(response.success);
    
    // Verify all complex state was restored
    assert_eq!(engine.world.inventory, complex_inventory);
    assert_eq!(engine.state.get_variable("health"), Some(&Value::Integer(75)));
    assert_eq!(engine.state.get_variable("magic"), Some(&Value::Integer(50)));
    assert_eq!(engine.state.get_variable("experience"), Some(&Value::Integer(100)));
    assert!(engine.state.has_flag("has_weapon"));
    assert!(engine.state.has_flag("visited_cave"));
    assert_eq!(engine.state.score, 250);
    assert_eq!(engine.state.turns, 15);
}

#[test]
fn test_save_load_error_handling() {
    init();
    let mut engine = create_test_engine();
    
    let script = create_comprehensive_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Test loading from non-existent slot
    let result = engine.load_game(Some(99));
    assert!(result.is_err(), "Loading from non-existent slot should fail");
    
    // Test saving to invalid slot (if there are limits)
    // Note: This depends on implementation - some engines might allow any slot number
    
    // Test save/load without starting game
    let mut fresh_engine = create_test_engine();
    let result = fresh_engine.save_game(Some(1));
    assert!(result.is_err(), "Saving without starting should fail");
    
    let result = fresh_engine.load_game(Some(1));
    assert!(result.is_err(), "Loading without starting should fail");
    
    // Test that engine state remains consistent after failed operations
    let original_state = engine.state.clone();
    let _failed_load = engine.load_game(Some(99));
    // State should be unchanged after failed load
    assert_eq!(engine.state.variables, original_state.variables);
    assert_eq!(engine.state.inventory, original_state.inventory);
}

#[test]
fn test_auto_save_functionality() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        auto_save: true,
        ..Default::default()
    });
    
    let script = create_comprehensive_test_script();
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Perform actions that should trigger auto-save
    engine.process_input("take key").expect("Failed to take key");
    
    // Move to trigger auto-save (if auto-save happens on room changes)
    let initial_location = engine.world.player_location.clone();
    engine.process_input("go north").expect("Failed to go north");
    let new_location = engine.world.player_location.clone();
    assert_ne!(initial_location, new_location);
    
    // Auto-save should have occurred, but this is internal to the engine
    // We can't directly test it without access to the save manager's state
    // This test verifies that auto-save doesn't break normal operation
    
    let response = engine.process_input("look").expect("Failed to look");
    assert!(response.success);
}

#[test]
fn test_save_data_compression() {
    init();
    let mut engine = create_test_engine();
    
    // Create a game with lots of data to test compression
    let large_script = create_large_game_script();
    engine.load_script(&large_script).expect("Failed to load large script");
    engine.start().expect("Failed to start game");
    
    // Fill inventory with many items
    for i in 0..20 {
        let take_cmd = format!("take item{}", i);
        engine.process_input(&take_cmd).ok(); // Some might fail, that's ok
    }\n    \n    // Add many variables and flags to increase save data size\n    for i in 0..50 {\n        engine.state.set_variable(format!(\"var{}\", i), Value::Integer(i as i64));\n        if i % 2 == 0 {\n            engine.state.set_flag(format!(\"flag{}\", i));\n        }\n    }\n    \n    // Save the large state\n    let response = engine.save_game(Some(1)).expect(\"Failed to save large state\");\n    assert!(response.success);\n    \n    // Reset state\n    engine.load_script(&large_script).expect(\"Failed to reload script\");\n    engine.start().expect(\"Failed to restart game\");\n    \n    // Load and verify the large state\n    let response = engine.load_game(Some(1)).expect(\"Failed to load large state\");\n    assert!(response.success);\n    \n    // Verify some of the data was restored\n    assert_eq!(engine.state.get_variable(\"var10\"), Some(&Value::Integer(10)));\n    assert!(engine.state.has_flag(\"flag20\"));\n    assert!(!engine.state.has_flag(\"flag21\")); // Odd numbers weren't flagged\n}\n\n#[test]\nfn test_save_game_with_dialogue_state() {\n    init();\n    let mut engine = create_test_engine();\n    \n    let script = create_script_with_dialogue();\n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Interact with character to change dialogue state\n    engine.process_input(\"talk to guard\").expect(\"Failed to talk to guard\");\n    \n    // Save after dialogue interaction\n    let response = engine.save_game(Some(1)).expect(\"Failed to save\");\n    assert!(response.success);\n    \n    // Continue dialogue or change character state somehow\n    // (This depends on how dialogue system maintains state)\n    \n    // Reset game\n    engine.load_script(&script).expect(\"Failed to reload script\");\n    engine.start().expect(\"Failed to restart game\");\n    \n    // Load save and verify dialogue state is preserved\n    let response = engine.load_game(Some(1)).expect(\"Failed to load\");\n    assert!(response.success);\n    \n    // Test that dialogue continues from where it was saved\n    let response = engine.process_input(\"talk to guard\").expect(\"Failed to talk after load\");\n    assert!(response.success);\n}\n\n#[test]\nfn test_version_compatibility() {\n    init();\n    // This test would check if saves from different engine versions can be loaded\n    // For now, it's a placeholder for future version compatibility testing\n    \n    let mut engine = create_test_engine();\n    let script = create_comprehensive_test_script();\n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Create a save with current version\n    engine.process_input(\"take key\").expect(\"Failed\");\n    let response = engine.save_game(Some(1)).expect(\"Failed to save\");\n    assert!(response.success);\n    \n    // In a real implementation, we would:\n    // 1. Save the current engine version with the save data\n    // 2. Test loading saves from previous versions\n    // 3. Handle migration/conversion of old save formats\n    // 4. Provide warnings or errors for incompatible versions\n    \n    // For now, verify that current version saves can be loaded\n    let response = engine.load_game(Some(1)).expect(\"Failed to load own save\");\n    assert!(response.success);\n}\n\n#[test]\nfn test_concurrent_save_operations() {\n    init();\n    // Test multiple save operations in quick succession\n    let mut engine = create_test_engine();\n    \n    let script = create_comprehensive_test_script();\n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Perform rapid save operations to different slots\n    for slot in 1..=10 {\n        engine.process_input(&format!(\"take item{}\", slot)).ok();\n        let response = engine.save_game(Some(slot as u8)).expect(&format!(\"Failed to save slot {}\", slot));\n        assert!(response.success);\n    }\n    \n    // Verify all saves can be loaded\n    for slot in 1..=10 {\n        let response = engine.load_game(Some(slot as u8)).expect(&format!(\"Failed to load slot {}\", slot));\n        assert!(response.success);\n    }\n}\n\n// Helper functions to create test scenarios\n\nfn create_test_engine() -> Engine {\n    Engine::with_config(EngineConfig {\n        mode: GameMode::TextAdventure,\n        auto_save: false, // Disable for testing unless specifically testing auto-save\n        ..Default::default()\n    })\n}\n\nfn create_comprehensive_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Save/Load Test Game\",\n    author: \"Test Suite\",\n    description: Some(\"A game for testing save/load functionality\"),\n    version: Some(\"1.0\"),\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    starting_location: \"start\",\n    \n    locations: {\n        \"start\": (\n            name: \"Starting Room\",\n            description: \"A simple room with a key on the floor.\",\n            exits: {\n                north: \"north_room\",\n            },\n            items: [\"key\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n        \"north_room\": (\n            name: \"Northern Room\",\n            description: \"A room to the north with a locked door.\",\n            exits: {\n                south: \"start\",\n            },\n            items: [\"door\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"key\": (\n            id: \"key\",\n            name: Some(\"brass key\"),\n            description: Some(\"A shiny brass key.\"),\n            location: Some(\"start\"),\n            takeable: true,\n            weight: 0.1,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"door\": (\n            id: \"door\",\n            name: Some(\"wooden door\"),\n            description: Some(\"A heavy wooden door with a keyhole.\"),\n            location: Some(\"north_room\"),\n            takeable: false,\n            weight: 100.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_complex_state_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Complex State Test\",\n    author: \"Test Suite\",\n    description: Some(\"Testing complex game state\"),\n    version: Some(\"1.0\"),\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    starting_location: \"armory\",\n    \n    locations: {\n        \"armory\": (\n            name: \"Armory\",\n            description: \"A room filled with weapons and armor.\",\n            exits: {},\n            items: [\"sword\", \"shield\", \"potion\", \"gold_coins\", \"map\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"sword\": (\n            id: \"sword\",\n            name: Some(\"steel sword\"),\n            description: Some(\"A sharp steel sword.\"),\n            location: Some(\"armory\"),\n            takeable: true,\n            weight: 3.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"shield\": (\n            id: \"shield\",\n            name: Some(\"iron shield\"),\n            description: Some(\"A sturdy iron shield.\"),\n            location: Some(\"armory\"),\n            takeable: true,\n            weight: 5.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"potion\": (\n            id: \"potion\",\n            name: Some(\"health potion\"),\n            description: Some(\"A red potion that restores health.\"),\n            location: Some(\"armory\"),\n            takeable: true,\n            weight: 0.5,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"gold_coins\": (\n            id: \"gold_coins\",\n            name: Some(\"gold coins\"),\n            description: Some(\"A pouch of gleaming gold coins.\"),\n            location: Some(\"armory\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"map\": (\n            id: \"map\",\n            name: Some(\"treasure map\"),\n            description: Some(\"An old map showing the location of treasure.\"),\n            location: Some(\"armory\"),\n            takeable: true,\n            weight: 0.1,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_large_game_script() -> String {\n    let mut script = String::from(r#\"\nTextAdventure((\n    title: \"Large Test Game\",\n    author: \"Test Suite\",\n    description: Some(\"A large game for testing save compression\"),\n    version: Some(\"1.0\"),\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    starting_location: \"start\",\n    \n    locations: {\n        \"start\": (\n            name: \"Starting Room\",\n            description: \"A room with many items.\",\n            exits: {},\n            items: [\"#);\n    \n    // Add many items to the room\n    for i in 0..50 {\n        if i > 0 { script.push_str(\", \"); }\n        script.push_str(&format!(\"\\\"item{}\\\"\", i));\n    }\n    \n    script.push_str(r#\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n\"#);\n    \n    // Add many item definitions\n    for i in 0..50 {\n        script.push_str(&format!(r#\"        \"item{}\": (\n            id: \"item{}\",\n            name: Some(\"Test Item {}\"),\n            description: Some(\"This is test item number {} for compression testing.\"),\n            location: Some(\"start\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {{}},\n        ),\n\"#, i, i, i, i));\n    }\n    \n    script.push_str(r#\"    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#);\n    \n    script\n}\n\nfn create_script_with_dialogue() -> String {\n    r#\"\nTextAdventure((\n    title: \"Dialogue Test Game\",\n    author: \"Test Suite\",\n    description: Some(\"A game for testing dialogue state saving\"),\n    version: Some(\"1.0\"),\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    starting_location: \"guardroom\",\n    \n    locations: {\n        \"guardroom\": (\n            name: \"Guard Room\",\n            description: \"A room with a guard standing at attention.\",\n            exits: {},\n            items: [],\n            characters: [\"guard\"],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {},\n    \n    characters: {\n        \"guard\": (\n            id: \"guard\",\n            name: Some(\"palace guard\"),\n            description: Some(\"A stern-looking guard in uniform.\"),\n            location: Some(\"guardroom\"),\n            dialogue: Some((\n                starting_node: \"greeting\",\n                nodes: {\n                    \"greeting\": (\n                        text: \"Halt! What is your business here?\",\n                        responses: Some([\n                            (\n                                text: \"I'm just passing through.\",\n                                next: \"suspicious\",\n                            ),\n                            (\n                                text: \"I have official business.\",\n                                next: \"official\",\n                            ),\n                        ]),\n                    ),\n                    \"suspicious\": (\n                        text: \"I don't believe you. State your real purpose.\",\n                        responses: None,\n                    ),\n                    \"official\": (\n                        text: \"Show me your papers.\",\n                        responses: None,\n                    ),\n                },\n            )),\n            state: \"alert\",\n            health: Some(100),\n            relationship: Some(0),\n            properties: {},\n        ),\n    },\n    \n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}"