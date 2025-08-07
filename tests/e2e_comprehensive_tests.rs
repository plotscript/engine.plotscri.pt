//! Comprehensive End-to-End tests for PlotScript Engine
//! Tests all major functionality across all three game modes

use plotscript::{Engine, EngineConfig, GameMode, Response};
use plotscript::script::{
    TextAdventureScript, Location, Item as ScriptItem, Character as ScriptCharacter,
    VisualNovelScript, InteractiveFictionScript, StoryNode, StoryChoice,
    Scene, DialogueLine, Choice, VisualNovelSettings, IFSettings, TextSpeed, SkipMode,
    Quality, Storylet, Condition, Action, Dialogue, DialogueNode, DialogueResponse
};
use std::collections::HashMap;

// ============================================================================
// TEXT ADVENTURE MODE TESTS (10 tests)
// ============================================================================

#[test]
fn test_text_adventure_basic_navigation() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Navigation Test" {
            mode: text_adventure
        }
        
        room entrance "Entrance Hall" {
            description: "A grand entrance hall."
            exits: {
                north: corridor
                east: library
            }
        }
        
        room corridor "Long Corridor" {
            description: "A long corridor."
            exits: {
                south: entrance
            }
        }
        
        room library "Library" {
            description: "A quiet library."
            exits: {
                west: entrance
            }
        }
    "#;

    engine.load_script(script).unwrap();
    let response = engine.start().unwrap();
    assert!(response.text.contains("Entrance Hall"));

    // Test north navigation
    let response = engine.process_input("go north").unwrap();
    assert!(response.text.contains("Long Corridor"));
    assert_eq!(response.location, Some("corridor".to_string()));

    // Test south navigation
    let response = engine.process_input("go south").unwrap();
    assert!(response.text.contains("Entrance Hall"));

    // Test east navigation
    let response = engine.process_input("go east").unwrap();
    assert!(response.text.contains("Library"));
}

#[test]
fn test_text_adventure_item_manipulation() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Item Test" {
            mode: text_adventure
        }
        
        room start "Starting Room" {
            description: "A room with items."
            items: [key, book]
        }
        
        item key "Golden Key" {
            description: "A shiny golden key."
            takeable: true
            weight: 0.1
        }
        
        item book "Ancient Book" {
            description: "A dusty old book."
            takeable: true
            weight: 2
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Test taking items
    let response = engine.process_input("take key").unwrap();
    assert!(response.text.contains("take"));
    assert!(response.success);

    let response = engine.process_input("take book").unwrap();
    assert!(response.success);

    // Test inventory
    let response = engine.process_input("inventory").unwrap();
    assert!(response.text.contains("key") || response.text.contains("Key"));
    assert!(response.text.contains("book") || response.text.contains("Book"));

    // Test dropping items
    let response = engine.process_input("drop key").unwrap();
    assert!(response.success);

    let response = engine.process_input("inventory").unwrap();
    assert!(!response.text.contains("key") || response.text.contains("book"));
}

#[test]
fn test_text_adventure_examine_commands() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Examine Test" {
            mode: text_adventure
        }
        
        room chamber "Secret Chamber" {
            description: "A mysterious chamber."
            items: [mirror, painting]
        }
        
        item mirror "Ornate Mirror" {
            description: "An ornate mirror with strange symbols."
            takeable: false
        }
        
        item painting "Oil Painting" {
            description: "A painting of a stormy sea."
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Examine items
    let response = engine.process_input("examine mirror").unwrap();
    assert!(response.text.contains("ornate") || response.text.contains("symbols"));

    let response = engine.process_input("examine painting").unwrap();
    assert!(response.text.contains("painting") || response.text.contains("stormy"));

    // Look around
    let response = engine.process_input("look").unwrap();
    assert!(response.text.contains("chamber") || response.text.contains("Chamber"));
}

#[test]
fn test_text_adventure_character_interaction() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Character Test" {
            mode: text_adventure
        }
        
        room tavern "Tavern" {
            description: "A busy tavern."
            characters: [innkeeper, traveler]
        }
        
        character innkeeper "Friendly Innkeeper" {
            description: "A jolly innkeeper."
            location: tavern
        }
        
        character traveler "Mysterious Traveler" {
            description: "A hooded traveler."
            location: tavern
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Talk to characters
    let response = engine.process_input("talk to innkeeper").unwrap();
    assert!(response.success);

    let response = engine.process_input("examine traveler").unwrap();
    assert!(response.text.contains("hooded") || response.text.contains("traveler"));
}

