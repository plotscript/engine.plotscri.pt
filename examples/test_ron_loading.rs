//! Test program to verify RON script loading works

use plotscript::{Engine, init};

fn main() {
    // Initialize the engine
    init();
    
    // Read the adventure.ron file
    let script_content = std::fs::read_to_string("examples/adventure.ron")
        .expect("Failed to read adventure.ron");
    
    // Create engine and load the script
    let mut engine = Engine::new();
    
    match engine.load_script(&script_content) {
        Ok(_) => {
            println!("✓ Successfully loaded RON script!");
            
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
}