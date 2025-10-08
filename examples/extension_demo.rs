//! Example: Extension System Demo
//! 
//! This example demonstrates how to create and use custom extensions
//! to add new commands and functionality to PlotScript games.

use plotscript::{
    Engine, EngineConfig, GameMode, init,
    extensions::{Extension, ExtensionMetadata, FunctionCondition, FunctionAction},
    Response, Result,
    types::{GameState, Value},
};
use std::io::{self, Write};

/// Combat extension that adds RPG-style combat
struct CombatExtension {
    in_combat: bool,
    enemy_health: i32,
    enemy_name: String,
}

impl CombatExtension {
    fn new() -> Self {
        Self {
            in_combat: false,
            enemy_health: 0,
            enemy_name: String::new(),
        }
    }
    
    fn start_combat(&mut self, enemy: &str, health: i32) -> Response {
        self.in_combat = true;
        self.enemy_health = health;
        self.enemy_name = enemy.to_string();
        
        Response {
            text: format!("A {} appears! It has {} health points.", enemy, health),
            success: true,
            ..Default::default()
        }
    }
    
    fn attack(&mut self, state: &mut GameState) -> Response {
        if !self.in_combat {
            return Response {
                text: "There's nothing to attack.".to_string(),
                success: false,
                ..Default::default()
            };
        }
        
        // Player damage (based on strength)
        let player_strength = match state.get_variable("strength") {
            Some(Value::Integer(s)) => *s,
            _ => 5,
        };
        
        let damage = (rand::random::<i32>() % (player_strength as i32)) + 1;
        self.enemy_health -= damage;
        
        let mut text = format!("You deal {} damage to the {}!", damage, self.enemy_name);
        
        if self.enemy_health <= 0 {
            text.push_str(&format!("\nThe {} is defeated!", self.enemy_name));
            self.in_combat = false;
            
            // Give experience
            let exp = match state.get_variable("experience") {
                Some(Value::Integer(e)) => e + 10,
                _ => 10,
            };
            state.set_variable("experience", Value::Integer(exp));
            text.push_str(&format!("\nYou gained 10 experience! Total: {}", exp));
        } else {
            // Enemy counter-attack
            let enemy_damage = rand::random::<i32>() % 5 + 1;
            let player_health = match state.get_variable("health") {
                Some(Value::Integer(h)) => h - (enemy_damage as i64),
                _ => 100 - (enemy_damage as i64),
            };
            state.set_variable("health", Value::Integer(player_health));
            
            text.push_str(&format!("\nThe {} attacks back for {} damage!", self.enemy_name, enemy_damage));
            text.push_str(&format!("\nEnemy health: {} | Your health: {}", self.enemy_health, player_health));
            
            if player_health <= 0 {
                text.push_str("\nYou have been defeated!");
                state.set_flag("game_over");
            }
        }
        
        Response {
            text,
            success: true,
            ..Default::default()
        }
    }
}

impl Extension for CombatExtension {
    fn name(&self) -> &str {
        "combat"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn get_verbs(&self) -> Vec<&str> {
        vec!["fight", "attack", "flee", "stats"]
    }
    
    fn on_load(&mut self, engine: &mut Engine) -> Result<()> {
        // Initialize combat stats
        engine.state.set_variable("health", Value::Integer(100));
        engine.state.set_variable("strength", Value::Integer(10));
        engine.state.set_variable("experience", Value::Integer(0));
        engine.state.set_variable("level", Value::Integer(1));
        Ok(())
    }
    
    fn process_command(&mut self, command: &str, args: &[&str], state: &mut GameState) -> Option<Response> {
        match command {
            "fight" => {
                if args.is_empty() {
                    return Some(Response {
                        text: "Fight what?".to_string(),
                        success: false,
                        ..Default::default()
                    });
                }
                
                // Random enemy health
                let health = rand::random::<i32>() % 50 + 20;
                Some(self.start_combat(args[0], health))
            }
            "attack" => {
                Some(self.attack(state))
            }
            "flee" => {
                if self.in_combat {
                    self.in_combat = false;
                    Some(Response {
                        text: format!("You flee from the {}!", self.enemy_name),
                        success: true,
                        ..Default::default()
                    })
                } else {
                    Some(Response {
                        text: "There's nothing to flee from.".to_string(),
                        success: false,
                        ..Default::default()
                    })
                }
            }
            "stats" => {
                let health = state.get_variable("health").and_then(|v| match v {
                    Value::Integer(i) => Some(i),
                    _ => None,
                }).unwrap_or(&100);
                
                let strength = state.get_variable("strength").and_then(|v| match v {
                    Value::Integer(i) => Some(i),
                    _ => None,
                }).unwrap_or(&10);
                
                let exp = state.get_variable("experience").and_then(|v| match v {
                    Value::Integer(i) => Some(i),
                    _ => None,
                }).unwrap_or(&0);
                
                let level = state.get_variable("level").and_then(|v| match v {
                    Value::Integer(i) => Some(i),
                    _ => None,
                }).unwrap_or(&1);
                
                Some(Response {
                    text: format!(
                        "=== Character Stats ===\nHealth: {}\nStrength: {}\nLevel: {}\nExperience: {}",
                        health, strength, level, exp
                    ),
                    success: true,
                    ..Default::default()
                })
            }
            _ => None,
        }
    }
    
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            name: self.name().to_string(),
            version: self.version().to_string(),
            author: "PlotScript Examples".to_string(),
            description: "Adds RPG-style combat to text adventures".to_string(),
            dependencies: vec![],
        }
    }
}

