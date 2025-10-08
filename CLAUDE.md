# CLAUDE.md - engine.plotscri.pt

## Repository Overview

The PlotScript Engine is an open-source runtime for interactive narratives, written in Rust and compilable to WebAssembly. It provides equal support for three core game formats: text adventures (parser-based), visual novels (multimedia narratives), and interactive fiction (choice-based stories).

### Purpose
- Universal runtime for all interactive narrative formats
- Equal support for parser, visual, and choice-based games
- WebAssembly compilation for browser deployment
- Extensible architecture for custom game types

### Supported Formats
- **Text Adventures**: Parser-based games with natural language input
- **Visual Novels**: Story-driven games with images and character sprites  
- **Interactive Fiction**: Choice-based narratives and simulations

### License
Dual-licensed under MIT and Apache 2.0

## Tech Stack

- **Language**: Rust (2021 edition)
- **Parser**: Pest (PEG grammar)
- **Serialization**: Serde with RON/JSON support
- **WebAssembly**: wasm-bindgen + wasm-pack
- **Testing**: Built-in test framework + proptest
- **Benchmarking**: Criterion

## Repository Structure

```
engine.plotscri.pt/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── parser/             # Natural language parser
│   │   ├── mod.rs
│   │   ├── grammar.pest    # PEG grammar definition
│   │   ├── lexer.rs        # Tokenization
│   │   ├── ast.rs          # Abstract syntax tree
│   │   └── fuzzy.rs        # Fuzzy matching
│   ├── runtime/            # Game runtime
│   │   ├── mod.rs
│   │   ├── state.rs        # Game state management
│   │   ├── world.rs        # World model
│   │   ├── inventory.rs    # Item management
│   │   └── events.rs       # Event system
│   ├── scripting/          # Scripting engine
│   │   ├── mod.rs
│   │   ├── conditions.rs   # Condition evaluation
│   │   ├── actions.rs      # Action execution
│   │   └── storylets.rs    # Quality-based narratives
│   ├── io/                 # Input/output
│   │   ├── mod.rs
│   │   ├── save.rs         # Save/load system
│   │   ├── config.rs       # Configuration loading
│   │   └── formats.rs      # RON/JSON/YAML support
│   └── wasm/              # WebAssembly bindings
│       ├── mod.rs
│       └── bindings.rs
├── cli/                   # Command-line interface
│   ├── src/
│   │   └── main.rs       # CLI entry point
│   └── Cargo.toml
├── examples/             # Example games
│   ├── tutorial/         # Basic tutorial game
│   ├── mystery/          # Parser game example
│   └── visual_novel/     # VN example
├── tests/                # Integration tests
├── benches/              # Performance benchmarks
├── grammar/              # Grammar documentation
├── Cargo.toml           # Package manifest
└── README.md            # Public documentation
```

## Core Components

### Game Format Support

#### Text Adventure Components
```rust
pub mod text_adventure {
    pub struct Parser {
        grammar: PestGrammar,
        vocabulary: Vocabulary,
        fuzzy_matcher: FuzzyMatcher,
    }
    
    pub struct Room {
        id: RoomId,
        name: String,
        description: String,
        exits: HashMap<Direction, RoomId>,
        items: Vec<ItemId>,
        dark: bool,
    }
    
    pub struct Inventory {
        items: Vec<Item>,
        capacity: usize,
        weight_limit: Option<u32>,
    }
}
```

#### Visual Novel Components
```rust
pub mod visual_novel {
    pub struct Scene {
        id: SceneId,
        background: Option<AssetPath>,
        characters: Vec<Character>,
        dialogue: DialogueTree,
        music: Option<AssetPath>,
    }
    
    pub struct Character {
        id: CharacterId,
        name: String,
        sprites: HashMap<Expression, AssetPath>,
        position: Position,
    }
    
    pub struct DialogueNode {
        speaker: Option<CharacterId>,
        text: String,
        choices: Vec<Choice>,
    }
}
```

