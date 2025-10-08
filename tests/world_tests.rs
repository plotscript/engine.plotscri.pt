//! Comprehensive tests for the world model and state management

use plotscript::world::{World, WorldQuery};
use plotscript::parser::{Room, Item, Character, DialogueTree, Statement, Expression};
use plotscript::types::{Direction, Value};
use std::collections::HashMap;

#[test]
fn test_world_creation() {
    let world = World::new();
    assert!(world.rooms.is_empty());
    assert!(world.items.is_empty());
    assert!(world.characters.is_empty());
    assert!(world.inventory.is_empty());
    assert_eq!(world.turn_count, 0);
}

#[test]
fn test_add_room() {
    let mut world = World::new();
    
    let room = Room {
        id: "test_room".to_string(),
        title: Some("Test Room".to_string()),
        description: Some("A test room".to_string()),
        exits: HashMap::from([(Direction::North, "other_room".to_string())]),
        items: vec!["item1".to_string()],
        characters: vec!["char1".to_string()],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("test_room".to_string(), room).expect("Failed to add room");
    
    assert!(world.rooms.contains_key("test_room"));
    assert_eq!(world.rooms.len(), 1);
}

#[test]
fn test_player_movement() {
    let mut world = World::new();
    
    // Create two connected rooms
    let room1 = Room {
        id: "room1".to_string(),
        title: Some("Room 1".to_string()),
        description: Some("First room".to_string()),
        exits: HashMap::from([(Direction::East, "room2".to_string())]),
        items: vec![],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    let room2 = Room {
        id: "room2".to_string(),
        title: Some("Room 2".to_string()),
        description: Some("Second room".to_string()),
        exits: HashMap::from([(Direction::West, "room1".to_string())]),
        items: vec![],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("room1".to_string(), room1).expect("Failed to add room1");
    world.add_room("room2".to_string(), room2).expect("Failed to add room2");
    world.player_location = "room1".to_string();
    
    // Test movement
    let new_location = world.move_player(Direction::East).expect("Failed to move");
    assert_eq!(new_location, "room2");
    assert_eq!(world.player_location, "room2");
    
    // Test invalid movement
    let result = world.move_player(Direction::North);
    assert!(result.is_err());
}

#[test]
fn test_inventory_management() {
    let mut world = World::new();
    
    // Add an item
    let item = Item {
        id: "key".to_string(),
        name: Some("brass key".to_string()),
        description: Some("A shiny brass key".to_string()),
        location: Some("room1".to_string()),
        takeable: true,
        weight: 1.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::new(),
    };
    
    world.add_item("key".to_string(), item).expect("Failed to add item");
    
    // Create room and place player there
    let room = Room {
        id: "room1".to_string(),
        title: Some("Room".to_string()),
        description: Some("A room".to_string()),
        exits: HashMap::new(),
        items: vec!["key".to_string()],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("room1".to_string(), room).expect("Failed to add room");
    world.player_location = "room1".to_string();
    
    // Take the item
    world.take_item("key").expect("Failed to take item");
    assert!(world.inventory.contains(&"key".to_string()));
    assert_eq!(world.items.get("key").unwrap().location, None);
    
    // Drop the item
    world.drop_item("key").expect("Failed to drop item");
    assert!(!world.inventory.contains(&"key".to_string()));
    assert_eq!(world.items.get("key").unwrap().location, Some("room1".to_string()));
}

#[test]
fn test_world_query() {
    let mut world = World::new();
    
    // Add items with different names
    world.add_item("brass_key".to_string(), Item {
        id: "brass_key".to_string(),
        name: Some("brass key".to_string()),
        description: Some("A key".to_string()),
        location: Some("room1".to_string()),
        takeable: true,
        weight: 1.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::new(),
    }).expect("Failed to add brass key");
    
    world.add_item("iron_key".to_string(), Item {
        id: "iron_key".to_string(),
        name: Some("iron key".to_string()),
        description: Some("Another key".to_string()),
        location: Some("room1".to_string()),
        takeable: true,
        weight: 1.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::new(),
    }).expect("Failed to add iron key");
    
    // Create room
    let room = Room {
        id: "room1".to_string(),
        title: Some("Room".to_string()),
        description: Some("A room".to_string()),
        exits: HashMap::new(),
        items: vec!["brass_key".to_string(), "iron_key".to_string()],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("room1".to_string(), room).expect("Failed to add room");
    world.player_location = "room1".to_string();
    
    // Test query
    let query = WorldQuery::new(&world);
    
    // Find by exact name
    let result = query.find_item_here("brass key");
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, "brass_key");
    
    // Find by partial name
    let result = query.find_item_here("brass");
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, "brass_key");
    
    // Ambiguous query
    let result = query.find_item_here("key");
    assert!(result.is_some()); // Should return one of them
}

#[test]
fn test_container_items() {
    let mut world = World::new();
    
    // Create a container
    let chest = Item {
        id: "chest".to_string(),
        name: Some("wooden chest".to_string()),
        description: Some("A large chest".to_string()),
        location: Some("room1".to_string()),
        takeable: false,
        weight: 100.0,
        is_container: true,
        contains: vec!["gold".to_string()],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([
            ("open".to_string(), Value::Bool(true)),
            ("locked".to_string(), Value::Bool(false)),
        ]),
    };
    
    // Create contained item
    let gold = Item {
        id: "gold".to_string(),
        name: Some("gold coins".to_string()),
        description: Some("Shiny gold coins".to_string()),
        location: Some("chest".to_string()), // Inside the chest
        takeable: true,
        weight: 5.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::new(),
    };
    
    world.add_item("chest".to_string(), chest).expect("Failed to add chest");
    world.add_item("gold".to_string(), gold).expect("Failed to add gold");
    
    // Check containment
    assert_eq!(world.items.get("gold").unwrap().location, Some("chest".to_string()));
    assert!(world.items.get("chest").unwrap().contains.contains(&"gold".to_string()));
}

#[test]
fn test_variables_and_flags() {
    let mut world = World::new();
    
    // Test variables
    world.set_variable("score".to_string(), Value::Integer(10));
    assert_eq!(world.get_variable("score"), Some(&Value::Integer(10)));
    
    world.set_variable("player_name".to_string(), Value::String("Alice".to_string()));
    assert_eq!(
        world.get_variable("player_name"), 
        Some(&Value::String("Alice".to_string()))
    );
    
    // Test flags
    assert!(!world.get_flag("puzzle_solved"));
    world.set_flag("puzzle_solved".to_string(), true);
    assert!(world.get_flag("puzzle_solved"));
    
    world.set_flag("puzzle_solved".to_string(), false);
    assert!(!world.get_flag("puzzle_solved"));
}

#[test]
fn test_turn_counter() {
    let mut world = World::new();
    assert_eq!(world.turn_count, 0);
    
    // Turn count is incremented by movement and item actions
    let room1 = Room {
        id: "room1".to_string(),
        title: Some("Room 1".to_string()),
        description: Some("First room".to_string()),
        exits: HashMap::from([(Direction::East, "room2".to_string())]),
        items: vec![],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    let room2 = Room {
        id: "room2".to_string(),
        title: Some("Room 2".to_string()),
        description: Some("Second room".to_string()),
        exits: HashMap::from([(Direction::West, "room1".to_string())]),
        items: vec![],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("room1".to_string(), room1).unwrap();
    world.add_room("room2".to_string(), room2).unwrap();
    world.player_location = "room1".to_string();
    
    // Move to increment turn
    world.move_player(Direction::East).unwrap();
    assert_eq!(world.turn_count, 1);
    
    world.move_player(Direction::West).unwrap();
    world.move_player(Direction::East).unwrap();
    assert_eq!(world.turn_count, 3);
}

// COMPREHENSIVE WORLD MODEL TESTS

#[test]
fn test_complex_room_network() {
    let mut world = World::new();
    
    // Create a complex network of interconnected rooms
    let rooms = [
        ("entrance", "Entrance Hall", vec![(Direction::North, "hallway"), (Direction::Up, "balcony")]),
        ("hallway", "Long Hallway", vec![(Direction::South, "entrance"), (Direction::East, "library"), (Direction::West, "kitchen")]),
        ("library", "Ancient Library", vec![(Direction::West, "hallway"), (Direction::North, "study")]),
        ("kitchen", "Castle Kitchen", vec![(Direction::East, "hallway"), (Direction::North, "pantry")]),
        ("study", "Private Study", vec![(Direction::South, "library")]),
        ("pantry", "Food Pantry", vec![(Direction::South, "kitchen")]),
        ("balcony", "Upper Balcony", vec![(Direction::Down, "entrance")]),
    ];
    
    // Add all rooms
    for (id, name, exits_data) in rooms {
        let mut exits = HashMap::new();
        for (direction, target) in exits_data {
            exits.insert(direction, target.to_string());
        }
        
        let room = Room {
            id: id.to_string(),
            title: Some(name.to_string()),
            description: Some(format!("You are in the {}.", name)),
            exits,
            items: vec![],
            characters: vec![],
            dark: false,
            visited: false,
            on_enter: vec![],
            on_exit: vec![],
            properties: HashMap::new(),
        };
        
        world.add_room(id.to_string(), room).expect("Failed to add room");
    }
    
    world.player_location = "entrance".to_string();
    
    // Test complex navigation
    assert_eq!(world.move_player(Direction::North).unwrap(), "hallway");
    assert_eq!(world.move_player(Direction::East).unwrap(), "library");
    assert_eq!(world.move_player(Direction::North).unwrap(), "study");
    assert_eq!(world.move_player(Direction::South).unwrap(), "library");
    assert_eq!(world.move_player(Direction::West).unwrap(), "hallway");
    assert_eq!(world.move_player(Direction::West).unwrap(), "kitchen");
    assert_eq!(world.move_player(Direction::North).unwrap(), "pantry");
    
    // Test that we can get back to entrance via multiple paths
    world.move_player(Direction::South).unwrap(); // kitchen
    world.move_player(Direction::East).unwrap(); // hallway
    world.move_player(Direction::South).unwrap(); // entrance
    assert_eq!(world.player_location, "entrance");
    
    // Test vertical movement
    assert_eq!(world.move_player(Direction::Up).unwrap(), "balcony");
    assert_eq!(world.move_player(Direction::Down).unwrap(), "entrance");
}

#[test]
fn test_item_weight_and_physics() {
    let mut world = World::new();
    
    // Create items with different weights
    let items = [
        ("feather", "Light Feather", 0.1, true),
        ("book", "Heavy Book", 2.0, true),
        ("sword", "Steel Sword", 5.0, true),
        ("chest", "Treasure Chest", 50.0, false), // Too heavy to take
        ("key", "Golden Key", 0.2, true),
    ];
    
    for (id, name, weight, takeable) in items {
        let item = Item {
            id: id.to_string(),
            name: Some(name.to_string()),
            description: Some(format!("A {}.", name.to_lowercase())),
            location: Some("room1".to_string()),
            takeable,
            weight,
            is_container: false,
            contains: vec![],
            on_take: vec![],
            on_use: vec![],
            on_examine: vec![],
            properties: HashMap::new(),
        };
        
        world.add_item(id.to_string(), item).expect("Failed to add item");
    }
    
    // Create room
    let room = Room {
        id: "room1".to_string(),
        title: Some("Test Room".to_string()),
        description: Some("A room with various items.".to_string()),
        exits: HashMap::new(),
        items: vec!["feather".to_string(), "book".to_string(), "sword".to_string(), "chest".to_string(), "key".to_string()],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("room1".to_string(), room).expect("Failed to add room");
    world.player_location = "room1".to_string();
    
    // Test taking items of different weights
    world.take_item("feather").expect("Should be able to take feather");
    world.take_item("key").expect("Should be able to take key");
    world.take_item("book").expect("Should be able to take book");
    world.take_item("sword").expect("Should be able to take sword");
    
    // Test that heavy non-takeable item can't be taken
    let result = world.take_item("chest");
    assert!(result.is_err(), "Should not be able to take chest");
    
    // Verify inventory contents
    assert_eq!(world.inventory.len(), 4);
    assert!(world.inventory.contains(&"feather".to_string()));
    assert!(world.inventory.contains(&"key".to_string()));
    assert!(world.inventory.contains(&"book".to_string()));
    assert!(world.inventory.contains(&"sword".to_string()));
    assert!(!world.inventory.contains(&"chest".to_string()));
    
    // Test dropping items
    world.drop_item("sword").expect("Should be able to drop sword");
    assert_eq!(world.inventory.len(), 3);
    assert!(!world.inventory.contains(&"sword".to_string()));
    assert_eq!(world.items.get("sword").unwrap().location, Some("room1".to_string()));
}

#[test]
fn test_nested_containers() {
    let mut world = World::new();
    
    // Create a nested container structure: backpack -> pouch -> coin
    let backpack = Item {
        id: "backpack".to_string(),
        name: Some("leather backpack".to_string()),
        description: Some("A sturdy leather backpack.".to_string()),
        location: Some("room1".to_string()),
        takeable: true,
        weight: 1.0,
        is_container: true,
        contains: vec!["pouch".to_string()],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([("open".to_string(), Value::Bool(true))]),
    };
    
    let pouch = Item {
        id: "pouch".to_string(),
        name: Some("coin pouch".to_string()),
        description: Some("A small coin pouch.".to_string()),
        location: Some("backpack".to_string()),
        takeable: true,
        weight: 0.5,
        is_container: true,
        contains: vec!["coin".to_string()],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([("open".to_string(), Value::Bool(true))]),
    };
    
    let coin = Item {
        id: "coin".to_string(),
        name: Some("gold coin".to_string()),
        description: Some("A shiny gold coin.".to_string()),
        location: Some("pouch".to_string()),
        takeable: true,
        weight: 0.1,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::new(),
    };
    
    world.add_item("backpack".to_string(), backpack).expect("Failed to add backpack");
    world.add_item("pouch".to_string(), pouch).expect("Failed to add pouch");
    world.add_item("coin".to_string(), coin).expect("Failed to add coin");
    
    // Verify containment hierarchy
    assert_eq!(world.items.get("pouch").unwrap().location, Some("backpack".to_string()));
    assert_eq!(world.items.get("coin").unwrap().location, Some("pouch".to_string()));
    assert!(world.items.get("backpack").unwrap().contains.contains(&"pouch".to_string()));
    assert!(world.items.get("pouch").unwrap().contains.contains(&"coin".to_string()));
}

#[test]
fn test_character_management() {
    let mut world = World::new();
    
    // Create characters with different properties
    let guard = Character {
        id: "guard".to_string(),
        name: Some("Palace Guard".to_string()),
        description: Some("A stern-looking guard in uniform.".to_string()),
        location: Some("entrance".to_string()),
        dialogue: DialogueTree::default(),
        state: "alert".to_string(),
        relationship: 0,
        properties: HashMap::from([
            ("health".to_string(), Value::Integer(100)),
            ("weapon".to_string(), Value::String("sword".to_string())),
            ("hostile".to_string(), Value::Bool(false)),
        ]),
    };
    
    let merchant = Character {
        id: "merchant".to_string(),
        name: Some("Traveling Merchant".to_string()),
        description: Some("A friendly merchant with wares to sell.".to_string()),
        location: Some("marketplace".to_string()),
        dialogue: DialogueTree::default(),
        state: "friendly".to_string(),
        relationship: 5,
        properties: HashMap::from([
            ("gold".to_string(), Value::Integer(1000)),
            ("inventory".to_string(), Value::List(vec![
                Value::String("potion".to_string()),
                Value::String("scroll".to_string()),
            ])),
        ]),
    };
    
    world.add_character("guard".to_string(), guard).expect("Failed to add guard");
    world.add_character("merchant".to_string(), merchant).expect("Failed to add merchant");
    
    // Test character properties
    let guard_ref = world.characters.get("guard").unwrap();
    assert_eq!(guard_ref.state, "alert");
    assert_eq!(guard_ref.relationship, 0);
    assert_eq!(guard_ref.properties.get("health"), Some(&Value::Integer(100)));
    
    let merchant_ref = world.characters.get("merchant").unwrap();
    assert_eq!(merchant_ref.state, "friendly");
    assert_eq!(merchant_ref.relationship, 5);
    assert_eq!(merchant_ref.properties.get("gold"), Some(&Value::Integer(1000)));
    
    // Test character locations
    assert_eq!(guard_ref.location, Some("entrance".to_string()));
    assert_eq!(merchant_ref.location, Some("marketplace".to_string()));
}

#[test]
fn test_world_state_persistence() {
    let mut world = World::new();
    
    // Set up initial state
    world.set_variable("chapter".to_string(), Value::Integer(1));
    world.set_variable("player_name".to_string(), Value::String("Hero".to_string()));
    world.set_variable("health".to_string(), Value::Integer(100));
    world.set_flag("tutorial_complete".to_string(), true);
    world.set_flag("has_sword".to_string(), false);
    world.score = 50;
    world.turn_count = 10;
    
    // Test state queries
    assert_eq!(world.get_variable("chapter"), Some(&Value::Integer(1)));
    assert_eq!(world.get_variable("player_name"), Some(&Value::String("Hero".to_string())));
    assert_eq!(world.get_variable("health"), Some(&Value::Integer(100)));
    assert!(world.get_flag("tutorial_complete"));
    assert!(!world.get_flag("has_sword"));
    assert_eq!(world.score, 50);
    assert_eq!(world.turn_count, 10);
    
    // Test state updates
    world.set_variable("health".to_string(), Value::Integer(75));
    world.set_flag("has_sword".to_string(), true);
    world.score += 25;
    world.turn_count += 1;
    
    // Verify updates
    assert_eq!(world.get_variable("health"), Some(&Value::Integer(75)));
    assert!(world.get_flag("has_sword"));
    assert_eq!(world.score, 75);
    assert_eq!(world.turn_count, 11);
}

#[test]
fn test_room_properties_and_states() {
    let mut world = World::new();
    
    // Create room with various properties
    let room = Room {
        id: "complex_room".to_string(),
        title: Some("Complex Room".to_string()),
        description: Some("A room with many properties.".to_string()),
        exits: HashMap::from([(Direction::North, "next_room".to_string())]),
        items: vec!["lamp".to_string()],
        characters: vec!["npc".to_string()],
        dark: true,
        visited: false,
        on_enter: vec![Statement::Print(Expression::Literal(Value::String("Welcome!".to_string())))],
        on_exit: vec![Statement::Print(Expression::Literal(Value::String("Goodbye!".to_string())))],
        properties: HashMap::from([
            ("temperature".to_string(), Value::Integer(20)),
            ("atmosphere".to_string(), Value::String("spooky".to_string())),
            ("locked".to_string(), Value::Bool(false)),
            ("treasure_found".to_string(), Value::Bool(false)),
        ]),
    };
    
    world.add_room("complex_room".to_string(), room).expect("Failed to add room");
    
    // Test room properties
    let room_ref = world.rooms.get("complex_room").unwrap();
    assert!(room_ref.dark);
    assert!(!room_ref.visited);
    assert_eq!(room_ref.items.len(), 1);
    assert_eq!(room_ref.characters.len(), 1);
    assert_eq!(room_ref.on_enter.len(), 1);
    assert_eq!(room_ref.on_exit.len(), 1);
    
    // Test property access
    assert_eq!(room_ref.properties.get("temperature"), Some(&Value::Integer(20)));
    assert_eq!(room_ref.properties.get("atmosphere"), Some(&Value::String("spooky".to_string())));
    assert_eq!(room_ref.properties.get("locked"), Some(&Value::Bool(false)));
    
    // Test room state changes
    let room_mut = world.rooms.get_mut("complex_room").unwrap();
    room_mut.visited = true;
    room_mut.dark = false;
    room_mut.properties.insert("treasure_found".to_string(), Value::Bool(true));
    
    // Verify changes
    let room_ref = world.rooms.get("complex_room").unwrap();
    assert!(room_ref.visited);
    assert!(!room_ref.dark);
    assert_eq!(room_ref.properties.get("treasure_found"), Some(&Value::Bool(true)));
}

#[test]
fn test_world_query_comprehensive() {
    let mut world = World::new();
    
    // Create a complex world with items, characters, and rooms
    setup_complex_world(&mut world);
    
    world.player_location = "main_room".to_string();
    
    // Test finding items in current room
    {
        let query = WorldQuery::new(&world);
        let result = query.find_item_here("sword");
        assert!(result.is_some());
        let (id, item) = result.unwrap();
        assert_eq!(id, "magic_sword");
        assert_eq!(item.name, Some("Magic Sword".to_string()));
    }
    
    // Test finding items in inventory (after taking them)
    world.take_item("magic_sword").expect("Failed to take sword");
    {
        let query = WorldQuery::new(&world);
        let result = query.find_item_inventory("sword");
        assert!(result.is_some());
    }
    
    // Test finding characters
    let query = WorldQuery::new(&world);
    let result = query.find_character_here("wizard");
    assert!(result.is_some());
    let (id, character) = result.unwrap();
    assert_eq!(id, "old_wizard");
    assert_eq!(character.name, Some("Old Wizard".to_string()));
    
    // Test ambiguous queries
    let result = query.find_item_here("magic");
    assert!(result.is_some()); // Should find one of the magic items
    
    // Test case-insensitive queries
    let result = query.find_item_here("SWORD");
    assert!(result.is_some());
    
    let result = query.find_character_here("WIZARD");
    assert!(result.is_some());
}

#[test]
fn test_world_events_and_triggers() {
    let mut world = World::new();
    
    // Create room with event triggers
    let room = Room {
        id: "event_room".to_string(),
        title: Some("Event Room".to_string()),
        description: Some("A room that triggers events.".to_string()),
        exits: HashMap::new(),
        items: vec![],
        characters: vec![],
        dark: false,
        visited: false,
        on_enter: vec![
            Statement::Assignment { variable: "entered_event_room".to_string(), value: Expression::Literal(Value::Bool(true)) },
            Statement::Command(plotscript::parser::Command::SetVariable { name: "score".to_string(), value: Value::Integer(10) }),
        ],
        on_exit: vec![
            Statement::Assignment { variable: "left_event_room".to_string(), value: Expression::Literal(Value::Bool(true)) },
        ],
        properties: HashMap::new(),
    };
    
    world.add_room("event_room".to_string(), room).expect("Failed to add room");
    
    // Create items with event triggers
    let magic_item = Item {
        id: "magic_orb".to_string(),
        name: Some("Glowing Orb".to_string()),
        description: Some("An orb that pulses with magical energy.".to_string()),
        location: Some("event_room".to_string()),
        takeable: true,
        weight: 0.5,
        is_container: false,
        contains: vec![],
        on_take: vec![
            Statement::Print(Expression::Literal(Value::String("The orb feels warm to the touch.".to_string()))),
            Statement::Assignment { variable: "has_magic_orb".to_string(), value: Expression::Literal(Value::Bool(true)) },
        ],
        on_use: vec![
            Statement::Print(Expression::Literal(Value::String("The orb glows brighter!".to_string()))),
            Statement::Command(plotscript::parser::Command::SetVariable { name: "score".to_string(), value: Value::Integer(50) }),
        ],
        on_examine: vec![
            Statement::Print(Expression::Literal(Value::String("You see ancient runes carved into the orb's surface.".to_string()))),
        ],
        properties: HashMap::from([
            ("magic_power".to_string(), Value::Integer(100)),
            ("cursed".to_string(), Value::Bool(false)),
        ]),
    };
    
    world.add_item("magic_orb".to_string(), magic_item).expect("Failed to add magic orb");
    
    // Verify event triggers are set up correctly
    let room_ref = world.rooms.get("event_room").unwrap();
    assert_eq!(room_ref.on_enter.len(), 2);
    assert_eq!(room_ref.on_exit.len(), 1);
    
    let item_ref = world.items.get("magic_orb").unwrap();
    assert_eq!(item_ref.on_take.len(), 2);
    assert_eq!(item_ref.on_use.len(), 2);
    assert_eq!(item_ref.on_examine.len(), 1);
}

#[test]
fn test_world_performance_with_large_datasets() {
    let mut world = World::new();
    
    let start = std::time::Instant::now();
    
    // Create a large number of rooms
    for i in 0..100 {
        let room = Room {
            id: format!("room_{}", i),
            title: Some(format!("Room {}", i)),
            description: Some(format!("This is room number {}.", i)),
            exits: if i > 0 {
                HashMap::from([(Direction::South, format!("room_{}", i - 1))])
            } else {
                HashMap::new()
            },
            items: vec![format!("item_{}", i)],
            characters: if i % 10 == 0 { vec![format!("npc_{}", i / 10)] } else { vec![] },
            dark: i % 3 == 0,
            visited: false,
            on_enter: vec![],
            on_exit: vec![],
            properties: HashMap::from([
                ("number".to_string(), Value::Integer(i as i64)),
                ("even".to_string(), Value::Bool(i % 2 == 0)),
            ]),
        };
        
        world.add_room(format!("room_{}", i), room).expect("Failed to add room");
    }
    
    // Create items for each room
    for i in 0..100 {
        let item = Item {
            id: format!("item_{}", i),
            name: Some(format!("Item {}", i)),
            description: Some(format!("This is item number {}.", i)),
            location: Some(format!("room_{}", i)),
            takeable: true,
            weight: 1.0 + (i as f32 * 0.1),
            is_container: i % 20 == 0,
            contains: vec![],
            on_take: vec![],
            on_use: vec![],
            on_examine: vec![],
            properties: HashMap::from([
                ("value".to_string(), Value::Integer(i as i64 * 10)),
                ("rare".to_string(), Value::Bool(i % 25 == 0)),
            ]),
        };
        
        world.add_item(format!("item_{}", i), item).expect("Failed to add item");
    }
    
    // Create NPCs
    for i in 0..10 {
        let character = Character {
            id: format!("npc_{}", i),
            name: Some(format!("NPC {}", i)),
            description: Some(format!("This is NPC number {}.", i)),
            location: Some(format!("room_{}", i * 10)),
            dialogue: DialogueTree::default(),
            state: "neutral".to_string(),
            relationship: i as i32,
            properties: HashMap::from([
                ("level".to_string(), Value::Integer(i as i64 + 1)),
                ("friendly".to_string(), Value::Bool(i % 2 == 0)),
            ]),
        };
        
        world.add_character(format!("npc_{}", i), character).expect("Failed to add character");
    }
    
    let setup_time = start.elapsed();
    println!("Large world setup time: {:?}", setup_time);
    
    // Test queries with large dataset
    let query_start = std::time::Instant::now();
    
    world.player_location = "room_50".to_string();
    let query = WorldQuery::new(&world);
    
    // Perform various queries
    for i in 0..100 {
        let item_name = format!("Item {}", i);
        if i == 50 {
            // This should find item in current room
            let result = query.find_item_here(&item_name);
            assert!(result.is_some());
        }
    }
    
    let query_time = query_start.elapsed();
    println!("Query performance time: {:?}", query_time);
    
    // Performance should be reasonable
    assert!(setup_time.as_millis() < 1000, "World setup should be fast");
    assert!(query_time.as_millis() < 100, "Queries should be fast");
    
    // Verify world state
    assert_eq!(world.rooms.len(), 100);
    assert_eq!(world.items.len(), 100);
    assert_eq!(world.characters.len(), 10);
}

// Helper function to set up a complex world for testing
fn setup_complex_world(world: &mut World) {
    // Create main room with items and characters
    let room = Room {
        id: "main_room".to_string(),
        title: Some("Main Hall".to_string()),
        description: Some("A grand hall with various items and people.".to_string()),
        exits: HashMap::from([(Direction::North, "north_room".to_string())]),
        items: vec!["magic_sword".to_string(), "healing_potion".to_string(), "ancient_book".to_string()],
        characters: vec!["old_wizard".to_string(), "young_apprentice".to_string()],
        dark: false,
        visited: false,
        on_enter: vec![],
        on_exit: vec![],
        properties: HashMap::new(),
    };
    
    world.add_room("main_room".to_string(), room).expect("Failed to add main room");
    
    // Add items
    let sword = Item {
        id: "magic_sword".to_string(),
        name: Some("Magic Sword".to_string()),
        description: Some("A gleaming sword imbued with magical power.".to_string()),
        location: Some("main_room".to_string()),
        takeable: true,
        weight: 3.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([("damage".to_string(), Value::Integer(25))]),
    };
    
    let potion = Item {
        id: "healing_potion".to_string(),
        name: Some("Healing Potion".to_string()),
        description: Some("A red potion that glows with healing energy.".to_string()),
        location: Some("main_room".to_string()),
        takeable: true,
        weight: 0.5,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([("healing".to_string(), Value::Integer(50))]),
    };
    
    let book = Item {
        id: "ancient_book".to_string(),
        name: Some("Ancient Tome".to_string()),
        description: Some("A leather-bound book filled with arcane knowledge.".to_string()),
        location: Some("main_room".to_string()),
        takeable: true,
        weight: 2.0,
        is_container: false,
        contains: vec![],
        on_take: vec![],
        on_use: vec![],
        on_examine: vec![],
        properties: HashMap::from([("knowledge".to_string(), Value::Integer(10))]),
    };
    
    world.add_item("magic_sword".to_string(), sword).expect("Failed to add sword");
    world.add_item("healing_potion".to_string(), potion).expect("Failed to add potion");
    world.add_item("ancient_book".to_string(), book).expect("Failed to add book");
    
    // Add characters
    let wizard = Character {
        id: "old_wizard".to_string(),
        name: Some("Old Wizard".to_string()),
        description: Some("An elderly wizard with a long white beard.".to_string()),
        location: Some("main_room".to_string()),
        dialogue: DialogueTree::default(),
        state: "wise".to_string(),
        relationship: 10,
        properties: HashMap::from([
            ("magic_level".to_string(), Value::Integer(100)),
            ("helpful".to_string(), Value::Bool(true)),
        ]),
    };
    
    let apprentice = Character {
        id: "young_apprentice".to_string(),
        name: Some("Young Apprentice".to_string()),
        description: Some("A young person learning the magical arts.".to_string()),
        location: Some("main_room".to_string()),
        dialogue: DialogueTree::default(),
        state: "eager".to_string(),
        relationship: 5,
        properties: HashMap::from([
            ("magic_level".to_string(), Value::Integer(20)),
            ("enthusiastic".to_string(), Value::Bool(true)),
        ]),
    };
    
    world.add_character("old_wizard".to_string(), wizard).expect("Failed to add wizard");
    world.add_character("young_apprentice".to_string(), apprentice).expect("Failed to add apprentice");
}