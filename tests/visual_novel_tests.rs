use plotscript::{
    engine::{Engine, EngineConfig},
    types::{GameMode, Value},
    script::{
        GameScript, VisualNovelScript, VisualNovelSettings, Scene, VNCharacter, 
        DialogueLine, CharacterPosition, Position, Assets, Choice,
        TextSpeed, SkipMode, Effect, Transition
    },
};
use std::collections::HashMap;

fn create_test_vn_script() -> VisualNovelScript {
    let mut scenes = HashMap::new();
    let mut characters = HashMap::new();
    let mut assets = Assets::default();
    
    // Create test characters
    characters.insert("alice".to_string(), VNCharacter {
        name: "Alice".to_string(),
        color: Some("#FF0000".to_string()),
        sprites: {
            let mut sprites = HashMap::new();
            sprites.insert("normal".to_string(), "alice_normal.png".to_string());
            sprites.insert("happy".to_string(), "alice_happy.png".to_string());
            sprites.insert("sad".to_string(), "alice_sad.png".to_string());
            sprites
        },
    });
    
    characters.insert("bob".to_string(), VNCharacter {
        name: "Bob".to_string(),
        color: Some("#0000FF".to_string()),
        sprites: {
            let mut sprites = HashMap::new();
            sprites.insert("normal".to_string(), "bob_normal.png".to_string());
            sprites.insert("angry".to_string(), "bob_angry.png".to_string());
            sprites
        },
    });
    
    // Create opening scene
    scenes.insert("opening".to_string(), Scene {
        background: Some("school_hallway.jpg".to_string()),
        music: Some("gentle_theme.ogg".to_string()),
        characters: vec![
            CharacterPosition {
                id: "alice".to_string(),
                sprite: "normal".to_string(),
                position: Position::Left,
            },
        ],
        dialogue: vec![
            DialogueLine {
                speaker: Some("alice".to_string()),
                text: "Hello! Welcome to our school.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            },
            DialogueLine {
                speaker: Some("alice".to_string()),
                text: "What would you like to do?".to_string(),
                voice: None,
                choices: Some(vec![
                    Choice {
                        text: "Tour the school".to_string(),
                        target: "tour".to_string(),
                        conditions: None,
                    },
                    Choice {
                        text: "Go to class".to_string(),
                        target: "classroom".to_string(),
                        conditions: None,
                    },
                ]),
                effects: None,
            },
        ],
    });
    
    // Create tour scene
    scenes.insert("tour".to_string(), Scene {
        background: Some("school_courtyard.jpg".to_string()),
        music: Some("upbeat_theme.ogg".to_string()),
        characters: vec![
            CharacterPosition {
                id: "alice".to_string(),
                sprite: "happy".to_string(),
                position: Position::Center,
            },
        ],
        dialogue: vec![
            DialogueLine {
                speaker: Some("alice".to_string()),
                text: "Great! Let me show you around.".to_string(),
                voice: None,
                choices: None,
                effects: Some(vec![Effect::Transition(Transition::Fade)]),
            },
            DialogueLine {
                speaker: None,
                text: "You spend the afternoon touring the campus.".to_string(),
                voice: None,
                choices: None,
                effects: None,
            },
        ],
    });
    
    // Create classroom scene
    scenes.insert("classroom".to_string(), Scene {
        background: Some("classroom.jpg".to_string()),
        music: None,
        characters: vec![
            CharacterPosition {
                id: "bob".to_string(),
                sprite: "normal".to_string(),
                position: Position::Right,
            },
            CharacterPosition {
                id: "alice".to_string(),
                sprite: "sad".to_string(),
                position: Position::Left,
            },
        ],
        dialogue: vec![
            DialogueLine {
                speaker: Some("bob".to_string()),
                text: "You're late!".to_string(),
                voice: None,
                choices: None,
                effects: Some(vec![Effect::Shake]),
            },
            DialogueLine {
                speaker: Some("alice".to_string()),
                text: "Sorry, I was showing them around...".to_string(),
                voice: None,
                choices: None,
                effects: None,
            },
        ],
    });
    
    // Setup assets
    assets.backgrounds.insert("school_hallway.jpg".to_string(), "/assets/bg/hallway.jpg".to_string());
    assets.backgrounds.insert("school_courtyard.jpg".to_string(), "/assets/bg/courtyard.jpg".to_string());
    assets.backgrounds.insert("classroom.jpg".to_string(), "/assets/bg/classroom.jpg".to_string());
    assets.music.insert("gentle_theme.ogg".to_string(), "/assets/music/gentle.ogg".to_string());
    assets.music.insert("upbeat_theme.ogg".to_string(), "/assets/music/upbeat.ogg".to_string());
    
    VisualNovelScript {
        title: "Test Visual Novel".to_string(),
        author: "Test Author".to_string(),
        description: Some("A test visual novel".to_string()),
        settings: VisualNovelSettings::default(),
        starting_scene: "opening".to_string(),
        scenes,
        characters,
        assets,
    }
}

#[test]
fn test_vn_loading() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    let result = engine.load_script(&ron_str);
    assert!(result.is_ok());
}

#[test]
fn test_vn_scene_display() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Get initial scene
    let response = engine.process_input("").unwrap();
    assert!(response.text.contains("Hello! Welcome to our school."));
    assert_eq!(response.location, Some("opening".to_string()));
    assert_eq!(response.media.len(), 1); // Background image
    assert_eq!(response.sounds.len(), 1); // Background music
}

