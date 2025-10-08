//! Comprehensive tests for error handling and edge cases across all engine components

use plotscript::{Engine, EngineConfig, GameMode, init, types::*, Error};
use std::collections::HashMap;

#[test]
fn test_engine_initialization_errors() {
    init();
    
    // Test creating engine without loading script
    let mut engine = Engine::new();
    
    // Should fail to start without script
    let result = engine.start();
    assert!(result.is_err(), "Starting without script should fail");
    match result {
        Err(Error::InvalidState(msg)) => assert!(msg.contains("script")),
        Err(_) => panic!("Expected InvalidState error"),
        _ => panic!("Should have failed"),
    }
    
    // Should fail to process input without starting
    let result = engine.process_input("look");
    assert!(result.is_err(), "Processing input without starting should fail");
}

#[test]
fn test_script_loading_errors() {
    init();
    let mut engine = Engine::new();
    
    // Test loading invalid RON
    let invalid_ron = r#"
    This is not valid RON syntax {[}]
    "#;
    
    let result = engine.load_script(invalid_ron);
    assert!(result.is_err(), "Invalid RON should fail to load");
    
    // Test loading malformed but syntactically valid RON
    let malformed_ron = r#"
    TextAdventure((
        title: "Test",
        // Missing required fields
    ))
    "#;
    
    let result = engine.load_script(malformed_ron);
    assert!(result.is_err(), "Malformed RON should fail to load");
    
    // Test loading empty script
    let result = engine.load_script("");
    assert!(result.is_err(), "Empty script should fail to load");
    
    // Test loading script with invalid enum values
    let invalid_enum_ron = r#"
    TextAdventure((
        title: "Test",
        author: "Test",
        starting_location: "start",
        settings: (
            parser_mode: InvalidMode,
            command_aliases: true,
            darkness_system: false,
            inventory_limits: false,
            max_inventory: None,
        ),
        locations: {},
        items: {},
        characters: {},
        vocabulary: None,
        events: None,
    ))
    "#;
    
    let result = engine.load_script(invalid_enum_ron);
    assert!(result.is_err(), "Invalid enum values should fail");
}

