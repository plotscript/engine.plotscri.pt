use plotscript::{
    engine::{Engine, EngineConfig},
    types::{GameMode, Value},
    script::{
        GameScript, InteractiveFictionScript, IFSettings, StoryNode, StoryChoice,
        Quality, Storylet, Condition, Action
    },
};
use std::collections::HashMap;

fn create_test_if_script() -> InteractiveFictionScript {
    let mut nodes = HashMap::new();
    let mut qualities = HashMap::new();
    let mut storylets = Vec::new();
    
    // Setup qualities
    qualities.insert("health".to_string(), Quality {
        initial: 100,
        min: Some(0),
        max: Some(100),
        hidden: false,
    });
    
    qualities.insert("courage".to_string(), Quality {
        initial: 5,
        min: Some(0),
        max: Some(10),
        hidden: false,
    });
    
    qualities.insert("gold".to_string(), Quality {
        initial: 10,
        min: Some(0),
        max: None,
        hidden: false,
    });
    
    qualities.insert("reputation".to_string(), Quality {
        initial: 0,
        min: Some(-100),
        max: Some(100),
        hidden: false,
    });
    
    // Create starting node
    nodes.insert("start".to_string(), StoryNode {
        content: "You stand at the entrance of a dark cave. The wind howls behind you, and you can hear strange sounds from within.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Enter the cave bravely".to_string(),
                target: "cave_entrance".to_string(),
                conditions: Some(vec![Condition::QualityAtLeast("courage".to_string(), 3)]),
                consequences: Some(vec![Action::ChangeQuality("courage".to_string(), 1)]),
            },
            StoryChoice {
                text: "Enter the cave cautiously".to_string(),
                target: "cave_entrance".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Run away".to_string(),
                target: "coward_ending".to_string(),
                conditions: None,
                consequences: Some(vec![Action::ChangeQuality("courage".to_string(), -2)]),
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Cave entrance node
    nodes.insert("cave_entrance".to_string(), StoryNode {
        content: "The cave is damp and cold. You see two paths ahead.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Take the left path".to_string(),
                target: "treasure_room".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Take the right path".to_string(),
                target: "monster_room".to_string(),
                conditions: None,
                consequences: None,
            },
        ],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("entered_cave".to_string())]),
    });
    
    // Treasure room
    nodes.insert("treasure_room".to_string(), StoryNode {
        content: "You find a room filled with glittering gold coins!".to_string(),
        choices: vec![
            StoryChoice {
                text: "Take all the gold".to_string(),
                target: "greedy_ending".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("gold".to_string(), 100),
                    Action::ChangeQuality("reputation".to_string(), -10),
                ]),
            },
            StoryChoice {
                text: "Take some gold".to_string(),
                target: "exit_cave".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("gold".to_string(), 20),
                ]),
            },
            StoryChoice {
                text: "Leave the gold".to_string(),
                target: "exit_cave".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("reputation".to_string(), 10),
                ]),
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Monster room
    nodes.insert("monster_room".to_string(), StoryNode {
        content: "A terrifying monster blocks your path!".to_string(),
        choices: vec![
            StoryChoice {
                text: "Fight the monster".to_string(),
                target: "battle".to_string(),
                conditions: Some(vec![Condition::QualityAtLeast("courage".to_string(), 5)]),
                consequences: Some(vec![Action::ChangeQuality("health".to_string(), -30)]),
            },
            StoryChoice {
                text: "Try to sneak past".to_string(),
                target: "sneak_attempt".to_string(),
                conditions: None,
                consequences: None,
            },
            StoryChoice {
                text: "Run back".to_string(),
                target: "cave_entrance".to_string(),
                conditions: None,
                consequences: Some(vec![Action::ChangeQuality("courage".to_string(), -1)]),
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Battle node
    nodes.insert("battle".to_string(), StoryNode {
        content: "You defeat the monster after a fierce battle!".to_string(),
        choices: vec![
            StoryChoice {
                text: "Continue deeper".to_string(),
                target: "deep_cave".to_string(),
                conditions: None,
                consequences: Some(vec![
                    Action::ChangeQuality("courage".to_string(), 2),
                    Action::SetFlag("defeated_monster".to_string()),
                ]),
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Sneak attempt
    nodes.insert("sneak_attempt".to_string(), StoryNode {
        content: "You try to sneak past the monster...".to_string(),
        choices: vec![
            StoryChoice {
                text: "Continue".to_string(),
                target: "deep_cave".to_string(),
                conditions: Some(vec![Condition::QualityAtLeast("courage".to_string(), 3)]),
                consequences: None,
            },
            StoryChoice {
                text: "Give up".to_string(),
                target: "cave_entrance".to_string(),
                conditions: None,
                consequences: None,
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Deep cave
    nodes.insert("deep_cave".to_string(), StoryNode {
        content: "You've reached the deepest part of the cave. An ancient artifact glows before you.".to_string(),
        choices: vec![
            StoryChoice {
                text: "Take the artifact".to_string(),
                target: "hero_ending".to_string(),
                conditions: Some(vec![Condition::HasFlag("defeated_monster".to_string())]),
                consequences: Some(vec![Action::SetFlag("has_artifact".to_string())]),
            },
            StoryChoice {
                text: "Leave it be".to_string(),
                target: "wise_ending".to_string(),
                conditions: None,
                consequences: Some(vec![Action::ChangeQuality("reputation".to_string(), 20)]),
            },
        ],
        conditions: None,
        consequences: None,
    });
    
    // Various endings
    nodes.insert("coward_ending".to_string(), StoryNode {
        content: "You run away from the cave. Perhaps adventure isn't for you...".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("game_over".to_string())]),
    });
    
    nodes.insert("greedy_ending".to_string(), StoryNode {
        content: "As you grab all the gold, the cave begins to collapse! You barely escape with your life and the gold.".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("game_over".to_string())]),
    });
    
    nodes.insert("hero_ending".to_string(), StoryNode {
        content: "You emerge from the cave as a hero, artifact in hand!".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("game_over".to_string())]),
    });
    
    nodes.insert("wise_ending".to_string(), StoryNode {
        content: "You leave the artifact undisturbed. The villagers praise your wisdom.".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("game_over".to_string())]),
    });
    
    nodes.insert("exit_cave".to_string(), StoryNode {
        content: "You exit the cave safely.".to_string(),
        choices: vec![],
        conditions: None,
        consequences: Some(vec![Action::SetFlag("game_over".to_string())]),
    });
    
    // Create a storylet that appears when conditions are met
    storylets.push(Storylet {
        id: "special_encounter".to_string(),
        title: "A Mysterious Stranger".to_string(),
        conditions: vec![
            Condition::HasFlag("entered_cave".to_string()),
            Condition::QualityAtLeast("gold".to_string(), 20),
        ],
        content: StoryNode {
            content: "A mysterious stranger offers you a deal...".to_string(),
            choices: vec![
                StoryChoice {
                    text: "Accept the deal".to_string(),
                    target: "start".to_string(),
                    conditions: None,
                    consequences: Some(vec![
                        Action::ChangeQuality("gold".to_string(), -10),
                        Action::ChangeQuality("reputation".to_string(), 5),
                    ]),
                },
                StoryChoice {
                    text: "Decline".to_string(),
                    target: "start".to_string(),
                    conditions: None,
                    consequences: None,
                },
            ],
            conditions: None,
            consequences: None,
        },
        priority: 10,
        repeatable: false,
    });
    
    InteractiveFictionScript {
        title: "Cave Adventure".to_string(),
        author: "Test Author".to_string(),
        description: Some("A test interactive fiction game".to_string()),
        settings: IFSettings::default(),
        starting_node: "start".to_string(),
        nodes,
        qualities,
        storylets: Some(storylets),
    }
}

#[test]
fn test_if_loading() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    let result = engine.load_script(&ron_str);
    assert!(result.is_ok());
}

#[test]
fn test_if_initial_node() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    let response = engine.process_input("").unwrap();
    assert!(response.text.contains("You stand at the entrance of a dark cave"));
    assert_eq!(response.choices.len(), 3);
}

#[test]
fn test_if_quality_system() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Check initial qualities
    assert_eq!(engine.get_state("health"), Some(&Value::Integer(100)));
    assert_eq!(engine.get_state("courage"), Some(&Value::Integer(5)));
    assert_eq!(engine.get_state("gold"), Some(&Value::Integer(10)));
}