const GAME_SCRIPT: &str = r#"
TextAdventure((
    title: "Extension Demo",
    author: "PlotScript Examples",
    description: Some("A demo of the extension system"),
    version: Some("1.0.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: false,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "forest",
    
    locations: {
        "forest": (
            name: "Dark Forest",
            description: "You're in a dark, mysterious forest. Strange sounds echo through the trees.",
            exits: {
                north: "clearing",
                east: "cave",
            },
            items: ["sword", "potion"],
            characters: [],
            dark: Some(false),
            first_visit: Some("This forest seems dangerous..."),
            events: None,
        ),
        "clearing": (
            name: "Forest Clearing",
            description: "A peaceful clearing in the forest. Sunlight filters through the canopy.",
            exits: {
                south: "forest",
            },
            items: ["flower"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "cave": (
            name: "Damp Cave",
            description: "A damp cave that descends into darkness. You hear growling from within.",
            exits: {
                west: "forest",
            },
            items: ["treasure"],
            characters: [],
            dark: Some(true),
            first_visit: Some("Something dangerous lurks here..."),
            events: None,
        ),
    },
    
    items: {
        "sword": (
            name: "Iron Sword",
            description: "A sturdy iron sword. It increases your strength.",
            takeable: true,
            weight: Some(5),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "potion": (
            name: "Health Potion",
            description: "A red potion that restores health.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "flower": (
            name: "Magic Flower",
            description: "A beautiful flower that glows with inner light.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "treasure": (
            name: "Ancient Treasure",
            description: "A chest full of gold and jewels!",
            takeable: false,
            weight: Some(100),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
    },
    
    characters: {},
    vocabulary: None,
    events: None,
))
"#;

fn main() {
    // Initialize the engine
    init();
    
    println!("=== PlotScript Extension Demo ===");
    println!("This game demonstrates custom extensions!");
    println!();
    
    // Create engine
    let config = EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        ..Default::default()
    };
    
    let mut engine = Engine::with_config(config);
    
    // Register the combat extension
    let combat_ext = Box::new(CombatExtension::new());
    engine.register_extension(combat_ext).expect("Failed to register combat extension");
    
    // Register a custom condition: is player strong enough?
    let strong_condition = Box::new(FunctionCondition::new(
        "is_strong",
        1,
        |state: &GameState, args: &[Value]| {
            if let Some(Value::Integer(required)) = args.first() {
                if let Some(Value::Integer(strength)) = state.get_variable("strength") {
                    return Ok(strength >= required);
                }
            }
            Ok(false)
        },
    ));
    engine.register_condition(strong_condition).expect("Failed to register condition");
    
    // Register a custom action: level up
    let levelup_action = Box::new(FunctionAction::new(
        "levelup",
        0,
        |state: &mut GameState, _args: &[Value]| {
            let level = match state.get_variable("level") {
                Some(Value::Integer(l)) => l + 1,
                _ => 2,
            };
            state.set_variable("level", Value::Integer(level));
            state.set_variable("strength", Value::Integer(10 + (level * 2)));
            state.set_variable("health", Value::Integer(100 + (level * 10)));
            Ok(())
        },
    ));
    engine.register_action(levelup_action).expect("Failed to register action");
    
    // Load the game
    match engine.load_script(GAME_SCRIPT) {
        Ok(_) => println!("Game loaded successfully!"),
        Err(e) => {
            eprintln!("Failed to load game: {}", e);
            return;
        }
    }
    
    // Start the game
    match engine.start() {
        Ok(response) => {
            println!("{}\n", response.text);
        }
        Err(e) => {
            eprintln!("Failed to start game: {}", e);
            return;
        }
    }
    
    println!("Combat commands: fight [enemy], attack, flee, stats");
    println!("Standard commands: look, go [direction], take [item], inventory, help");
    println!();
    
    // Game loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Check for quit
        if input == "quit" || input == "exit" {
            println!("Thanks for playing!");
            break;
        }
        
        // Process input
        match engine.process_input(input) {
            Ok(response) => {
                println!("{}\n", response.text);
                
                // Check game over
                if engine.state.has_flag("game_over") {
                    println!("=== GAME OVER ===");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
    }
}