#[test]
fn test_world_consistency_errors() {
    init();
    let mut engine = Engine::new();
    
    // Test script with inconsistent references
    let inconsistent_script = r#"
    TextAdventure((
        title: "Inconsistent Test",
        author: "Test",
        starting_location: "nonexistent_room", // This room doesn't exist
        
        settings: (
            parser_mode: Natural,
            command_aliases: true,
            darkness_system: false,
            inventory_limits: false,
            max_inventory: None,
        ),
        
        locations: {
            "room1": (
                name: "Room 1",
                description: "A room.",
                exits: {
                    north: "nonexistent_room2", // This also doesn't exist
                },
                items: ["nonexistent_item"], // This item doesn't exist
                characters: ["nonexistent_character"], // This character doesn't exist
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
    
    let result = engine.load_script(inconsistent_script);
    // This might load but should fail on validation or when trying to start
    match result {
        Ok(_) => {
            // If loading succeeds, starting should fail
            let start_result = engine.start();
            assert!(start_result.is_err(), "Starting with inconsistent world should fail");
        }
        Err(_) => {
            // Loading failed, which is also acceptable
        }
    }
}

#[test]
fn test_movement_errors() {
    init();
    let mut engine = Engine::new();
    let script = create_movement_error_test_script();
    
    engine.load_script(&script).expect("Failed to load script");
    engine.start().expect("Failed to start game");
    
    // Test moving in invalid directions
    let invalid_movements = [
        "go north", // No north exit
        "go south", 
        "go east",
        "go west",
        "go up",
        "go down",
        "go northeast",
        "go northwest",
    ];
    
    for movement in invalid_movements {
        let result = engine.process_input(movement);
        match result {
            Ok(response) => {
                assert!(!response.success, "Invalid movement '{}' should fail", movement);
                assert!(response.text.contains("can't") || 
                        response.text.contains("no exit") ||
                        response.text.contains("blocked") ||
                        response.text.contains("way"),
                        "Error message should be informative for '{}'", movement);
            }
            Err(_) => {
                // Also acceptable for invalid movements to return errors
            }
        }\n    }\n    \n    // Test movement with typos that can't be corrected\n    let uncorrectable_movements = [\n        \"go xyz\", // Not close to any real direction\n        \"move qwerty\",\n        \"walk 12345\",\n    ];\n    \n    for movement in uncorrectable_movements {\n        let result = engine.process_input(movement);\n        match result {\n            Ok(response) => {\n                assert!(!response.success, \"Uncorrectable movement '{}' should fail\", movement);\n            }\n            Err(_) => {}\n        }\n    }\n}\n\n#[test]\nfn test_item_interaction_errors() {\n    init();\n    let mut engine = Engine::new();\n    let script = create_item_error_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Test taking items that don't exist\n    let nonexistent_items = [\n        \"take unicorn\",\n        \"get flying_car\", \n        \"pick up invisible_cloak\",\n        \"grab nonexistent_item\",\n    ];\n    \n    for command in nonexistent_items {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                assert!(!response.success, \"Taking nonexistent item should fail: {}\", command);\n                assert!(response.text.contains(\"don't see\") ||\n                        response.text.contains(\"not here\") ||\n                        response.text.contains(\"can't find\"),\n                        \"Error message should be clear for: {}\", command);\n            }\n            Err(Error::NotFound(_)) => {}, // Also acceptable\n            Err(_) => panic!(\"Unexpected error type for: {}\", command),\n        }\n    }\n    \n    // Test taking items that can't be taken\n    let untakeable_items = [\n        \"take mountain\",\n        \"get building\",\n        \"pick up immovable_object\",\n    ];\n    \n    for command in untakeable_items {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                assert!(!response.success, \"Taking untakeable item should fail: {}\", command);\n                assert!(response.text.contains(\"can't take\") ||\n                        response.text.contains(\"too heavy\") ||\n                        response.text.contains(\"fixed\") ||\n                        response.text.contains(\"immovable\"),\n                        \"Error message should explain why: {}\", command);\n            }\n            Err(_) => {}\n        }\n    }\n    \n    // Test using items not in inventory\n    let not_owned_items = [\n        \"use key\", // Key exists but not in inventory\n        \"use sword\",\n        \"use potion\",\n    ];\n    \n    for command in not_owned_items {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                // Some engines allow using items in the room, others don't\n                if !response.success {\n                    assert!(response.text.contains(\"don't have\") ||\n                            response.text.contains(\"not carrying\"),\n                            \"Error message should indicate possession issue: {}\", command);\n                }\n            }\n            Err(_) => {}\n        }\n    }\n    \n    // Test dropping items not in inventory\n    let result = engine.process_input(\"drop magical_sword\");\n    match result {\n        Ok(response) => {\n            assert!(!response.success, \"Dropping unowned item should fail\");\n        }\n        Err(Error::NotFound(_)) => {}, // Acceptable\n        Err(_) => {}\n    }\n}\n\n#[test]\nfn test_inventory_limit_errors() {\n    init();\n    let mut engine = Engine::with_config(EngineConfig {\n        mode: GameMode::TextAdventure,\n        max_inventory: 2, // Very small inventory for testing\n        ..Default::default()\n    });\n    \n    let script = create_inventory_limit_test_script();\n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Fill inventory to capacity\n    let response = engine.process_input(\"take item1\").expect(\"Failed\");\n    assert!(response.success, \"Should be able to take first item\");\n    \n    let response = engine.process_input(\"take item2\").expect(\"Failed\");\n    assert!(response.success, \"Should be able to take second item\");\n    \n    // Try to exceed capacity\n    let response = engine.process_input(\"take item3\").expect(\"Failed\");\n    assert!(!response.success, \"Should not be able to exceed inventory capacity\");\n    assert!(response.text.contains(\"carrying too much\") ||\n            response.text.contains(\"inventory full\") ||\n            response.text.contains(\"can't carry\"),\n            \"Error message should indicate inventory limit\");\n    \n    // Verify inventory count\n    let response = engine.process_input(\"inventory\").expect(\"Failed\");\n    assert!(response.success);\n    // Should show 2 items (at capacity)\n}\n\n#[test]\nfn test_parser_error_handling() {\n    init();\n    let mut engine = Engine::new();\n    let script = create_simple_error_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Test completely unrecognizable commands\n    let gibberish_commands = [\n        \"xyzabc\",\n        \"qwertyuiop\",\n        \"12345\",\n        \"@#$%^&*()\",\n        \".........\",\n        \"|||||||||\",\n    ];\n    \n    for command in gibberish_commands {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                assert!(!response.success, \"Gibberish command should fail: {}\", command);\n                assert!(response.text.contains(\"don't understand\") ||\n                        response.text.contains(\"not recognized\") ||\n                        response.text.contains(\"invalid\") ||\n                        response.text.contains(\"help\"),\n                        \"Should provide helpful error for: {}\", command);\n            }\n            Err(_) => {}, // Also acceptable\n        }\n    }\n    \n    // Test commands with mixed valid/invalid parts\n    let mixed_commands = [\n        \"take xyz123\", // Valid verb, invalid object\n        \"qwerty sword\", // Invalid verb, valid object\n        \"go nowhere\", // Valid verb, invalid direction\n        \"examine nothing\", // Valid verb, nonexistent object\n    ];\n    \n    for command in mixed_commands {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                assert!(!response.success, \"Mixed command should fail: {}\", command);\n                // Error message should be informative\n                assert!(!response.text.is_empty(), \"Should provide error message for: {}\", command);\n            }\n            Err(_) => {}\n        }\n    }\n}\n\n#[test]\nfn test_save_load_error_conditions() {\n    init();\n    let mut engine = Engine::new();\n    let script = create_simple_error_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Test loading from nonexistent save slot\n    let result = engine.load_game(Some(99));\n    assert!(result.is_err(), \"Loading from nonexistent slot should fail\");\n    match result {\n        Err(Error::NotFound(_)) => {}, // Expected\n        Err(Error::IOError(_)) => {}, // Also acceptable\n        Err(_) => panic!(\"Unexpected error type for nonexistent save\"),\n        _ => panic!(\"Should have failed\"),\n    }\n    \n    // Test saving to invalid slot (if there are restrictions)\n    // Note: This test depends on implementation - some engines allow any slot\n    let result = engine.save_game(Some(0)); // Slot 0 might be invalid\n    match result {\n        Ok(_) => {}, // If slot 0 is valid, that's fine\n        Err(_) => {}, // If slot 0 is invalid, that's also fine\n    }\n    \n    // Test save/load without game started\n    let mut fresh_engine = Engine::new();\n    \n    let result = fresh_engine.save_game(Some(1));\n    assert!(result.is_err(), \"Saving without started game should fail\");\n    \n    let result = fresh_engine.load_game(Some(1));\n    assert!(result.is_err(), \"Loading without started game should fail\");\n}\n\n#[test]\nfn test_extension_error_handling() {\n    init();\n    let mut engine = Engine::new();\n    \n    // Test unregistering extension that doesn't exist\n    let result = engine.unregister_extension(\"nonexistent_extension\");\n    assert!(result.is_err(), \"Unregistering nonexistent extension should fail\");\n    \n    // Test registering extension with duplicate name\n    struct DuplicateExtension {\n        name: String,\n    }\n    \n    impl plotscript::extensions::Extension for DuplicateExtension {\n        fn name(&self) -> &str {\n            &self.name\n        }\n        \n        fn get_verbs(&self) -> Vec<&str> {\n            vec![\"test\"]\n        }\n        \n        fn process_command(&mut self, _: &str, _: &[&str], _: &mut GameState) -> Option<Response> {\n            Some(Response {\n                text: \"Test response\".to_string(),\n                success: true,\n                ..Default::default()\n            })\n        }\n    }\n    \n    // Register first extension\n    let ext1 = Box::new(DuplicateExtension {\n        name: \"duplicate\".to_string(),\n    });\n    let result = engine.register_extension(ext1);\n    assert!(result.is_ok(), \"First extension registration should succeed\");\n    \n    // Try to register extension with same name\n    let ext2 = Box::new(DuplicateExtension {\n        name: \"duplicate\".to_string(),\n    });\n    let result = engine.register_extension(ext2);\n    // This should either fail or replace the existing extension\n    match result {\n        Ok(_) => {\n            // If it succeeds, there should still be only one extension with that name\n            assert!(engine.has_extension(\"duplicate\"));\n        }\n        Err(_) => {\n            // If it fails, the original extension should still be there\n            assert!(engine.has_extension(\"duplicate\"));\n        }\n    }\n}\n\n#[test]\nfn test_memory_and_resource_limits() {\n    init();\n    let mut engine = Engine::new();\n    \n    // Test with very large script content\n    let mut large_script = String::from(r#\"\nTextAdventure((\n    title: \"Large Test\",\n    author: \"Test Suite\",\n    starting_location: \"room_0\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    locations: {\n\"#);\n    \n    // Add many locations to test memory limits\n    for i in 0..1000 {\n        large_script.push_str(&format!(r#\"        \"room_{}\": (\n            name: \"Room {}\",\n            description: \"This is room number {}. It has a very long description that goes on and on and on to test memory usage and string handling capabilities of the engine. The description continues with more text to make it even longer and test various edge cases in text processing and storage.\",\n            exits: {{\n                north: \"room_{}\",\n            }},\n            items: [],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n\"#, i, i, i, (i + 1) % 1000));\n    }\n    \n    large_script.push_str(r#\"    },\n    items: {},\n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#);\n    \n    // Try to load the large script\n    let start_time = std::time::Instant::now();\n    let result = engine.load_script(&large_script);\n    let load_time = start_time.elapsed();\n    \n    match result {\n        Ok(_) => {\n            println!(\"Large script loaded successfully in {:?}\", load_time);\n            \n            // Try to start the game\n            let start_result = engine.start();\n            match start_result {\n                Ok(_) => {\n                    // Test performance with large world\n                    let perf_start = std::time::Instant::now();\n                    let _response = engine.process_input(\"look\");\n                    let perf_time = perf_start.elapsed();\n                    \n                    println!(\"Command processing with large world: {:?}\", perf_time);\n                    assert!(perf_time.as_millis() < 1000, \"Large world should still be responsive\");\n                }\n                Err(_) => {\n                    println!(\"Large world failed to start (acceptable)\");\n                }\n            }\n        }\n        Err(_) => {\n            println!(\"Large script failed to load (acceptable for resource limits)\");\n        }\n    }\n}\n\n#[test]\nfn test_concurrent_access_safety() {\n    init();\n    \n    // Test that engine handles rapid successive calls safely\n    let mut engine = Engine::new();\n    let script = create_simple_error_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Rapid fire commands\n    let commands = [\"look\", \"inventory\", \"help\", \"look\", \"inventory\"];\n    \n    for _ in 0..100 {\n        for command in &commands {\n            let result = engine.process_input(command);\n            match result {\n                Ok(response) => {\n                    // Response should be consistent\n                    assert!(!response.text.is_empty() || !response.success);\n                }\n                Err(_) => {\n                    // Errors should be consistent too\n                }\n            }\n        }\n    }\n}\n\n#[test]\nfn test_unicode_and_special_character_handling() {\n    init();\n    let mut engine = Engine::new();\n    let script = create_unicode_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Test commands with various Unicode characters\n    let unicode_commands = [\n        \"look 🗡️\", // Emoji\n        \"take café\", // Accented characters\n        \"examine 北京\", // Chinese characters\n        \"use λ\", // Greek letters\n        \"get item™\", // Special symbols\n        \"look at \\\"quoted item\\\"\", // Quoted strings\n        \"take item\\nwith\\nnewlines\", // Control characters\n    ];\n    \n    for command in unicode_commands {\n        let result = engine.process_input(command);\n        match result {\n            Ok(response) => {\n                // Should handle Unicode gracefully, even if command fails\n                assert!(!response.text.contains(\"panic\") && \n                        !response.text.contains(\"error\"),\n                        \"Unicode command should be handled gracefully: {}\", command);\n            }\n            Err(_) => {\n                // Errors are acceptable for Unicode commands\n            }\n        }\n    }\n}\n\n#[test]\nfn test_malformed_input_edge_cases() {\n    init();\n    let mut engine = Engine::new();\n    let script = create_simple_error_test_script();\n    \n    engine.load_script(&script).expect(\"Failed to load script\");\n    engine.start().expect(\"Failed to start game\");\n    \n    // Test various malformed inputs\n    let malformed_inputs = [\n        \"\\0\", // Null character\n        \"\\x01\\x02\\x03\", // Control characters\n        \"take\\x00item\", // Embedded null\n        \"\\u{FEFF}look\", // BOM character\n        \"\\u{202E}take key\\u{202C}\", // Right-to-left override\n        \"\\r\\n\\r\\n\", // Just line endings\n        \"\\t\\t\\t\", // Just tabs\n    ];\n    \n    for input in malformed_inputs {\n        let result = engine.process_input(input);\n        // Should not panic or crash, regardless of success/failure\n        match result {\n            Ok(_) => {}, // Handled gracefully\n            Err(_) => {}, // Also acceptable\n        }\n    }\n}\n\n// Helper functions to create test scenarios\n\nfn create_movement_error_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Movement Error Test\",\n    author: \"Test Suite\",\n    starting_location: \"isolated_room\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    locations: {\n        \"isolated_room\": (\n            name: \"Isolated Room\",\n            description: \"A room with no exits - you are trapped!\",\n            exits: {}, // No exits!\n            items: [],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {},\n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_item_error_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Item Error Test\",\n    author: \"Test Suite\",\n    starting_location: \"test_room\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    locations: {\n        \"test_room\": (\n            name: \"Test Room\",\n            description: \"A room with some takeable and non-takeable items.\",\n            exits: {},\n            items: [\"key\", \"sword\", \"potion\", \"mountain\", \"building\", \"immovable_object\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"key\": (\n            id: \"key\",\n            name: Some(\"brass key\"),\n            description: Some(\"A small brass key.\"),\n            location: Some(\"test_room\"),\n            takeable: true,\n            weight: 0.1,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"sword\": (\n            id: \"sword\",\n            name: Some(\"steel sword\"),\n            description: Some(\"A sharp steel sword.\"),\n            location: Some(\"test_room\"),\n            takeable: true,\n            weight: 3.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"potion\": (\n            id: \"potion\",\n            name: Some(\"healing potion\"),\n            description: Some(\"A red healing potion.\"),\n            location: Some(\"test_room\"),\n            takeable: true,\n            weight: 0.5,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"mountain\": (\n            id: \"mountain\",\n            name: Some(\"towering mountain\"),\n            description: Some(\"A massive mountain - clearly immovable.\"),\n            location: Some(\"test_room\"),\n            takeable: false, // Can't take a mountain!\n            weight: 1000000.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"building\": (\n            id: \"building\",\n            name: Some(\"large building\"),\n            description: Some(\"A large building - too big to carry.\"),\n            location: Some(\"test_room\"),\n            takeable: false,\n            weight: 500000.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"immovable_object\": (\n            id: \"immovable_object\",\n            name: Some(\"immovable object\"),\n            description: Some(\"An object that cannot be moved by definition.\"),\n            location: Some(\"test_room\"),\n            takeable: false,\n            weight: 999999.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_inventory_limit_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Inventory Limit Test\",\n    author: \"Test Suite\",\n    starting_location: \"item_room\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: true,\n        max_inventory: Some(2),\n    ),\n    \n    locations: {\n        \"item_room\": (\n            name: \"Item Room\",\n            description: \"A room full of items to test inventory limits.\",\n            exits: {},\n            items: [\"item1\", \"item2\", \"item3\", \"item4\", \"item5\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"item1\": (\n            id: \"item1\",\n            name: Some(\"first item\"),\n            description: Some(\"The first test item.\"),\n            location: Some(\"item_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"item2\": (\n            id: \"item2\",\n            name: Some(\"second item\"),\n            description: Some(\"The second test item.\"),\n            location: Some(\"item_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"item3\": (\n            id: \"item3\",\n            name: Some(\"third item\"),\n            description: Some(\"The third test item.\"),\n            location: Some(\"item_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"item4\": (\n            id: \"item4\",\n            name: Some(\"fourth item\"),\n            description: Some(\"The fourth test item.\"),\n            location: Some(\"item_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"item5\": (\n            id: \"item5\",\n            name: Some(\"fifth item\"),\n            description: Some(\"The fifth test item.\"),\n            location: Some(\"item_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_simple_error_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Error Test\",\n    author: \"Test Suite\",\n    starting_location: \"simple_room\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    locations: {\n        \"simple_room\": (\n            name: \"Simple Room\",\n            description: \"A simple room for error testing.\",\n            exits: {},\n            items: [\"sword\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"sword\": (\n            id: \"sword\",\n            name: Some(\"test sword\"),\n            description: Some(\"A sword for testing.\"),\n            location: Some(\"simple_room\"),\n            takeable: true,\n            weight: 2.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}\n\nfn create_unicode_test_script() -> String {\n    r#\"\nTextAdventure((\n    title: \"Unicode Test 🎮\",\n    author: \"Test Suite 测试\",\n    starting_location: \"unicode_room\",\n    \n    settings: (\n        parser_mode: Natural,\n        command_aliases: true,\n        darkness_system: false,\n        inventory_limits: false,\n        max_inventory: None,\n    ),\n    \n    locations: {\n        \"unicode_room\": (\n            name: \"Unicode Room 🏠\",\n            description: \"A room with Unicode items: café, 北京, λ-calculus.\",\n            exits: {},\n            items: [\"cafe\", \"beijing\", \"lambda\"],\n            characters: [],\n            dark: Some(false),\n            first_visit: None,\n            events: None,\n        ),\n    },\n    \n    items: {\n        \"cafe\": (\n            id: \"cafe\",\n            name: Some(\"café ☕\"),\n            description: Some(\"A French café with accented characters.\"),\n            location: Some(\"unicode_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"beijing\": (\n            id: \"beijing\",\n            name: Some(\"北京\"),\n            description: Some(\"The name of China's capital in Chinese characters.\"),\n            location: Some(\"unicode_room\"),\n            takeable: true,\n            weight: 1.0,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n        \"lambda\": (\n            id: \"lambda\",\n            name: Some(\"λ symbol\"),\n            description: Some(\"A Greek lambda symbol used in mathematics.\"),\n            location: Some(\"unicode_room\"),\n            takeable: true,\n            weight: 0.1,\n            is_container: false,\n            contains: [],\n            on_take: [],\n            on_use: [],\n            on_examine: [],\n            properties: {},\n        ),\n    },\n    \n    characters: {},\n    vocabulary: None,\n    events: None,\n))\n    \"#.to_string()\n}"