#[test]
fn test_if_conditional_choices() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Set low courage
    engine.set_state("courage", Value::Integer(2));
    
    let response = engine.process_input("").unwrap();
    
    // First choice requires courage >= 3, should be disabled
    assert!(!response.choices[0].enabled);
    assert!(response.choices[1].enabled); // Cautious entry
    assert!(response.choices[2].enabled); // Run away
}

#[test]
fn test_if_quality_changes() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Choose brave entry (first choice)
    engine.process_input("1").unwrap();
    
    // Courage should have increased
    assert_eq!(engine.get_state("courage"), Some(&Value::Integer(6)));
}

#[test]
fn test_if_quality_caps() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Try to set health above max
    engine.set_state("health", Value::Integer(150));
    // Should be capped at 100
    assert_eq!(engine.get_state("health"), Some(&Value::Integer(100)));
    
    // Try to set health below min
    engine.set_state("health", Value::Integer(-50));
    // Should be capped at 0
    assert_eq!(engine.get_state("health"), Some(&Value::Integer(0)));
}

#[test]
fn test_if_flags() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Enter cave cautiously
    engine.process_input("2").unwrap();
    
    // Should have set entered_cave flag
    assert_eq!(engine.get_state("flag_entered_cave"), Some(&Value::Bool(true)));
}