#### Interactive Fiction Components
```rust
pub mod interactive_fiction {
    pub struct StoryNode {
        id: NodeId,
        content: String,
        choices: Vec<Choice>,
        conditions: Vec<Condition>,
        consequences: Vec<Action>,
    }
    
    pub struct Quality {
        name: String,
        value: i32,
        min: Option<i32>,
        max: Option<i32>,
    }
    
    pub struct Storylet {
        id: StoryletId,
        conditions: Vec<Condition>,
        content: StoryNode,
        priority: i32,
    }
}
```

### Universal Systems

All game formats share these core systems:

```rust
pub mod core {
    pub struct GameState {
        variables: HashMap<String, Value>,
        flags: HashSet<String>,
        history: Vec<HistoryEntry>,
    }
    
    pub struct SaveSystem {
        compression: CompressionType,
        versioning: bool,
        cloud_sync: bool,
    }
    
    pub trait GameFormat {
        fn process_input(&mut self, input: &str) -> Response;
        fn render(&self) -> Output;
        fn save(&self) -> SaveData;
        fn load(&mut self, data: SaveData) -> Result<()>;
    }
}
```

### Configuration Format (RON)

The engine uses a unified configuration format that adapts to each game type:

#### Text Adventure Configuration
```rust
Game(
    format: TextAdventure,
    title: "Mystery Manor",
    author: "Jane Doe",
    starting_location: "entrance",
    
    settings: TextAdventureSettings(
        parser_mode: Natural,
        command_aliases: true,
        darkness_system: true,
    ),
    
    locations: {
        "entrance": Location(
            name: "Manor Entrance",
            description: "A grand entrance with marble floors.",
            exits: {
                North: "hallway",
                Up: "stairs",
            },
            items: ["brass_key", "welcome_mat"],
        ),
    },
    
    vocabulary: Vocabulary(
        verbs: ["take", "drop", "examine", "go", "use"],
        synonyms: {
            "get": "take",
            "look": "examine",
        },
    ),
)
```

#### Visual Novel Configuration
```rust
Game(
    format: VisualNovel,
    title: "Summer Romance",
    author: "Studio Name",
    starting_scene: "opening",
    
    settings: VisualNovelSettings(
        resolution: (1920, 1080),
        text_speed: Variable,
        auto_save: true,
    ),
    
    scenes: {
        "opening": Scene(
            background: "school_entrance.png",
            music: "gentle_morning.ogg",
            characters: [
                Character(
                    id: "maya",
                    sprite: "maya_happy.png",
                    position: Left,
                ),
            ],
            dialogue: [
                Line(speaker: "maya", text: "Good morning!"),
            ],
        ),
    },
)
```

#### Interactive Fiction Configuration
```rust
Game(
    format: InteractiveFiction,
    title: "Corporate Spy",
    author: "IF Studios",
    starting_node: "mission_brief",
    
    settings: InteractiveFictionSettings(
        show_stats: true,
        checkpoint_saves: true,
        timed_choices: false,
    ),
    
    nodes: {
        "mission_brief": StoryNode(
            content: "Your handler slides a folder across the table.",
            choices: [
                Choice(
                    text: "Open the folder",
                    target: "read_mission",
                    conditions: [],
                ),
                Choice(
                    text: "Push it back",
                    target: "refuse_mission",
                    conditions: [QualityAtLeast("courage", 5)],
                ),
            ],
        ),
    },
    
    qualities: {
        "stealth": Quality(initial: 3, min: 0, max: 10),
        "courage": Quality(initial: 5, min: 0, max: 10),
    },
)
```

## API Reference

### Rust API

The engine provides format-specific APIs while maintaining a common interface:

```rust
use plotscript_engine::{Game, GameFormat, Config};

// Create any game type from configuration
let game = Game::from_config_file("game.ron")?;

// Or create specific format
let text_adventure = TextAdventure::new(config)?;
let visual_novel = VisualNovel::new(config)?;
let interactive_fiction = InteractiveFiction::new(config)?;

// Common interface for all formats
match game.format() {
    GameFormat::TextAdventure => {
        let response = game.process_command("go north")?;
    },
    GameFormat::VisualNovel => {
        let response = game.advance_dialogue()?;
    },
    GameFormat::InteractiveFiction => {
        let response = game.select_choice(0)?;
    },
}

// Universal features
let save_data = game.save()?;
game.load(&save_data)?;
```