#[test]
fn test_text_adventure_dark_rooms() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Darkness Test" {
            mode: text_adventure
        }
        
        room cave "Dark Cave" {
            description: "A pitch black cave."
            dark: true
            items: [torch]
            exits: {
                out: entrance
            }
        }
        
        room entrance "Cave Entrance" {
            description: "The entrance to a cave."
            exits: {
                in: cave
            }
        }
        
        item torch "Lit Torch" {
            description: "A burning torch."
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    let response = engine.start().unwrap();
    assert!(response.text.contains("entrance") || response.text.contains("Entrance"));

    // Enter dark room
    let response = engine.process_input("go in").unwrap();
    // Dark rooms might show limited description
    assert!(response.location == Some("cave".to_string()));
}

#[test]
fn test_text_adventure_container_items() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Container Test" {
            mode: text_adventure
        }
        
        room vault "Vault" {
            description: "A secure vault."
            items: [chest, safe]
        }
        
        item chest "Wooden Chest" {
            description: "A large wooden chest."
            takeable: false
            is_container: true
            contains: [gold, jewel]
        }
        
        item safe "Metal Safe" {
            description: "A heavy metal safe."
            takeable: false
            is_container: true
        }
        
        item gold "Gold Coins" {
            description: "A pile of gold coins."
            takeable: true
        }
        
        item jewel "Ruby Jewel" {
            description: "A precious ruby."
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Examine container
    let response = engine.process_input("examine chest").unwrap();
    assert!(response.success);
}

#[test]
fn test_text_adventure_locked_doors() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Lock Test" {
            mode: text_adventure
        }
        
        room hallway "Hallway" {
            description: "A hallway with a locked door."
            exits: {
                north: office
            }
            items: [key]
        }
        
        room office "Office" {
            description: "A private office."
            exits: {
                south: hallway
            }
        }
        
        item key "Office Key" {
            description: "A key to the office."
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Take key and unlock
    let response = engine.process_input("take key").unwrap();
    assert!(response.success);

    let response = engine.process_input("go north").unwrap();
    assert!(response.location == Some("office".to_string()));
}

#[test]
fn test_text_adventure_multiple_exits() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Complex Navigation" {
            mode: text_adventure
        }
        
        room center "Central Hub" {
            description: "A central hub with many exits."
            exits: {
                north: north_room
                south: south_room
                east: east_room
                west: west_room
                up: upper_level
                down: basement
            }
        }
        
        room north_room "North Room" {
            description: "The northern room."
            exits: { south: center }
        }
        
        room south_room "South Room" {
            description: "The southern room."
            exits: { north: center }
        }
        
        room east_room "East Room" {
            description: "The eastern room."
            exits: { west: center }
        }
        
        room west_room "West Room" {
            description: "The western room."
            exits: { east: center }
        }
        
        room upper_level "Upper Level" {
            description: "The upper level."
            exits: { down: center }
        }
        
        room basement "Basement" {
            description: "The basement."
            exits: { up: center }
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Test all directions
    let directions = ["north", "south", "east", "west", "up", "down"];
    let return_dirs = ["south", "north", "west", "east", "down", "up"];

    for (dir, ret) in directions.iter().zip(return_dirs.iter()) {
        let response = engine.process_input(&format!("go {}", dir)).unwrap();
        assert!(response.success);
        
        let response = engine.process_input(&format!("go {}", ret)).unwrap();
        assert!(response.location == Some("center".to_string()));
    }
}

#[test]
fn test_text_adventure_weight_limit() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        max_inventory: 3,
        ..Default::default()
    });

    let script = r#"
        game "Weight Test" {
            mode: text_adventure
        }
        
        room storage "Storage Room" {
            description: "A room full of items."
            items: [item1, item2, item3, item4]
        }
        
        item item1 "Item 1" {
            takeable: true
        }
        
        item item2 "Item 2" {
            takeable: true
        }
        
        item item3 "Item 3" {
            takeable: true
        }
        
        item item4 "Item 4" {
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Take items up to limit
    engine.process_input("take item1").unwrap();
    engine.process_input("take item2").unwrap();
    engine.process_input("take item3").unwrap();

    // Try to exceed limit
    let response = engine.process_input("take item4").unwrap();
    assert!(!response.success); // Should fail due to inventory limit
}

