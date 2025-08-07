//! Test loading RON script directly without fallback

use plotscript::{Engine, init};

fn main() {
    // Initialize the engine
    init();
    
    // Read the adventure.ron file
    let script_content = std::fs::read_to_string("examples/adventure.ron")
        .expect("Failed to read adventure.ron");
    
    // Parse the RON directly
    match plotscript::script::GameScript::from_ron(&script_content) {
        Ok(game_script) => {
            println!("✓ Successfully parsed RON script!");
            println!("  Game type: {:?}", game_script.game_mode());
            
            // Create engine and load the script
            let mut engine = Engine::new();
            
            // Load using the internal method (we'd need to make this public or use load_script)
            match engine.load_script(&script_content) {
                Ok(_) => {
                    println!("✓ Successfully loaded script into engine!");
                    
                    // Try to start the game
                    match engine.start() {
                        Ok(response) => {
                            println!("\n--- Game Started ---");
                            println!("{}", response.text);
                            println!("Location: {:?}", response.location);
                        },
                        Err(e) => {
                            println!("✗ Failed to start game: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("✗ Failed to load script: {}", e);
                }
            }
        },
        Err(e) => {
            println!("✗ Failed to parse RON: {}", e);
        }
    }
}