#[test]
fn test_if_navigation() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Navigate through the story
    engine.process_input("2").unwrap(); // Enter cautiously
    assert_eq!(engine.get_state("if_current_node"), 
               Some(&Value::String("cave_entrance".to_string())));
    
    engine.process_input("1").unwrap(); // Take left path
    assert_eq!(engine.get_state("if_current_node"),
               Some(&Value::String("treasure_room".to_string())));
}

#[test]
fn test_if_multiple_consequences() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Navigate to treasure room
    engine.process_input("2").unwrap(); // Enter cautiously
    engine.process_input("1").unwrap(); // Left path
    
    // Take all gold (should change gold and reputation)
    engine.process_input("1").unwrap();
    
    assert_eq!(engine.get_state("gold"), Some(&Value::Integer(110))); // 10 + 100
    assert_eq!(engine.get_state("reputation"), Some(&Value::Integer(-10))); // 0 - 10
}

#[test]
fn test_if_combat_path() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Set high courage for combat
    engine.set_state("courage", Value::Integer(6));
    
    // Navigate to monster
    engine.process_input("2").unwrap(); // Enter cautiously
    engine.process_input("2").unwrap(); // Right path to monster
    
    // Fight option should be available
    let response = engine.process_input("").unwrap();
    assert!(response.choices[0].enabled); // Fight option
    
    // Fight the monster
    engine.process_input("1").unwrap();
    
    assert_eq!(engine.get_state("health"), Some(&Value::Integer(70))); // 100 - 30
    assert_eq!(engine.get_state("flag_defeated_monster"), Some(&Value::Bool(true)));
}

#[test]
fn test_if_endings() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Choose coward ending
    let response = engine.process_input("3").unwrap(); // Run away
    
    assert!(response.text.contains("Perhaps adventure isn't for you"));
    assert!(response.choices.is_empty()); // No more choices
    assert_eq!(engine.get_state("flag_game_over"), Some(&Value::Bool(true)));
}

#[test]
fn test_if_save_and_load() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script_1 = GameScript::InteractiveFiction(script.clone());
    let ron_str_1 = game_script_1.to_ron().unwrap();
    engine.load_script(&ron_str_1).unwrap();
    
    // Progress through story
    engine.process_input("2").unwrap(); // Enter cautiously
    engine.set_state("gold", Value::Integer(25));
    engine.set_state("flag_custom_flag", Value::Bool(true));
    
    // Save
    let save_response = engine.save_game(Some(0)).unwrap();
    assert!(save_response.success);
    
    // New engine
    let mut new_engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    new_engine.load_script(&ron_str).unwrap();
    let load_response = new_engine.load_game(Some(0)).unwrap();
    assert!(load_response.success);
    
    // Verify state restored
    assert_eq!(new_engine.get_state("if_current_node"),
               Some(&Value::String("cave_entrance".to_string())));
    assert_eq!(new_engine.get_state("gold"), Some(&Value::Integer(25)));
    assert_eq!(new_engine.get_state("flag_custom_flag"), Some(&Value::Bool(true)));
}

#[test]
fn test_if_invalid_choice() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Try invalid choice number
    let result = engine.process_input("99");
    assert!(result.is_err() || !result.unwrap().success);
}

#[test]
fn test_if_disabled_choice() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Set low courage
    engine.set_state("courage", Value::Integer(2));
    
    // Try to select disabled choice (requires courage >= 3)
    let result = engine.process_input("1");
    assert!(result.is_err() || !result.unwrap().success);
}

#[test]
fn test_if_storylet_system() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Set conditions for storylet
    engine.set_state("flag_entered_cave", Value::Bool(true));
    engine.set_state("gold", Value::Integer(25));
    
    // Check if storylet is available (would need a way to query available storylets)
    // For now, just verify the storylets were loaded
    assert!(engine.get_state("if_storylets").is_some());
}

#[test]
fn test_if_quality_with_no_max() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Gold has no max, should allow high values
    engine.set_state("gold", Value::Integer(9999));
    assert_eq!(engine.get_state("gold"), Some(&Value::Integer(9999)));
}

#[test]
fn test_if_negative_quality() {
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::InteractiveFiction,
        ..Default::default()
    });
    
    let script = create_test_if_script();
    let game_script = GameScript::InteractiveFiction(script);
    let ron_str = game_script.to_ron().unwrap();
    engine.load_script(&ron_str).unwrap();
    
    // Reputation can go negative (min -100)
    engine.set_state("reputation", Value::Integer(-50));
    assert_eq!(engine.get_state("reputation"), Some(&Value::Integer(-50)));
    
    // But not below min
    engine.set_state("reputation", Value::Integer(-150));
    assert_eq!(engine.get_state("reputation"), Some(&Value::Integer(-100)));
}