### WebAssembly API

JavaScript API supports all game formats:

```javascript
import init, { Game, GameFormat } from './plotscript_engine.js';

await init();

// Load any game type
const game = Game.fromConfig(configJson);

// Format-specific methods
switch (game.format) {
    case GameFormat.TextAdventure:
        const response = game.processCommand("take key");
        break;
    case GameFormat.VisualNovel:
        const response = game.nextDialogue();
        break;
    case GameFormat.InteractiveFiction:
        const response = game.makeChoice(0);
        break;
}

// Universal methods work for all formats
const saveData = game.save();
game.load(saveData);
```

### CLI Usage

The CLI adapts to the game format:

```bash
# Run any game type (auto-detects format)
plotscript play game.ron

# Create new game with specific format
plotscript new my-adventure --format text-adventure
plotscript new my-vn --format visual-novel  
plotscript new my-story --format interactive-fiction

# Convert between formats (where possible)
plotscript convert story.ron --to visual-novel

# Format-specific validation
plotscript validate game.ron --strict
```

## Development Guidelines

### Building
```bash
# Build library
cargo build --release

# Build WebAssembly
wasm-pack build --target web --out-dir pkg

# Build CLI
cargo build --release --bin plotscript

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Testing
- Unit tests for each module
- Integration tests for game scenarios
- Property-based tests for parser
- Benchmarks for performance-critical code

### Performance Requirements
- Parser response: <50ms for typical commands
- Save/load: <100ms for average game
- Memory usage: <10MB for typical game
- WASM size: <500KB compressed

### Code Style
- Follow Rust standard style
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Document all public APIs

## Extension System

### Custom Commands
```rust
use plotscript_engine::{Command, Extension, Response};

pub struct MagicExtension;

impl Extension for MagicExtension {
    fn name(&self) -> &str {
        "magic"
    }
    
    fn handle_command(&mut self, cmd: &Command, game: &mut Game) -> Option<Response> {
        match cmd.verb.as_str() {
            "cast" => Some(self.cast_spell(cmd, game)),
            _ => None,
        }
    }
}
```

### Custom Conditions/Actions
```rust
// Register custom condition
game.register_condition("is_raining", |game| {
    game.get_property("weather") == Some(&Value::String("rain".into()))
});

// Register custom action
game.register_action("thunder", |game| {
    game.print("Thunder rumbles overhead!");
    game.set_property("scared_npc", Value::Bool(true));
});
```

## WebAssembly Optimization

### Size Optimization
- Use `wee_alloc` instead of default allocator
- Enable LTO in release builds
- Strip debug symbols
- Use `wasm-opt` for additional optimization

### Build Configuration
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
strip = true        # Strip symbols

[dependencies.web-sys]
version = "0.3"
features = [        # Only required features
    "console",
    "Document", 
    "Element",
    "HtmlElement",
    "Window",
]
```

## Versioning

Follows semantic versioning:
- MAJOR: Breaking API changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes

Check compatibility:
```rust
plotscript_engine::VERSION  // "1.2.3"
plotscript_engine::check_version("^1.0.0")  // true
```

## Contributing

### Process
1. Fork the repository
2. Create feature branch
3. Write tests
4. Implement feature
5. Run tests and benchmarks
6. Submit PR

### Areas Needing Help
- Parser improvements
- Language translations
- Performance optimization
- Documentation
- Example games

### Code of Conduct
- Be respectful
- Welcome newcomers
- Focus on constructive feedback
- No harassment or discrimination

## Benchmarks

Current performance metrics:
- Simple command parse: 5-10ms
- Complex command parse: 20-40ms
- Save game (1000 objects): 50ms
- Load game (1000 objects): 30ms
- WASM size: 450KB compressed

## License

Dual-licensed under:
- MIT License
- Apache License 2.0

Choose whichever license works better for your use case.

## Support

- GitHub Issues: Bug reports
- Discussions: Questions and ideas
- Discord: Community chat
- Email: security@plotscri.pt (security only)

Remember: The engine is the foundation of PlotScript. It must be fast, reliable, and extensible while maintaining a clean API for both Rust and WebAssembly users.