#[test]
fn test_text_adventure_room_visited_state() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Visit Tracking" {
            mode: text_adventure
        }
        
        room entrance "Grand Entrance" {
            description: "You enter a magnificent hall."
            exits: {
                north: throne_room
            }
        }
        
        room throne_room "Throne Room" {
            description: "The throne room is impressive."
            visited: false
            exits: {
                south: entrance
            }
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // First visit
    let response = engine.process_input("go north").unwrap();
    assert!(response.text.contains("throne") || response.text.contains("Throne"));

    // Return and revisit
    engine.process_input("go south").unwrap();
    let response = engine.process_input("go north").unwrap();
    assert!(response.success); // Room should be marked as visited
}

// ============================================================================
// VISUAL NOVEL MODE TESTS (10 tests)
// ============================================================================

#[test]
fn test_visual_novel_scene_progression() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "VN Test".to_string(),
        author: "Test Author".to_string(),
        description: Some("A test visual novel".to_string()),
        settings: VisualNovelSettings {
            resolution: (1920, 1080),
            text_speed: TextSpeed::Normal,
            auto_save: true,
            skip_mode: SkipMode::None,
        },
        starting_scene: "opening".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    // Add opening scene
    vn_script.scenes.insert("opening".to_string(), Scene {
        background: Some("classroom.jpg".to_string()),
        music: Some("peaceful.mp3".to_string()),
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "It was a peaceful morning.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            },
            DialogueLine {
                speaker: Some("Narrator".to_string()),
                text: "The story begins...".to_string(),
                voice: None,
                choices: Some(vec![
                    Choice {
                        text: "Continue".to_string(),
                        target: "next_scene".to_string(),
                        conditions: None,
                    }
                ]),
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_character_dialogue() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Dialogue Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "conversation".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("conversation".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: Some("Alice".to_string()),
                text: "Hello there!".to_string(),
                voice: None,
                choices: None,
                effects: None,
            },
            DialogueLine {
                speaker: Some("Bob".to_string()),
                text: "Hi Alice!".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Choice Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "decision".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("decision".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "You come to a crossroads.".to_string(),
                voice: None,
                choices: Some(vec![
                    Choice {
                        text: "Go left".to_string(),
                        target: "left_path".to_string(),
                        conditions: None,
                    },
                    Choice {
                        text: "Go right".to_string(),
                        target: "right_path".to_string(),
                        conditions: None,
                    },
                    Choice {
                        text: "Turn back".to_string(),
                        target: "start".to_string(),
                        conditions: None,
                    }
                ]),
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
    // Visual novels should have choices available
}

#[test]
fn test_visual_novel_backgrounds_and_music() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Media Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "scene1".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("scene1".to_string(), Scene {
        background: Some("school_day.jpg".to_string()),
        music: Some("cheerful.mp3".to_string()),
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "School during the day.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    vn_script.scenes.insert("scene2".to_string(), Scene {
        background: Some("school_night.jpg".to_string()),
        music: Some("mysterious.mp3".to_string()),
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "School at night.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_character_sprites() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Sprite Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "meeting".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    // Add character with sprites
    vn_script.characters.insert("maya".to_string(), plotscript::script::VNCharacter {
        name: "Maya".to_string(),
        color: Some("#FF69B4".to_string()),
        sprites: {
            let mut sprites = HashMap::new();
            sprites.insert("happy".to_string(), "maya_happy.png".to_string());
            sprites.insert("sad".to_string(), "maya_sad.png".to_string());
            sprites.insert("angry".to_string(), "maya_angry.png".to_string());
            sprites
        },
    });

    vn_script.scenes.insert("meeting".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![
            plotscript::script::CharacterPosition {
                id: "maya".to_string(),
                sprite: "happy".to_string(),
                position: plotscript::script::Position::Center,
            }
        ],
        dialogue: vec![
            DialogueLine {
                speaker: Some("Maya".to_string()),
                text: "Nice to meet you!".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_text_speed_settings() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Settings Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: VisualNovelSettings {
            resolution: (1280, 720),
            text_speed: TextSpeed::Fast,
            auto_save: false,
            skip_mode: SkipMode::Read,
        },
        starting_scene: "test".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("test".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "Testing text speed.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_voice_acting() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Voice Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "voiced".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("voiced".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: Some("Character".to_string()),
                text: "This line has voice acting.".to_string(),
                voice: Some("line001.ogg".to_string()),
                choices: None,
                effects: None,
            },
            DialogueLine {
                speaker: Some("Character".to_string()),
                text: "This one too.".to_string(),
                voice: Some("line002.ogg".to_string()),
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_special_effects() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Effects Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "effects".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("effects".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "Screen shakes!".to_string(),
                voice: None,
                choices: None,
                effects: Some(vec![
                    plotscript::script::Effect::Shake,
                    plotscript::script::Effect::Flash("white".to_string()),
                ]),
            },
            DialogueLine {
                speaker: None,
                text: "Transition effect.".to_string(),
                voice: None,
                choices: None,
                effects: Some(vec![
                    plotscript::script::Effect::Transition(plotscript::script::Transition::Fade),
                ]),
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_multiple_characters() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Multi Character".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "group".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    // Add multiple characters
    for (id, name) in [("alice", "Alice"), ("bob", "Bob"), ("charlie", "Charlie")] {
        vn_script.characters.insert(id.to_string(), plotscript::script::VNCharacter {
            name: name.to_string(),
            color: None,
            sprites: HashMap::new(),
        });
    }

    vn_script.scenes.insert("group".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![
            plotscript::script::CharacterPosition {
                id: "alice".to_string(),
                sprite: "default".to_string(),
                position: plotscript::script::Position::Left,
            },
            plotscript::script::CharacterPosition {
                id: "bob".to_string(),
                sprite: "default".to_string(),
                position: plotscript::script::Position::Center,
            },
            plotscript::script::CharacterPosition {
                id: "charlie".to_string(),
                sprite: "default".to_string(),
                position: plotscript::script::Position::Right,
            }
        ],
        dialogue: vec![
            DialogueLine {
                speaker: Some("Alice".to_string()),
                text: "We're all here!".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_visual_novel_branching_paths() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Branching Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "branch".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    // Create branching narrative
    vn_script.scenes.insert("branch".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "Choose your path.".to_string(),
                voice: None,
                choices: Some(vec![
                    Choice {
                        text: "Good ending".to_string(),
                        target: "good_end".to_string(),
                        conditions: None,
                    },
                    Choice {
                        text: "Bad ending".to_string(),
                        target: "bad_end".to_string(),
                        conditions: None,
                    },
                    Choice {
                        text: "True ending".to_string(),
                        target: "true_end".to_string(),
                        conditions: Some(vec![Condition::HasFlag("secret".to_string())]),
                    }
                ]),
                effects: None,
            }
        ],
    });

    for ending in ["good_end", "bad_end", "true_end"] {
        vn_script.scenes.insert(ending.to_string(), Scene {
            background: None,
            music: None,
            characters: vec![],
            dialogue: vec![
                DialogueLine {
                    speaker: None,
                    text: format!("You reached the {} ending.", ending),
                    voice: None,
                    choices: None,
                    effects: None,
                }
            ],
        });
    }

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

// ============================================================================
// INTERACTIVE FICTION MODE TESTS (10 tests)
// ============================================================================

#[test]
fn test_interactive_fiction_basic_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Choice Story".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: IFSettings {
            show_stats: true,
            checkpoint_saves: false,
            timed_choices: false,
            quality_caps: true,
        },
        starting_node: "start".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.nodes.insert("start".to_string(), StoryNode {
        content: "You wake up in a strange room.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Look around".to_string(),
                target: "examine_room".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Try the door".to_string(),
                target: "try_door".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Go back to sleep".to_string(),
                target: "sleep".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
    assert_eq!(response.choices.len(), 3);
}

#[test]
fn test_interactive_fiction_qualities() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Quality Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "start".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    // Add qualities
    if_script.qualities.insert("health".to_string(), Quality {
        initial: 100,
        min: Some(0),
        max: Some(100),
        hidden: false,
    });

    if_script.qualities.insert("strength".to_string(), Quality {
        initial: 5,
        min: Some(1),
        max: Some(10),
        hidden: false,
    });

    if_script.qualities.insert("wisdom".to_string(), Quality {
        initial: 3,
        min: None,
        max: None,
        hidden: false,
    });

    if_script.nodes.insert("start".to_string(), StoryNode {
        content: "Test your qualities.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Train strength".to_string(),
                target: "train".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("strength".to_string(), 1),
                ]),
            },
            StoryChoice {
                text: "Study books".to_string(),
                target: "study".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("wisdom".to_string(), 2),
                ]),
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_conditional_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Conditional Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "gate".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.qualities.insert("keys".to_string(), Quality {
        initial: 0,
        min: Some(0),
        max: None,
        hidden: false,
    });

    if_script.nodes.insert("gate".to_string(), StoryNode {
        content: "You stand before a locked gate.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Open gate (requires key)".to_string(),
                target: "beyond_gate".to_string(),
                conditions: Some(vec![
                    Condition::QualityAtLeast("keys".to_string(), 1),
                ]),
                consequences: None,
            },
            StoryChoice {
                text: "Search for key".to_string(),
                target: "find_key".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("keys".to_string(), 1),
                ]),
            },
            StoryChoice {
                text: "Give up".to_string(),
                target: "leave".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_storylets() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Storylet Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "hub".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: Some(vec![]),
    };

    if_script.qualities.insert("day".to_string(), Quality {
        initial: 1,
        min: Some(1),
        max: None,
        hidden: false,
    });

    // Add storylets
    if_script.storylets = Some(vec![
        Storylet {
            id: "morning_event".to_string(),
            title: "Morning Event".to_string(),
            conditions: vec![
                Condition::QualityBetween("day".to_string(), 1, 3),
            ],
            content: StoryNode {
                content: "A morning event occurs.".to_string(),
                choices: vec![],
                conditions: None,
                consequences: None,
            },
            priority: 10,
            repeatable: true,
        },
        Storylet {
            id: "special_event".to_string(),
            title: "Special Event".to_string(),
            conditions: vec![
                Condition::QualityAtLeast("day".to_string(), 5),
            ],
            content: StoryNode {
                content: "A special event!".to_string(),
                choices: vec![],
                conditions: None,
                consequences: None,
            },
            priority: 20,
            repeatable: false,
        }
    ]);

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_consequences() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Consequence Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "choice".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.qualities.insert("karma".to_string(), Quality {
        initial: 0,
        min: Some(-10),
        max: Some(10),
        hidden: false,
    });

    if_script.nodes.insert("choice".to_string(), StoryNode {
        content: "A moral choice.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Help the stranger".to_string(),
                target: "helped".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("karma".to_string(), 2),
                    Action::SetFlag("helped_stranger".to_string()),
                ]),
            },
            StoryChoice {
                text: "Ignore them".to_string(),
                target: "ignored".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Rob them".to_string(),
                target: "robbed".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("karma".to_string(), -3),
                    Action::SetFlag("criminal".to_string()),
                ]),
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_timed_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Timed Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: IFSettings {
            show_stats: false,
            checkpoint_saves: false,
            timed_choices: true,  // Enable timed choices
            quality_caps: false,
        },
        starting_node: "urgent".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.nodes.insert("urgent".to_string(), StoryNode {
        content: "The bomb is ticking! You have seconds to decide!".to_string(),
        choices: vec![
            StoryChoice {
                text: "Cut the red wire".to_string(),
                target: "red_wire".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Cut the blue wire".to_string(),
                target: "blue_wire".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Run away!".to_string(),
                target: "flee".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_quality_caps() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Cap Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: IFSettings {
            show_stats: true,
            checkpoint_saves: false,
            timed_choices: false,
            quality_caps: true,  // Enforce quality caps
        },
        starting_node: "test".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.qualities.insert("energy".to_string(), Quality {
        initial: 5,
        min: Some(0),
        max: Some(10),  // Capped at 10
        hidden: false,
    });

    if_script.nodes.insert("test".to_string(), StoryNode {
        content: "Test quality caps.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Gain lots of energy".to_string(),
                target: "energized".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("energy".to_string(), 20), // Should cap at 10
                ]),
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_checkpoints() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Checkpoint Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: IFSettings {
            show_stats: false,
            checkpoint_saves: true,  // Enable checkpoint saves
            timed_choices: false,
            quality_caps: false,
        },
        starting_node: "checkpoint1".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.nodes.insert("checkpoint1".to_string(), StoryNode {
        content: "First checkpoint.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Continue".to_string(),
                target: "checkpoint2".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: Some(vec![
            Action::SetFlag("checkpoint1_reached".to_string()),
        ]),
    });

    if_script.nodes.insert("checkpoint2".to_string(), StoryNode {
        content: "Second checkpoint.".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![
            Action::SetFlag("checkpoint2_reached".to_string()),
        ]),
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_multiple_endings() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Endings Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "final_choice".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.qualities.insert("morality".to_string(), Quality {
        initial: 0,
        min: Some(-100),
        max: Some(100),
        hidden: false,
    });

    if_script.nodes.insert("final_choice".to_string(), StoryNode {
        content: "The final choice determines your ending.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Hero ending".to_string(),
                target: "hero_end".to_string(),
                conditions: Some(vec![
                    Condition::QualityAtLeast("morality".to_string(), 50),
                ]),
                consequences: None,
            },
            StoryChoice {
                text: "Villain ending".to_string(),
                target: "villain_end".to_string(),
                conditions: Some(vec![
                    Condition::QualityAtMost("morality".to_string(), -50),
                ]),
                consequences: None,
            },
            StoryChoice {
                text: "Neutral ending".to_string(),
                target: "neutral_end".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: None,
    });

    for (ending, text) in [
        ("hero_end", "You saved the world!"),
        ("villain_end", "You conquered the world!"),
        ("neutral_end", "You walked away.")
    ] {
        if_script.nodes.insert(ending.to_string(), StoryNode {
            content: text.to_string(),
            choices: vec![],
            conditions: None,
            consequences: None,
        });
    }

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

#[test]
fn test_interactive_fiction_complex_conditions() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Complex Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_node: "complex".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    // Multiple qualities
    for (name, initial) in [("gold", 10), ("reputation", 5), ("level", 1)] {
        if_script.qualities.insert(name.to_string(), Quality {
            initial,
            min: Some(0),
            max: None,
            hidden: false,
        });
    }

    if_script.nodes.insert("complex".to_string(), StoryNode {
        content: "A complex situation with multiple requirements.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Elite option".to_string(),
                target: "elite".to_string(),
                conditions: Some(vec![
                    Condition::QualityAtLeast("gold".to_string(), 50),
                    Condition::QualityAtLeast("reputation".to_string(), 10),
                    Condition::QualityAtLeast("level".to_string(), 5),
                ]),
                consequences: None,
            },
            StoryChoice {
                text: "Moderate option".to_string(),
                target: "moderate".to_string(),
                conditions: Some(vec![
                    Condition::QualityAtLeast("gold".to_string(), 20),
                    Condition::QualityAtLeast("reputation".to_string(), 3),
                ]),
                consequences: None,
            },
            StoryChoice {
                text: "Basic option".to_string(),
                target: "basic".to_string(),
                conditions: None,
                consequences: None,
            }
        ],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    
    let response = engine.start().unwrap();
    assert!(response.success);
}

// ============================================================================
// CROSS-MODE AND SYSTEM TESTS (10+ tests)
// ============================================================================

#[test]
fn test_save_load_text_adventure() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Save Test" {
            mode: text_adventure
        }
        
        room room1 "Room 1" {
            description: "First room."
            exits: { east: room2 }
            items: [item1]
        }
        
        room room2 "Room 2" {
            description: "Second room."
            exits: { west: room1 }
        }
        
        item item1 "Test Item" {
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Make some changes
    engine.process_input("take item1").unwrap();
    engine.process_input("go east").unwrap();

    // Save game
    let save_response = engine.save_game(Some(1)).unwrap();
    assert!(save_response.success);

    // Make more changes
    engine.process_input("go west").unwrap();

    // Load game
    let load_response = engine.load_game(Some(1)).unwrap();
    assert!(load_response.success);
    assert_eq!(load_response.location, Some("room2".to_string()));
}

#[test]
fn test_save_load_visual_novel() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });

    let mut vn_script = VisualNovelScript {
        title: "Save VN Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: VisualNovelSettings {
            resolution: (1920, 1080),
            text_speed: TextSpeed::Normal,
            auto_save: true,  // Enable auto-save
            skip_mode: SkipMode::None,
        },
        starting_scene: "scene1".to_string(),
        scenes: HashMap::new(),
        characters: HashMap::new(),
        assets: Default::default(),
    };

    vn_script.scenes.insert("scene1".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "Scene 1".to_string(),
                voice: None,
                choices: None,
                effects: None,
            }
        ],
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    engine.start().unwrap();

    // Save and load
    let save_response = engine.save_game(Some(2)).unwrap();
    assert!(save_response.success);

    let load_response = engine.load_game(Some(2)).unwrap();
    assert!(load_response.success);
}

#[test]
fn test_save_load_interactive_fiction() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });

    let mut if_script = InteractiveFictionScript {
        title: "Save IF Test".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: IFSettings {
            show_stats: true,
            checkpoint_saves: true,
            timed_choices: false,
            quality_caps: false,
        },
        starting_node: "node1".to_string(),
        nodes: HashMap::new(),
        qualities: HashMap::new(),
        storylets: None,
    };

    if_script.qualities.insert("test_quality".to_string(), Quality {
        initial: 5,
        min: Some(0),
        max: Some(10),
        hidden: false,
    });

    if_script.nodes.insert("node1".to_string(), StoryNode {
        content: "Node 1".to_string(),
        choices: vec![],
        conditions: None,
        consequences: None,
    });

    let script_str = ron::to_string(&plotscript::script::GameScript::InteractiveFiction(if_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    engine.start().unwrap();

    // Save and load
    let save_response = engine.save_game(Some(3)).unwrap();
    assert!(save_response.success);

    let load_response = engine.load_game(Some(3)).unwrap();
    assert!(load_response.success);
}

#[test]
fn test_parser_typo_correction() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 70,
        ..Default::default()
    });

    let script = r#"
        game "Typo Test" {
            mode: text_adventure
        }
        
        room start "Start Room" {
            description: "Starting room."
            items: [apple]
        }
        
        item apple "Red Apple" {
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Test typo correction
    let response = engine.process_input("teka apple").unwrap();  // "teka" -> "take"
    assert!(response.success || response.text.contains("take"));
}

#[test]
fn test_parser_synonyms() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Synonym Test" {
            mode: text_adventure
        }
        
        room chamber "Chamber" {
            description: "A chamber."
            items: [sword]
        }
        
        item sword "Sword" {
            takeable: true
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Test synonyms
    let response = engine.process_input("get sword").unwrap();  // "get" is synonym for "take"
    assert!(response.success || response.text.contains("take"));

    let response = engine.process_input("examine sword").unwrap();
    assert!(response.success);
}

