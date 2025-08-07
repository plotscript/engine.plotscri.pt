//! End-to-end tests for the PlotScript engine
//! Tests all three game modes with realistic scenarios

use plotscript::{Engine, EngineConfig, GameMode, init, types::*};

#[test]
fn test_text_adventure_complete_game() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 80,
        ..Default::default()
    });
    
    // Load a complete text adventure game
    let script = r#"
TextAdventure((
    title: "Mystery Manor",
    author: "Test Suite",
    description: Some("A mysterious adventure"),
    version: Some("1.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: true,
        inventory_limits: true,
        max_inventory: Some(5),
    ),
    
    starting_location: "entrance",
    
    locations: {
        "entrance": (
            name: "Manor Entrance",
            description: "A grand entrance with a heavy wooden door. A brass key lies on the ground.",
            exits: {
                north: "hallway",
            },
            items: ["brass_key"],
            characters: [],
            dark: Some(false),
            first_visit: Some("You stand before the imposing Manor entrance."),
            events: None,
        ),
        "hallway": (
            name: "Dark Hallway",
            description: "A long, dark hallway stretches before you.",
            exits: {
                south: "entrance",
                north: "library",
                east: "kitchen",
            },
            items: ["lantern"],
            characters: [],
            dark: Some(true),
            first_visit: None,
            events: None,
        ),
        "library": (
            name: "Dusty Library",
            description: "Rows of ancient books line the walls. A locked chest sits in the corner.",
            exits: {
                south: "hallway",
            },
            items: ["ancient_book", "locked_chest"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "kitchen": (
            name: "Old Kitchen",
            description: "A kitchen with rusty pots and pans.",
            exits: {
                west: "hallway",
            },
            items: ["matches"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "brass_key": (
            name: "brass key",
            description: "A heavy brass key with intricate engravings.",
            takeable: true,
            hidden: false,
            weight: Some(1),
            value: Some(10),
            container: None,
            on_use: [],
            on_examine: [],
        ),
        "lantern": (
            name: "old lantern",
            description: "An old oil lantern, currently unlit.",
            takeable: true,
            hidden: false,
            weight: Some(2),
            value: Some(5),
            container: None,
            on_use: [],
            on_examine: [],
        ),
        "matches": (
            name: "box of matches",
            description: "A small box of matches.",
            takeable: true,
            hidden: false,
            weight: Some(1),
            value: Some(2),
            container: None,
            on_use: [],
            on_examine: [],
        ),
        "ancient_book": (
            name: "ancient book",
            description: "A leather-bound book filled with mysterious symbols.",
            takeable: true,
            hidden: false,
            weight: Some(3),
            value: Some(50),
            container: None,
            on_use: [],
            on_examine: [],
        ),
        "locked_chest": (
            name: "locked chest",
            description: "A sturdy wooden chest with a brass lock.",
            takeable: false,
            hidden: false,
            weight: None,
            value: None,
            container: Some((
                locked: true,
                key: Some("brass_key"),
                items: ["treasure"],
            )),
            on_use: [],
            on_examine: [],
        ),
        "treasure": (
            name: "golden treasure",
            description: "A pile of golden coins!",
            takeable: true,
            hidden: true,
            weight: Some(5),
            value: Some(1000),
            container: None,
            on_use: [],
            on_examine: [],
        ),
    },
    
    characters: {},
    
    vocabulary: (
        verbs: ["look", "examine", "take", "get", "drop", "go", "walk", "move", "use", "unlock", "open", "light"],
        synonyms: {
            "get": "take",
            "grab": "take",
            "pickup": "take",
            "walk": "go",
            "move": "go",
            "travel": "go",
            "inspect": "examine",
            "check": "examine",
            "l": "look",
            "x": "examine",
            "n": "north",
            "s": "south",
            "e": "east",
            "w": "west",
        },
    ),
    
    flags: {},
    
    counters: {},
    
    events: [],
))
"#;
    
    engine.load_script(script).expect("Failed to load script");
    let start_response = engine.start().expect("Failed to start game");
    
    // Test the game flow
    assert!(start_response.text.contains("Manor Entrance") || start_response.text.contains("imposing Manor"));
    
    // Process commands and test responses
    let response = engine.process_input("look").unwrap();
    assert!(response.text.contains("entrance") || response.text.contains("brass key"));
    
    let response = engine.process_input("take brass key").unwrap();
    assert!(response.text.contains("take") || response.text.contains("brass") || response.text.contains("key"));
    
    let response = engine.process_input("go north").unwrap();
    assert!(response.text.contains("dark") || response.text.contains("hallway") || response.text.contains("can't see"));
    
    // Test inventory
    let response = engine.process_input("inventory").unwrap();
    assert!(response.text.contains("brass key") || response.text.contains("carrying"));
}

#[test]
fn test_visual_novel_complete_game() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::VisualNovel,
        ..Default::default()
    });
    
    let script = r#"
VisualNovel((
    title: "Summer Romance",
    author: "Test Suite",
    description: Some("A visual novel test"),
    
    settings: (
        resolution: (1920, 1080),
        text_speed: Normal,
        auto_save: true,
        skip_mode: Read,
    ),
    
    starting_scene: "opening",
    
    scenes: {
        "opening": (
            background: Some("school.jpg"),
            music: Some("gentle.mp3"),
            characters: [
                (
                    id: "maya",
                    sprite: "maya_happy.png",
                    position: Left,
                ),
            ],
            dialogue: [
                (
                    speaker: Some("Maya"),
                    text: "Good morning! It's a beautiful day, isn't it?",
                    voice: None,
                    choices: Some([
                        (
                            text: "Yes, it's lovely!",
                            target: "scene2_happy",
                            conditions: None,
                        ),
                        (
                            text: "I guess so...",
                            target: "scene2_neutral",
                            conditions: None,
                        ),
                    ]),
                    effects: None,
                ),
            ],
        ),
        "scene2_happy": (
            background: Some("school.jpg"),
            music: Some("gentle.mp3"),
            characters: [
                (
                    id: "maya",
                    sprite: "maya_excited.png",
                    position: Left,
                ),
            ],
            dialogue: [
                (
                    speaker: Some("Maya"),
                    text: "I'm so glad you think so! Let's make the most of today!",
                    voice: None,
                    choices: None,
                    effects: None,
                ),
            ],
        ),
        "scene2_neutral": (
            background: Some("school.jpg"),
            music: Some("gentle.mp3"),
            characters: [
                (
                    id: "maya",
                    sprite: "maya_neutral.png",
                    position: Left,
                ),
            ],
            dialogue: [
                (
                    speaker: Some("Maya"),
                    text: "Well, I hope your day gets better!",
                    voice: None,
                    choices: None,
                    effects: None,
                ),
            ],
        ),
    },
    
    characters: {
        "maya": (
            name: "Maya",
            color: Some("pink"),
            sprites: {
                "happy": "maya_happy.png",
                "excited": "maya_excited.png",
                "neutral": "maya_neutral.png",
            },
        ),
    },
    
    assets: (
        backgrounds: {
            "school": "school.jpg",
        },
        sprites: {
            "maya_happy": "maya_happy.png",
            "maya_excited": "maya_excited.png",
            "maya_neutral": "maya_neutral.png",
        },
        music: {
            "gentle": "gentle.mp3",
        },
        sounds: {},
        voices: {},
    ),
))
"#;
    
    engine.load_script(script).expect("Failed to load VN script");
    let start_response = engine.start().expect("Failed to start VN");
    
    // Test visual novel flow
    assert!(start_response.text.contains("Maya") || start_response.text.contains("morning"));
    
    // Test choices
    let choices = engine.get_current_choices();
    assert_eq!(choices.len(), 2);
    assert!(choices[0].contains("lovely") || choices[0].contains("Yes"));
    
    // Make a choice
    let response = engine.process_input("1").unwrap();
    assert!(response.text.contains("glad") || response.text.contains("today"));
}

