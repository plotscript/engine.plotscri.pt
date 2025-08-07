//! Example: Mystery Manor - A Text Adventure
//! 
//! This example demonstrates a complete text adventure game with:
//! - Multiple rooms with descriptions
//! - Items that can be taken and used
//! - Characters with dialogue trees
//! - Puzzles and locked doors
//! - Save/load functionality

use plotscript::{Engine, EngineConfig, GameMode, init};
use std::io::{self, Write};

const GAME_SCRIPT: &str = r#"
TextAdventure((
    title: "Mystery Manor",
    author: "PlotScript Examples",
    description: Some("A mysterious manor holds dark secrets..."),
    version: Some("1.0.0"),
    
    settings: (
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: true,
        inventory_limits: false,
        max_inventory: None,
    ),
    
    starting_location: "entrance",
    
    locations: {
        "entrance": (
            name: "Manor Entrance",
            description: "You stand before a grand Victorian manor. The heavy oak door looms before you, its brass handle gleaming despite years of neglect. Thunder rumbles overhead.",
            exits: {
                north: "foyer",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: Some("The manor seems to be expecting you..."),
            events: None,
        ),
        "foyer": (
            name: "Grand Foyer",
            description: "A magnificent foyer stretches before you. A crystal chandelier hangs from the ceiling, casting dancing shadows. Doors lead in all directions.",
            exits: {
                north: "library",
                east: "dining_room",
                west: "parlor",
                south: "entrance",
                up: "upstairs_hall",
            },
            items: ["brass_key"],
            characters: ["butler"],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "library": (
            name: "Library",
            description: "Floor-to-ceiling bookshelves line the walls. The smell of old paper fills the air. A large desk sits in the center of the room.",
            exits: {
                south: "foyer",
            },
            items: ["ancient_tome", "letter"],
            characters: [],
            dark: Some(false),
            first_visit: Some("You notice one book seems out of place..."),
            events: None,
        ),
        "dining_room": (
            name: "Dining Room",
            description: "A long table dominates the room, set for a dinner that never came. Dusty plates and tarnished silver remain untouched.",
            exits: {
                west: "foyer",
                north: "kitchen",
            },
            items: ["silver_knife"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "kitchen": (
            name: "Kitchen",
            description: "An old-fashioned kitchen with a large stove and preparation area. Everything is remarkably clean.",
            exits: {
                south: "dining_room",
                down: "cellar",
            },
            items: ["lantern", "matches"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "cellar": (
            name: "Cellar",
            description: "A damp, musty cellar. Wine racks line the walls, most bottles covered in dust. It's very dark here.",
            exits: {
                up: "kitchen",
            },
            items: ["old_wine", "rusty_key"],
            characters: [],
            dark: Some(true),
            first_visit: Some("You can barely see anything in this darkness..."),
            events: None,
        ),
        "parlor": (
            name: "Parlor",
            description: "A cozy parlor with comfortable chairs and a fireplace. Family portraits line the walls, their eyes seeming to follow you.",
            exits: {
                east: "foyer",
            },
            items: ["photo_album"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "upstairs_hall": (
            name: "Upstairs Hallway",
            description: "A long hallway with several doors. Faded carpet muffles your footsteps.",
            exits: {
                down: "foyer",
                north: "master_bedroom",
                east: "guest_room",
                west: "study",
            },
            items: [],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "master_bedroom": (
            name: "Master Bedroom",
            description: "A luxurious bedroom with a four-poster bed. Everything is covered in a thin layer of dust.",
            exits: {
                south: "upstairs_hall",
            },
            items: ["jewelry_box", "diary"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
        "study": (
            name: "Study",
            description: "A private study filled with papers and books. A large safe sits in the corner.",
            exits: {
                east: "upstairs_hall",
            },
            items: ["safe"],
            characters: [],
            dark: Some(false),
            first_visit: Some("The safe requires a combination..."),
            events: None,
        ),
        "guest_room": (
            name: "Guest Room",
            description: "A modest guest room. The bed is still made, waiting for visitors who never arrived.",
            exits: {
                west: "upstairs_hall",
            },
            items: ["note"],
            characters: [],
            dark: Some(false),
            first_visit: None,
            events: None,
        ),
    },
    
    items: {
        "brass_key": (
            name: "Brass Key",
            description: "A small brass key with intricate engravings.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "ancient_tome": (
            name: "Ancient Tome",
            description: "A leather-bound book written in an ancient language. Some passages are underlined.",
            takeable: true,
            weight: Some(5),
            container: None,
            contains: None,
            openable: Some(true),
            locked: None,
            key: None,
            events: None,
        ),
        "letter": (
            name: "Letter",
            description: "A yellowed letter. It reads: 'The truth lies within the wine cellar. Beware the darkness.' It's signed 'E.B.'",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "silver_knife": (
            name: "Silver Knife",
            description: "An ornate silver knife from the dining set. Still sharp despite its age.",
            takeable: true,
            weight: Some(2),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "lantern": (
            name: "Lantern",
            description: "An old oil lantern. It looks like it still works.",
            takeable: true,
            weight: Some(3),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "matches": (
            name: "Matches",
            description: "A box of matches. Still dry and usable.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "old_wine": (
            name: "Old Wine",
            description: "A dusty bottle of wine from 1885. The label mentions something about 'the combination'.",
            takeable: true,
            weight: Some(3),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "rusty_key": (
            name: "Rusty Key",
            description: "An old, rusty key. It might still work in the right lock.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "photo_album": (
            name: "Photo Album",
            description: "A family photo album. The dates range from 1880 to 1885, then abruptly stop.",
            takeable: true,
            weight: Some(4),
            container: None,
            contains: None,
            openable: Some(true),
            locked: None,
            key: None,
            events: None,
        ),
        "jewelry_box": (
            name: "Jewelry Box",
            description: "An ornate jewelry box. It's locked.",
            takeable: true,
            weight: Some(2),
            container: Some(true),
            contains: Some(["golden_locket"]),
            openable: Some(true),
            locked: Some(true),
            key: Some("brass_key"),
            events: None,
        ),
        "golden_locket": (
            name: "Golden Locket",
            description: "A beautiful golden locket. Inside is a photo of a young woman and the numbers '1885'.",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "diary": (
            name: "Diary",
            description: "A personal diary. The last entry speaks of 'hiding the family treasure in the safe'.",
            takeable: true,
            weight: Some(2),
            container: None,
            contains: None,
            openable: Some(true),
            locked: None,
            key: None,
            events: None,
        ),
        "safe": (
            name: "Safe",
            description: "A large, sturdy safe with a combination lock.",
            takeable: false,
            weight: Some(1000),
            container: Some(true),
            contains: Some(["treasure"]),
            openable: Some(true),
            locked: Some(true),
            key: Some("1885"),
            events: None,
        ),
        "treasure": (
            name: "Family Treasure",
            description: "A collection of gold coins and precious gems. You've found the Blackwood family treasure!",
            takeable: true,
            weight: Some(10),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
        "note": (
            name: "Note",
            description: "A hastily written note: 'The butler knows more than he lets on. - E.B.'",
            takeable: true,
            weight: Some(1),
            container: None,
            contains: None,
            openable: None,
            locked: None,
            key: None,
            events: None,
        ),
    },
    
    characters: {
        "butler": (
            name: "Jeeves",
            description: "An elderly butler in impeccable attire. His eyes hold secrets.",
            dialogue: Some((
                start: "greeting",
                nodes: {
                    "greeting": (
                        text: "Good evening. I've been expecting you. The master... well, the master has been gone for quite some time.",
                        responses: Some([
                            (text: "What happened here?", next: "history"),
                            (text: "Who are you?", next: "identity"),
                            (text: "Where is everyone?", next: "missing"),
                        ]),
                        actions: None,
                        next: None,
                    ),
                    "history": (
                        text: "The Blackwood family lived here for generations. Then, in 1885, they all... disappeared. Some say it was the family curse.",
                        responses: Some([
                            (text: "Tell me about the curse", next: "curse"),
                            (text: "What do you think happened?", next: "theory"),
                        ]),
                        actions: None,
                        next: None,
                    ),
                    "identity": (
                        text: "I am Jeeves, the last remaining servant of the Blackwood estate. I maintain the manor, waiting for... well, waiting.",
                        responses: Some([
                            (text: "Waiting for what?", next: "waiting"),
                        ]),
                        actions: None,
                        next: None,
                    ),
                    "missing": (
                        text: "They vanished one autumn night in 1885. The police found nothing. Only I remained, bound by duty.",
                        responses: Some([
                            (text: "Why did you stay?", next: "duty"),
                        ]),
                        actions: None,
                        next: None,
                    ),
                    "curse": (
                        text: "They say the family was cursed for their greed. The treasure they hoarded would be their downfall. Perhaps it's still hidden somewhere in the manor.",
                        responses: None,
                        actions: None,
                        next: None,
                    ),
                    "theory": (
                        text: "I believe they found something in the cellar. Something that should have remained hidden. The wine cellar holds many secrets.",
                        responses: None,
                        actions: None,
                        next: None,
                    ),
                    "waiting": (
                        text: "For someone to solve the mystery. To bring peace to this place. Perhaps... perhaps that someone is you.",
                        responses: None,
                        actions: None,
                        next: None,
                    ),
                    "duty": (
                        text: "A Jeeves never abandons his post. Besides, someone must guard the secrets of this place.",
                        responses: None,
                        actions: None,
                        next: None,
                    ),
                },
            )),
            inventory: None,
            events: None,
        ),
    },
    
    vocabulary: None,
    events: None,
))
"#;

fn main() {
    // Initialize the engine
    init();
    
    println!("=== Mystery Manor ===");
    println!("A PlotScript Text Adventure Example");
    println!();
    
    // Create engine with text adventure configuration
    let config = EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 70,
        auto_save: false,
        ..Default::default()
    };
    
    let mut engine = Engine::with_config(config);
    
    // Load the game script
    match engine.load_script(GAME_SCRIPT) {
        Ok(_) => println!("Game loaded successfully!\n"),
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
    
    // Game loop
    loop {
        // Prompt for input
        print!("> ");
        io::stdout().flush().unwrap();
        
        // Read player input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Handle meta commands
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Thanks for playing!");
                break;
            }
            "save" => {
                match engine.save_game(Some(1)) {
                    Ok(_) => println!("Game saved to slot 1.\n"),
                    Err(e) => println!("Failed to save game: {}\n", e),
                }
                continue;
            }
            "load" => {
                match engine.load_game(Some(1)) {
                    Ok(response) => println!("{}\n", response.text),
                    Err(e) => println!("Failed to load game: {}\n", e),
                }
                continue;
            }
            "help" => {
                print_help();
                continue;
            }
            _ => {}
        }
        
        // Process game command
        match engine.process_input(input) {
            Ok(response) => {
                println!("{}\n", response.text);
                
                // Check if game ended
                if response.ended {
                    println!("=== THE END ===");
                    println!("Thanks for playing Mystery Manor!");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
    }
}

fn print_help() {
    println!("=== HELP ===");
    println!("Basic commands:");
    println!("  look/l - Look around");
    println!("  go [direction] - Move in a direction (north/n, south/s, east/e, west/w, up/u, down/d)");
    println!("  take/get [item] - Pick up an item");
    println!("  drop [item] - Drop an item");
    println!("  inventory/i - Check your inventory");
    println!("  examine/x [thing] - Examine something closely");
    println!("  use [item] - Use an item");
    println!("  use [item] on [target] - Use an item on something");
    println!("  talk to [character] - Talk to someone");
    println!();
    println!("Meta commands:");
    println!("  save - Save your game");
    println!("  load - Load your saved game");
    println!("  help - Show this help");
    println!("  quit/exit - Exit the game");
    println!();
    println!("TIP: The parser understands typos and variations!");
    println!();
}