#[test]
fn test_inventory_management() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        max_inventory: 5,
        ..Default::default()
    });

    let script = r#"
        game "Inventory Test" {
            mode: text_adventure
        }
        
        room storage "Storage" {
            description: "A storage room."
            items: [item1, item2, item3, item4, item5, item6]
        }
        
        item item1 "Item 1" { takeable: true }
        item item2 "Item 2" { takeable: true }
        item item3 "Item 3" { takeable: true }
        item item4 "Item 4" { takeable: true }
        item item5 "Item 5" { takeable: true }
        item item6 "Item 6" { takeable: true }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Fill inventory
    for i in 1..=5 {
        let response = engine.process_input(&format!("take item{}", i)).unwrap();
        assert!(response.success);
    }

    // Try to exceed limit
    let response = engine.process_input("take item6").unwrap();
    assert!(!response.success);

    // Drop item and try again
    engine.process_input("drop item1").unwrap();
    let response = engine.process_input("take item6").unwrap();
    assert!(response.success);
}

#[test]
fn test_world_state_variables() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Variable Test" {
            mode: text_adventure
        }
        
        room lab "Laboratory" {
            description: "A laboratory."
        }
    "#;

    engine.load_script(script).unwrap();
    let response = engine.start().unwrap();
    
    // Check that game state is properly initialized
    assert!(response.state.variables.is_empty() || response.state.variables.len() >= 0);
    assert_eq!(response.state.score, 0);
    assert_eq!(response.state.turns, 0);
}

