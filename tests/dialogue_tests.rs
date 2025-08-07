//! Tests for the text adventure dialogue system

use plotscript::{Engine, EngineConfig, GameMode};
use plotscript::script::GameScript;

#[test]
fn test_dialogue_system_basic() {
    let script_source = r#"
        GameScript::TextAdventure(TextAdventureScript {
            title: "Dialogue Test",
            author: Some("Test Author"),
            version: Some("1.0.0"),
            description: Some("Testing dialogue"),
            settings: TextAdventureSettings {
                starting_location: "tavern",
                max_score: Some(100),
                enable_compass: true,
                enable_score: false,
            },
            locations: {
                "tavern": Location {
                    name: "The Rusty Anchor",
                    description: "A cozy tavern with a warm fireplace.",
                    exits: {},
                    items: [],
                    characters: ["bartender"],
                    dark: false,
                    visited: false,
                },
            },
            items: {},
            characters: {
                "bartender": Character {
                    name: "Bob the Bartender",
                    description: "A friendly bartender with a warm smile.",
                    dialogue: Some(DialogueTree {
                        start: "greeting",
                        nodes: {
                            "greeting": DialogueNode {
                                text: "Welcome to the Rusty Anchor! What can I get you?",
                                responses: Some([
                                    DialogueResponse {
                                        text: "I'd like some ale.",
                                        next: "ale_order",
                                        conditions: None,
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "Any news from town?",
                                        next: "town_news",
                                        conditions: None,
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "Nothing, thanks.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "ale_order": DialogueNode {
                                text: "Coming right up! That'll be 5 gold pieces.",
                                responses: Some([
                                    DialogueResponse {
                                        text: "Here you go.",
                                        next: "end",
                                        conditions: Some([Condition::HasVariable("gold", Value::Integer(5))]),
                                        actions: Some([Action::SetVariable("gold", Value::Integer(0))]),
                                    },
                                    DialogueResponse {
                                        text: "I don't have enough gold.",
                                        next: "no_money",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "town_news": DialogueNode {
                                text: "Well, there's been talk of strange noises coming from the old mill at night.",
                                responses: Some([
                                    DialogueResponse {
                                        text: "Interesting. Tell me more.",
                                        next: "more_news",
                                        conditions: None,
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "Thanks for the info.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "more_news": DialogueNode {
                                text: "Some say it's haunted, others think bandits are using it as a hideout.",
                                responses: Some([
                                    DialogueResponse {
                                        text: "I'll check it out.",
                                        next: "end",
                                        conditions: None,
                                        actions: Some([Action::SetFlag("knows_about_mill")]),
                                    },
                                    DialogueResponse {
                                        text: "Not my problem.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "no_money": DialogueNode {
                                text: "No worries, come back when you have the coin.",
                                responses: None,
                                actions: None,
                                next: Some("end"),
                            },
                        },
                    }),
                    inventory: None,
                    events: None,
                },
            },
            vocabulary: None,
            events: {},
            variables: {
                "gold": Value::Integer(10),
            },
            functions: {},
        })
    "#;
    
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    // Load the script
    let script = ron::from_str::<GameScript>(script_source).unwrap();
    let script_str = ron::to_string(&script).unwrap();
    assert!(engine.load_script(&script_str).is_ok());
    
    // Start the game
    let response = engine.start().unwrap();
    assert!(response.text.contains("Rusty Anchor"));
    
    // Talk to the bartender
    let response = engine.process_input("talk to bartender").unwrap();
    assert!(response.text.contains("Welcome to the Rusty Anchor"));
    assert_eq!(response.choices.len(), 3);
    assert_eq!(response.choices[0].text, "I'd like some ale.");
    assert_eq!(response.choices[1].text, "Any news from town?");
    assert_eq!(response.choices[2].text, "Nothing, thanks.");
    
    // Choose to ask about news
    let response = engine.process_input("2").unwrap();
    assert!(response.text.contains("strange noises"));
    assert_eq!(response.choices.len(), 2);
    
    // Ask for more info
    let response = engine.process_input("1").unwrap();
    assert!(response.text.contains("haunted"));
    assert_eq!(response.choices.len(), 2);
    
    // Agree to check it out
    let response = engine.process_input("1").unwrap();
    assert!(response.text.contains("conversation ends"));
    
    // Verify the flag was set through the game state
    let state = response.state.unwrap();
    assert!(state.flags.contains("knows_about_mill"));
}

#[test]
fn test_dialogue_conditions() {
    let script_source = r#"
        GameScript::TextAdventure(TextAdventureScript {
            title: "Conditional Dialogue Test",
            author: Some("Test Author"),
            version: Some("1.0.0"),
            description: Some("Testing conditional dialogue"),
            settings: TextAdventureSettings {
                starting_location: "shop",
                max_score: None,
                enable_compass: false,
                enable_score: false,
            },
            locations: {
                "shop": Location {
                    name: "Magic Shop",
                    description: "A shop full of mystical items.",
                    exits: {},
                    items: [],
                    characters: ["shopkeeper"],
                    dark: false,
                    visited: false,
                },
            },
            items: {},
            characters: {
                "shopkeeper": Character {
                    name: "Merlin",
                    description: "A wise old wizard.",
                    dialogue: Some(DialogueTree {
                        start: "greeting",
                        nodes: {
                            "greeting": DialogueNode {
                                text: "Welcome to my shop!",
                                responses: Some([
                                    DialogueResponse {
                                        text: "I'd like to buy a potion. (10 gold)",
                                        next: "buy_potion",
                                        conditions: Some([Condition::HasVariable("gold", Value::Integer(10))]),
                                        actions: Some([
                                            Action::SetVariable("gold", Value::Integer(0)),
                                            Action::SetVariable("potions", Value::Integer(1)),
                                        ]),
                                    },
                                    DialogueResponse {
                                        text: "I need a special spell.",
                                        next: "special_spell",
                                        conditions: Some([Condition::HasFlag("trusted_customer")]),
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "Just browsing.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "buy_potion": DialogueNode {
                                text: "Excellent choice! This potion will serve you well.",
                                responses: None,
                                actions: None,
                                next: Some("end"),
                            },
                            "special_spell": DialogueNode {
                                text: "Ah, I have just the thing for a trusted customer like you!",
                                responses: None,
                                actions: Some([Action::SetFlag("has_special_spell")]),
                                next: Some("end"),
                            },
                        },
                    }),
                    inventory: None,
                    events: None,
                },
            },
            vocabulary: None,
            events: {},
            variables: {
                "gold": Value::Integer(5),
                "potions": Value::Integer(0),
            },
            functions: {},
        })
    "#;
    
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    // Load the script
    let script = ron::from_str::<GameScript>(script_source).unwrap();
    let script_str = ron::to_string(&script).unwrap();
    assert!(engine.load_script(&script_str).is_ok());
    
    // Start the game
    engine.start().unwrap();
    
    // Talk to shopkeeper
    let response = engine.process_input("talk to shopkeeper").unwrap();
    assert!(response.text.contains("Welcome to my shop"));
    
    // Check that we have only 2 choices (not enough gold for potion, not trusted)
    assert_eq!(response.choices.len(), 3);
    assert!(!response.choices[0].enabled); // Can't buy potion (not enough gold)
    assert!(!response.choices[1].enabled); // Can't get special spell (not trusted)
    assert!(response.choices[2].enabled);  // Can browse
    
    // End conversation
    engine.process_input("3").unwrap();
    
    // Since we can't modify state directly, let's reload with more gold
    // Create a new engine with more gold and trust flag
    let script_source_rich = script_source.replace(
        r#""gold": Value::Integer(5)"#,
        r#""gold": Value::Integer(15)"#
    ).replace(
        "variables: {",
        r#"variables: {
                "trusted_customer": Value::Bool(true),"#
    );
    
    let mut engine2 = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    let script2 = ron::from_str::<GameScript>(&script_source_rich).unwrap();
    let script_str2 = ron::to_string(&script2).unwrap();
    assert!(engine2.load_script(&script_str2).is_ok());
    engine2.start().unwrap();
    
    // Talk to shopkeeper with more gold and trust
    let response = engine2.process_input("talk to shopkeeper").unwrap();
    assert_eq!(response.choices.len(), 3);
    // Note: Since we use flags instead of variables for trust, the second choice still won't be enabled
    assert!(response.choices[0].enabled);  // Can buy potion now (have gold)
    assert!(!response.choices[1].enabled); // Still can't get special spell (need flag, not variable)
    assert!(response.choices[2].enabled);  // Can browse
    
    // Buy the potion
    let response = engine2.process_input("1").unwrap();
    assert!(response.text.contains("Excellent choice"));
    
    // Check that gold was spent and potion was gained through the game state
    let final_state = response.state.unwrap();
    // We started with 15 gold and spent 10, so should have 5 left
    assert_eq!(final_state.variables.get("gold"), Some(&plotscript::types::Value::Integer(5)));
    assert_eq!(final_state.variables.get("potions"), Some(&plotscript::types::Value::Integer(1)));
}

#[test]
fn test_dialogue_exit() {
    let script_source = r#"
        GameScript::TextAdventure(TextAdventureScript {
            title: "Exit Dialogue Test",
            author: Some("Test Author"),
            version: Some("1.0.0"),
            description: Some("Testing dialogue exit"),
            settings: TextAdventureSettings {
                starting_location: "room",
                max_score: None,
                enable_compass: false,
                enable_score: false,
            },
            locations: {
                "room": Location {
                    name: "A Room",
                    description: "Just a room.",
                    exits: {},
                    items: [],
                    characters: ["npc"],
                    dark: false,
                    visited: false,
                },
            },
            items: {},
            characters: {
                "npc": Character {
                    name: "NPC",
                    description: "A non-player character.",
                    dialogue: Some(DialogueTree {
                        start: "chat",
                        nodes: {
                            "chat": DialogueNode {
                                text: "Let's have a long conversation...",
                                responses: Some([
                                    DialogueResponse {
                                        text: "Sure, let's talk.",
                                        next: "more_chat",
                                        conditions: None,
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "Actually, I need to go.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                            "more_chat": DialogueNode {
                                text: "So much to discuss!",
                                responses: Some([
                                    DialogueResponse {
                                        text: "Keep talking.",
                                        next: "chat",
                                        conditions: None,
                                        actions: None,
                                    },
                                    DialogueResponse {
                                        text: "I'm done.",
                                        next: "end",
                                        conditions: None,
                                        actions: None,
                                    },
                                ]),
                                actions: None,
                                next: None,
                            },
                        },
                    }),
                    inventory: None,
                    events: None,
                },
            },
            vocabulary: None,
            events: {},
            variables: {},
            functions: {},
        })
    "#;
    
    let mut engine = Engine::with_config(EngineConfig {
        mode: GameMode::TextAdventure,
        ..Default::default()
    });
    
    // Load the script
    let script = ron::from_str::<GameScript>(script_source).unwrap();
    let script_str = ron::to_string(&script).unwrap();
    assert!(engine.load_script(&script_str).is_ok());
    
    // Start the game
    engine.start().unwrap();
    
    // Start dialogue
    let response = engine.process_input("talk to npc").unwrap();
    assert!(response.text.contains("long conversation"));
    
    // Use "exit" command to leave dialogue early
    let response = engine.process_input("exit").unwrap();
    assert!(response.text.contains("end the conversation"));
    
    // Verify we can do other commands now
    let response = engine.process_input("look").unwrap();
    assert!(response.text.contains("Just a room"));
    
    // Start dialogue again
    engine.process_input("talk to npc").unwrap();
    
    // Use "leave" command to exit
    let response = engine.process_input("leave").unwrap();
    assert!(response.text.contains("end the conversation"));
}