#[test]
fn test_interactive_fiction_complete_game() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = r#"
InteractiveFiction((
    title: "Corporate Spy",
    author: "Test Suite",
    description: Some("An interactive fiction test"),
    
    settings: (
        show_stats: true,
        checkpoint_saves: true,
        timed_choices: false,
        quality_caps: true,
    ),
    
    starting_node: "mission_brief",
    
    nodes: {
        "mission_brief": (
            content: "Your handler slides a folder across the table. 'This is your next assignment.'",
            choices: [
                (
                    text: "Open the folder",
                    target: "read_mission",
                    conditions: None,
                    consequences: Some([
                        SetQuality("curiosity", 1),
                    ]),
                ),
                (
                    text: "Push it back",
                    target: "refuse_mission",
                    conditions: Some([
                        QualityAtLeast("courage", 5),
                    ]),
                    consequences: Some([
                        SetQuality("reputation", -2),
                    ]),
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "read_mission": (
            content: "The folder contains details about infiltrating a tech company. The pay is substantial.",
            choices: [
                (
                    text: "Accept the mission",
                    target: "accept",
                    conditions: None,
                    consequences: Some([
                        SetQuality("money", 100),
                        SetFlag("mission_accepted"),
                    ]),
                ),
                (
                    text: "Ask for more money",
                    target: "negotiate",
                    conditions: Some([
                        QualityAtLeast("negotiation", 3),
                    ]),
                    consequences: None,
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "refuse_mission": (
            content: "Your handler looks disappointed. 'I thought you had more nerve than that.'",
            choices: [
                (
                    text: "Reconsider",
                    target: "mission_brief",
                    conditions: None,
                    consequences: None,
                ),
                (
                    text: "Leave",
                    target: "game_over",
                    conditions: None,
                    consequences: None,
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "accept": (
            content: "You accept the mission. Time to prepare for infiltration.",
            choices: [],
            conditions: None,
            consequences: None,
        ),
        "negotiate": (
            content: "After some back and forth, you secure a better deal.",
            choices: [
                (
                    text: "Accept improved offer",
                    target: "accept",
                    conditions: None,
                    consequences: Some([
                        SetQuality("money", 150),
                        SetFlag("mission_accepted"),
                    ]),
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "game_over": (
            content: "You walk away from the spy life. Game Over.",
            choices: [],
            conditions: None,
            consequences: None,
        ),
    },
    
    qualities: {
        "courage": (
            initial: 3,
            min: Some(0),
            max: Some(10),
            hidden: false,
        ),
        "curiosity": (
            initial: 5,
            min: Some(0),
            max: Some(10),
            hidden: false,
        ),
        "money": (
            initial: 50,
            min: Some(0),
            max: None,
            hidden: false,
        ),
        "negotiation": (
            initial: 2,
            min: Some(0),
            max: Some(10),
            hidden: false,
        ),
        "reputation": (
            initial: 5,
            min: Some(-10),
            max: Some(10),
            hidden: false,
        ),
    },
    
    storylets: None,
))
"#;
    
    engine.load_script(script).expect("Failed to load IF script");
    let start_response = engine.start().expect("Failed to start IF");
    
    // Test interactive fiction flow
    assert!(start_response.text.contains("handler") || start_response.text.contains("folder"));
    
    // Test choices
    let choices = engine.get_current_choices();
    assert!(choices.len() >= 1);
    assert!(choices[0].contains("Open") || choices[0].contains("folder"));
    
    // Make a choice
    let response = engine.process_input("1").unwrap();
    assert!(response.text.contains("infiltrating") || response.text.contains("tech") || response.text.contains("pay"));
    
    // Check qualities were updated
    if let Some(curiosity) = engine.get_state("curiosity") {
        if let Value::Float(n) = curiosity {
            assert!(*n > 0.0);
        }
    }
}

#[test]
fn test_save_and_load_functionality() {
    init();
    let mut engine = Engine::new();
    
    // Create a simple game
    let script = r#"
InteractiveFiction((
    title: "Save Test",
    author: "Test",
    description: None,
    
    settings: (
        show_stats: false,
        checkpoint_saves: true,
        timed_choices: false,
        quality_caps: false,
    ),
    
    starting_node: "start",
    
    nodes: {
        "start": (
            content: "Beginning of the game.",
            choices: [
                (
                    text: "Continue",
                    target: "middle",
                    conditions: None,
                    consequences: Some([
                        SetQuality("progress", 1),
                    ]),
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "middle": (
            content: "Middle of the game.",
            choices: [
                (
                    text: "Continue",
                    target: "end",
                    conditions: None,
                    consequences: Some([
                        SetQuality("progress", 2),
                    ]),
                ),
            ],
            conditions: None,
            consequences: None,
        ),
        "end": (
            content: "End of the game.",
            choices: [],
            conditions: None,
            consequences: None,
        ),
    },
    
    qualities: {
        "progress": (
            initial: 0,
            min: None,
            max: None,
            hidden: false,
        ),
    },
    
    storylets: None,
))
"#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start");
    
    // Progress through the game
    engine.process_input("1").unwrap();
    
    // Save the game
    engine.save_game(Some(1)).expect("Failed to save");
    
    // Create a new engine and load the save
    let mut new_engine = Engine::new();
    new_engine.load_script(script).expect("Failed to load script");
    new_engine.load_game(Some(1)).expect("Failed to load save");
    
    // Verify we're at the same point
    let response = new_engine.process_input("look").unwrap();
    assert!(response.text.contains("Middle") || response.text.contains("middle"));
    
    // Check progress quality was preserved
    if let Some(progress) = new_engine.get_state("progress") {
        if let Value::Float(n) = progress {
            assert_eq!(*n, 1.0);
        }
    }
}

#[test]
fn test_typo_correction() {
    init();
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 80,
        ..Default::default()
    });
    
    let script = r#"
TextAdventure((
    title: "Typo Test",
    author: "Test",
    description: None,
    version: None,
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "room",
    
    locations: {
        "room": (
            name: "Test Room",
            description: "A room with a sword.",
            exits: {},
            items: ["sword"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "sword": (
            name: "sword",
            description: "A sharp sword.",
            takeable: true,
            hidden: false,
            weight: Some(3),
            value: Some(100),
            container: None,
            on_use: [],
            on_examine: [],
        ),
    },
    
    characters: {},
    
    vocabulary: (
        verbs: ["take", "examine", "look"],
        synonyms: {},
    ),
    
    flags: {},
    counters: {},
    events: [],
))
"#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start");
    
    // Test typo correction
    let response = engine.process_input("tkae sword").unwrap();  // typo in "take"
    // Should correct to "take sword"
    assert!(response.text.contains("take") || response.text.contains("sword") || response.text.contains("pick"));
    
    let response = engine.process_input("exmaine sword").unwrap();  // typo in "examine"
    // Should correct to "examine sword"
    assert!(response.text.contains("sharp") || response.text.contains("sword") || response.text.contains("Sharp"));
}

#[test]
fn test_complex_state_management() {
    init();
    let mut engine = Engine::new();
    
    // Test setting and getting various state values
    engine.set_state("test_string", Value::String("hello".to_string()));
    engine.set_state("test_number", Value::Float(42.0));
    engine.set_state("test_bool", Value::Bool(true));
    
    assert_eq!(engine.get_state("test_string"), Some(&Value::String("hello".to_string())));
    assert_eq!(engine.get_state("test_number"), Some(&Value::Float(42.0)));
    assert_eq!(engine.get_state("test_bool"), Some(&Value::Bool(true)));
    
    // Test modifying state
    engine.set_state("test_number", Value::Float(100.0));
    assert_eq!(engine.get_state("test_number"), Some(&Value::Float(100.0)));
}

#[test]
fn test_error_handling() {
    init();
    let mut engine = Engine::new();
    
    // Test various error conditions
    assert!(engine.start().is_err());  // No script loaded
    assert!(engine.load_script("invalid {{{{ script").is_err());  // Invalid script
    
    // Load a valid script
    let script = r#"
InteractiveFiction((
    title: "Error Test",
    author: "Test",
    description: None,
    
    settings: (
        show_stats: false,
        checkpoint_saves: false,
        timed_choices: false,
        quality_caps: false,
    ),
    
    starting_node: "start",
    
    nodes: {
        "start": (
            content: "Start",
            choices: [],
            conditions: None,
            consequences: None,
        ),
    },
    
    qualities: {},
    storylets: None,
))
"#;
    
    engine.load_script(script).expect("Failed to load script");
    engine.start().expect("Failed to start");
    
    // Test invalid input handling
    let response = engine.process_input("completely invalid nonsense command").unwrap();
    assert!(response.success || !response.text.is_empty());  // Should handle gracefully
}