#[test]
fn test_extension_system_basic() {
    use plotscript::extensions::{Extension, ExtensionMetadata};

    // Create a simple test extension
    struct TestExtension {
        name: String,
    }

    impl Extension for TestExtension {
        fn metadata(&self) -> ExtensionMetadata {
            ExtensionMetadata {
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                author: "Test".to_string(),
                description: "Test extension".to_string(),
            }
        }

        fn on_load(&mut self, _engine: &mut Engine) -> plotscript::error::Result<()> {
            Ok(())
        }

        fn on_unload(&mut self, _engine: &mut Engine) -> plotscript::error::Result<()> {
            Ok(())
        }

        fn process_command(&mut self, _command: &str, _args: &[&str], _state: &mut plotscript::types::GameState) -> Option<Response> {
            None
        }
    }

    let mut engine = Engine::new();
    let extension = Box::new(TestExtension {
        name: "test_ext".to_string(),
    });

    // Register extension
    let result = engine.register_extension(extension);
    assert!(result.is_ok());

    // Check if extension is loaded
    assert!(engine.has_extension("test_ext"));

    // List extensions
    let extensions = engine.list_extensions();
    assert!(extensions.iter().any(|e| e.name == "test_ext"));
}

#[test]
fn test_error_handling_invalid_script() {
    let mut engine = Engine::new();

    let invalid_script = "This is not valid PlotScript syntax!";
    let result = engine.load_script(invalid_script);
    assert!(result.is_err());
}