#[test]
fn test_vn_dialogue_progression() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Get first dialogue
    let response1 = engine.process_input("").unwrap();
    assert!(response1.text.contains("Hello! Welcome to our school."));
    
    // Advance to next dialogue
    let response2 = engine.process_input("next").unwrap();
    assert!(response2.text.contains("What would you like to do?"));
    assert_eq!(response2.choices.len(), 2);
}

#[test]
fn test_vn_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Advance to choice point
    engine.process_input("").unwrap();
    let response = engine.process_input("next").unwrap();
    
    // Verify choices
    assert_eq!(response.choices.len(), 2);
    assert_eq!(response.choices[0].text, "Tour the school");
    assert_eq!(response.choices[1].text, "Go to class");
    
    // Select first choice
    let response = engine.process_input("1").unwrap();
    assert!(response.text.contains("Great! Let me show you around."));
    assert_eq!(response.location, Some("tour".to_string()));
}

#[test]
fn test_vn_scene_transition() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Navigate to choice
    engine.process_input("").unwrap();
    engine.process_input("next").unwrap();
    
    // Choose classroom
    let response = engine.process_input("2").unwrap();
    assert!(response.text.contains("You're late!"));
    assert_eq!(response.location, Some("classroom".to_string()));
    
    // Verify multiple characters on screen
    assert!(response.text.contains("Bob"));
}

#[test]
fn test_vn_auto_mode() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Enable auto mode
    let response = engine.process_input("auto on").unwrap();
    assert!(response.success);
    
    // Check auto mode is enabled
    assert_eq!(
        engine.get_state("vn_auto_mode"),
        Some(&Value::Bool(true))
    );
}

#[test]
fn test_vn_skip_mode() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Enable skip mode
    let response = engine.process_input("skip on").unwrap();
    assert!(response.success);
    
    // Check skip mode is enabled
    assert_eq!(
        engine.get_state("vn_skip_mode"),
        Some(&Value::Bool(true))
    );
}

#[test]
fn test_vn_save_and_load() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script.clone());
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Progress to a specific point
    engine.process_input("").unwrap();
    engine.process_input("next").unwrap();
    engine.process_input("1").unwrap(); // Choose tour
    
    // Save state
    let save_response = engine.save_game(Some(0)).unwrap();
    assert!(save_response.success);
    
    // Create new engine and load
    let mut new_engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    new_engine.load_script(&ron_str).unwrap();
    let load_response = new_engine.load_game(Some(0)).unwrap();
    assert!(load_response.success);
    
    // Verify we're at the saved location
    let response = new_engine.process_input("").unwrap();
    assert_eq!(response.location, Some("tour".to_string()));
}

#[test]
fn test_vn_character_sprites() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Navigate to classroom with multiple characters
    engine.process_input("").unwrap();
    engine.process_input("next").unwrap();
    let response = engine.process_input("2").unwrap();
    
    // Verify both characters are displayed
    // The response should include information about both characters
    assert!(engine.get_state("vn_current_scene").is_some());
}

#[test]
fn test_vn_effects() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let mut script = create_test_vn_script();
    
    // Add a scene with effects
    script.scenes.insert("effects_test".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![
            DialogueLine {
                speaker: None,
                text: "Testing effects...".to_string(),
                voice: None,
                choices: None,
                effects: Some(vec![
                    Effect::Transition(Transition::Dissolve),
                    Effect::Flash("white".to_string()),
                ]),
            },
        ],
    });
    
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Navigate to effects test scene
    engine.set_state("vn_current_scene", Value::String("effects_test".to_string()));
    let response = engine.process_input("").unwrap();
    
    // Effects should be processed (though not visually in tests)
    assert!(response.text.contains("Testing effects..."));
}

#[test]
fn test_vn_invalid_choice() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Advance to choice point
    engine.process_input("").unwrap();
    engine.process_input("next").unwrap();
    
    // Try invalid choice
    let response = engine.process_input("99");
    assert!(response.is_err() || !response.unwrap().success);
}

#[test]
fn test_vn_scene_not_found() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let mut script = create_test_vn_script();
    script.starting_scene = "nonexistent".to_string();
    
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    let result = engine.load_script(&ron_str);
    assert!(result.is_err());
}

#[test]
fn test_vn_empty_dialogue() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let mut script = create_test_vn_script();
    script.scenes.insert("empty".to_string(), Scene {
        background: None,
        music: None,
        characters: vec![],
        dialogue: vec![],
    });
    script.starting_scene = "empty".to_string();
    
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    let response = engine.process_input("").unwrap();
    assert!(response.text.contains("end") || response.text.contains("Scene complete"));
}

#[test]
fn test_vn_text_speed_setting() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let mut script = create_test_vn_script();
    script.settings.text_speed = TextSpeed::Instant;
    
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Text speed should be stored in state
    assert!(engine.get_state("vn_settings").is_some());
}

#[test]
fn test_vn_music_changes() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = create_test_vn_script();
    let game_script = GameScript::VisualNovel(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // First scene has music
    let response1 = engine.process_input("").unwrap();
    assert_eq!(response1.sounds.len(), 1);
    assert_eq!(response1.sounds[0].source, "/assets/music/gentle.ogg");
    
    // Navigate to tour scene with different music
    engine.process_input("next").unwrap();
    let response2 = engine.process_input("1").unwrap();
    assert_eq!(response2.sounds.len(), 1);
    assert_eq!(response2.sounds[0].source, "/assets/music/upbeat.ogg");
}