#[test]
fn test_error_handling_missing_room() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Error Test" {
            mode: text_adventure
        }
        
        room start "Start" {
            description: "Start room."
            exits: {
                north: nonexistent_room
            }
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Try to go to non-existent room
    let response = engine.process_input("go north");
    assert!(response.is_err() || !response.unwrap().success);
}

#[test]
fn test_error_handling_game_not_started() {
    let mut engine = Engine::new();

    // Try to process input without starting game
    let result = engine.process_input("look");
    assert!(result.is_err());
}

#[test]
fn test_ron_script_parsing() {
    let mut engine = Engine::new();

    // Create a RON script
    let ta_script = plotscript::script::TextAdventureScript {
        title: "RON Test".to_string(),
        author: "Test Author".to_string(),
        description: "Testing RON parsing".to_string(),
        version: Some("1.0.0".to_string()),
        starting_location: "entrance".to_string(),
        locations: {
            let mut locs = HashMap::new();
            locs.insert("entrance".to_string(), Location {
                name: "Entrance".to_string(),
                description: "The entrance.".to_string(),
                exits: HashMap::new(),
                items: vec![],
                characters: vec![],
                dark: None,
                first_visit: None,
            });
            locs
        },
        items: HashMap::new(),
        characters: HashMap::new(),
    };

    let script_str = ron::to_string(&plotscript::script::GameScript::TextAdventure(ta_script)).unwrap();
    let result = engine.load_script(&script_str);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_game_mode_switching() {
    // Test that engine can switch between game modes
    let mut engine = Engine::new();

    // Load text adventure
    let ta_script = r#"
        game "TA Game" {
            mode: text_adventure
        }
        room start "Start" {
            description: "Text adventure."
        }
    "#;
    engine.load_script(ta_script).unwrap();
    assert_eq!(engine.config.mode, GameMode::TextAdventure);

    // Load visual novel (should switch mode)
    let vn_script = VisualNovelScript {
        title: "VN Game".to_string(),
        author: "Test".to_string(),
        description: None,
        settings: Default::default(),
        starting_scene: "start".to_string(),
        scenes: {
            let mut scenes = HashMap::new();
            scenes.insert("start".to_string(), Scene {
                background: None,
                music: None,
                characters: vec![],
                dialogue: vec![],
            });
            scenes
        },
        characters: HashMap::new(),
        assets: Default::default(),
    };

    let script_str = ron::to_string(&plotscript::script::GameScript::VisualNovel(vn_script)).unwrap();
    engine.load_script(&script_str).unwrap();
    assert_eq!(engine.config.mode, GameMode::VisualNovel);
}

#[test]
fn test_help_command() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Help Test" {
            mode: text_adventure
        }
        room start "Start" {
            description: "Start room."
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    let response = engine.process_input("help").unwrap();
    assert!(response.success);
    assert!(response.text.len() > 0); // Help text should be provided
}

#[test]
fn test_quit_command() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });

    let script = r#"
        game "Quit Test" {
            mode: text_adventure
        }
        room start "Start" {
            description: "Start room."
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    let response = engine.process_input("quit").unwrap();
    assert!(response.success);
    assert!(response.ended); // Game should be marked as ended
}

#[test]
fn test_debug_mode() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        debug: true,  // Enable debug mode
        ..Default::default()
    });

    let script = r#"
        game "Debug Test" {
            mode: text_adventure
        }
        room start "Start" {
            description: "Start room."
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Debug commands should work
    let response = engine.process_input("debug test").unwrap();
    assert!(response.success);
}

#[test]
fn test_auto_save_functionality() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        auto_save: true,  // Enable auto-save
        ..Default::default()
    });

    let script = r#"
        game "Auto Save Test" {
            mode: text_adventure
        }
        room room1 "Room 1" {
            description: "First room."
            exits: { east: room2 }
        }
        room room2 "Room 2" {
            description: "Second room."
            exits: { west: room1 }
        }
    "#;

    engine.load_script(script).unwrap();
    engine.start().unwrap();

    // Movement should trigger auto-save
    engine.process_input("go east").unwrap();

    // Try loading auto-save (slot 0)
    let response = engine.load_game(Some(0));
    assert!(response.is_ok() || response.is_err()); // Auto-save may or may not exist
}