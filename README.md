# PlotScript Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)](https://webassembly.org/)

The open-source runtime engine for interactive narratives. PlotScript Engine powers text adventures, visual novels, and interactive fiction with a unified, extensible architecture.

## Features

- 🎮 **Three Game Formats**: Equal support for text adventures, visual novels, and interactive fiction
- 🌐 **WebAssembly Ready**: Compile to WASM for browser deployment (<500KB)
- 🔧 **Extensible**: Plugin system for custom commands and behaviors
- 📝 **Multiple Script Formats**: Support for RON, YAML, and custom DSL
- 🎯 **Fuzzy Matching**: Typo-tolerant parser for natural language input
- 💾 **Save System**: Automatic and manual save support with compression
- 🎨 **Rich Content**: Support for images, audio, and visual effects
- 🌍 **i18n Ready**: Built-in internationalization support

## Quick Start

### Installation

Add PlotScript to your `Cargo.toml`:

```toml
[dependencies]
plotscript = "0.1.0"
```

### Basic Usage

```rust
use plotscript::{Engine, EngineConfig, GameMode, init};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the engine (required for WASM)
    init();
    
    // Create a new engine
    let mut engine = Engine::new();
    
    // Or create with custom config
    let config = EngineConfig {
        mode: GameMode::TextAdventure,
        typo_correction: true,
        typo_threshold: 70,
        ..Default::default()
    };
    let mut engine = Engine::with_config(config);
    
    // Load a game script (supports RON, YAML, or DSL)
    engine.load_script(include_str!("my_game.ron"))?;
    
    // Start the game
    let response = engine.start()?;
    println!("{}", response.text);
    
    // Process player input
    let response = engine.process_input("go north")?;
    println!("{}", response.text);
    
    // Save the game
    engine.save_game(Some(1))?;
    
    Ok(())
}
```

### WebAssembly Usage

```javascript
import init, { Engine, GameMode } from './pkg/plotscript.js';

async function runGame() {
    // Initialize WASM module
    await init();
    
    // Create engine
    const engine = Engine.new();
    
    // Load game script
    engine.load_script(gameScript);
    
    // Start game
    const response = engine.start();
    document.getElementById('output').textContent = response.text;
    
    // Handle player input
    document.getElementById('input').addEventListener('submit', (e) => {
        e.preventDefault();
        const input = e.target.command.value;
        const response = engine.process_input(input);
        document.getElementById('output').textContent = response.text;
        e.target.command.value = '';
    });
}

runGame();
```

## Game Script Formats

PlotScript supports multiple script formats. The recommended format is RON (Rusty Object Notation).

### Text Adventure (RON)

```rust
TextAdventure((
    title: "Mystery Manor",
    author: "Jane Doe",
    description: Some("A thrilling mystery adventure"),
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
            description: "A grand entrance with marble floors.",
            exits: {
                north: "hallway",
                up: "stairs",
            },
            items: ["brass_key", "welcome_mat"],
            characters: ["butler"],
            dark: Some(false),
            first_visit: Some("You've never seen such an imposing entrance."),
            events: None,
        ),
    },
    
    items: {
        "brass_key": (
            name: "Brass Key",
            description: "A small, worn brass key.",
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
            description: "An elderly butler with impeccable posture.",
            dialogue: Some((
                start: "greeting",
                nodes: {
                    "greeting": (
                        text: "Good evening. How may I assist you?",
                        responses: Some([
                            (text: "What happened here?", next: "mystery"),
                            (text: "Where is everyone?", next: "missing"),
                        ]),
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
```

### Visual Novel (RON)

```rust
VisualNovel((
    title: "Summer Romance",
    author: "Studio Name",
    description: Some("A heartwarming visual novel"),
    
    settings: (
        resolution: (1920, 1080),
        text_speed: Normal,
        auto_save: true,
        skip_mode: Read,
    ),
    
    starting_scene: "opening",
    
    scenes: {
        "opening": (
            background: Some("school_entrance.png"),
            music: Some("gentle_morning.ogg"),
            characters: [
                (id: "maya", sprite: "maya_happy.png", position: Left),
            ],
            dialogue: [
                (
                    speaker: Some("Maya"),
                    text: "Good morning! Ready for the first day?",
                    voice: None,
                    choices: Some([
                        (text: "Absolutely!", target: "enthusiastic"),
                        (text: "I'm nervous...", target: "nervous"),
                    ]),
                    effects: Some([Sound("bell.ogg")]),
                ),
            ],
        ),
    },
    
    characters: {
        "maya": (
            name: "Maya Sakura",
            color: Some("#ff69b4"),
            sprites: {
                "happy": "maya_happy.png",
                "sad": "maya_sad.png",
                "surprised": "maya_surprised.png",
            },
        ),
    },
    
    assets: (
        backgrounds: {
            "school_entrance": "assets/bg/school_entrance.png",
        },
        sprites: {},
        music: {},
        sounds: {},
        voices: {},
    ),
))
```

### Interactive Fiction (RON)

```rust
InteractiveFiction((
    title: "The Corporate Ladder",
    author: "IF Studios",
    description: Some("A corporate thriller"),
    
    settings: (
        show_stats: true,
        checkpoint_saves: true,
        timed_choices: false,
        quality_caps: true,
    ),
    
    starting_node: "interview",
    
    nodes: {
        "interview": (
            content: "You sit across from the interviewer, resume in hand.",
            choices: [
                (
                    text: "Present confidently",
                    target: "confident",
                    conditions: None,
                    consequences: Some([
                        SetQuality("confidence", 1),
                    ]),
                ),
                (
                    text: "Admit nervousness",
                    target: "honest",
                    conditions: None,
                    consequences: Some([
                        SetQuality("honesty", 1),
                    ]),
                ),
            ],
            conditions: None,
            consequences: None,
        ),
    },
    
    qualities: {
        "confidence": (initial: 5, min: Some(0), max: Some(10), hidden: false),
        "honesty": (initial: 5, min: Some(0), max: Some(10), hidden: false),
        "reputation": (initial: 0, min: Some(-10), max: Some(10), hidden: false),
    },
    
    storylets: Some([
        (
            id: "promotion_opportunity",
            title: "A Chance for Advancement",
            conditions: [
                QualityAtLeast("reputation", 5),
                QualityAtLeast("confidence", 7),
            ],
            content: (
                content: "Your manager calls you into their office...",
                choices: [],
                conditions: None,
                consequences: None,
            ),
            priority: 10,
            repeatable: false,
        ),
    ]),
))
```

## API Reference

### Core Types

```rust
// Engine configuration
pub struct EngineConfig {
    pub mode: GameMode,
    pub debug: bool,
    pub max_inventory: usize,
    pub auto_save: bool,
    pub history_size: usize,
    pub typo_correction: bool,
    pub typo_threshold: i64,
}

// Game modes
pub enum GameMode {
    TextAdventure,
    VisualNovel,
    InteractiveFiction,
}

// Response from engine
pub struct Response {
    pub text: String,
    pub mode: GameMode,
    pub location: Option<String>,
    pub choices: Vec<String>,
    pub media: Option<MediaContent>,
    pub effects: Vec<Effect>,
    pub state_changes: Vec<StateChange>,
    pub score: Option<i32>,
    pub ended: bool,
}
```

### Engine Methods

```rust
impl Engine {
    // Create new engine with default config
    pub fn new() -> Self;
    
    // Create engine with custom config
    pub fn with_config(config: EngineConfig) -> Self;
    
    // Load game script (auto-detects format)
    pub fn load_script(&mut self, source: &str) -> Result<()>;
    
    // Start the game
    pub fn start(&mut self) -> Result<Response>;
    
    // Process player input
    pub fn process_input(&mut self, input: &str) -> Result<Response>;
    
    // Save game to slot
    pub fn save_game(&self, slot: Option<usize>) -> Result<Response>;
    
    // Load game from slot
    pub fn load_game(&mut self, slot: Option<usize>) -> Result<Response>;
    
    // Get current game state
    pub fn get_state(&self) -> &GameState;
    
    // Register custom extension
    pub fn register_extension(&mut self, ext: Box<dyn Extension>);
}
```

## Building from Source

### Prerequisites

- Rust 1.70 or later
- wasm-pack (for WebAssembly builds)

### Native Build

```bash
# Clone the repository
git clone https://github.com/plotscript/engine.plotscri.pt
cd engine.plotscri.pt

# Build the library
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build documentation
cargo doc --open
```

### WebAssembly Build

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web browsers
wasm-pack build --target web --out-dir pkg

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg

# Optimize WASM size
wasm-opt -Oz -o pkg/plotscript_bg_opt.wasm pkg/plotscript_bg.wasm
```

## Examples

The `examples/` directory contains complete game examples:

```bash
# Run the text adventure example
cargo run --example adventure

# Run the visual novel example
cargo run --example visual_novel

# Run the interactive fiction example
cargo run --example interactive_fiction

# Test RON script loading
cargo run --example test_ron_loading
```

## Testing

PlotScript has comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test engine_tests
cargo test --test parser_tests
cargo test --test world_tests
cargo test --test script_tests

# Run with verbose output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Performance

- **Parser Response**: <50ms for typical commands
- **Save/Load**: <100ms for average game state
- **Memory Usage**: <10MB for typical game
- **WASM Bundle**: <500KB compressed
- **Startup Time**: <100ms

## Architecture

```
plotscript/
├── src/
│   ├── engine/          # Core engine implementation
│   │   ├── mod.rs       # Engine struct and main logic
│   │   ├── commands.rs  # Command parsing with fuzzy matching
│   │   ├── formatter.rs # Output formatting
│   │   └── save.rs      # Save/load system
│   ├── parser/          # PEG parser and AST
│   │   ├── grammar.pest # Grammar definition
│   │   ├── ast.rs       # Abstract syntax tree
│   │   ├── fuzzy.rs     # Fuzzy matching implementation
│   │   └── mod.rs       # Parser implementation
│   ├── world/           # World model
│   │   ├── mod.rs       # World state management
│   │   ├── graph.rs     # Location graph
│   │   └── query.rs     # World queries
│   ├── runtime/         # Script execution
│   │   ├── evaluator.rs # Expression evaluation
│   │   └── functions.rs # Built-in functions
│   ├── script/          # Script format definitions
│   │   └── mod.rs       # RON/YAML structures
│   ├── wasm/            # WebAssembly bindings
│   │   └── mod.rs       # WASM API
│   └── lib.rs           # Library entry point
```

## License

PlotScript Engine is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where we especially welcome help:
- Parser improvements and optimizations
- Additional language support
- More comprehensive examples
- Documentation improvements
- Performance optimizations

## Community

- [Official Website](https://plotscri.pt)
- [Documentation](https://docs.plotscri.pt)
- [Discord Community](https://discord.gg/plotscript)
- [GitHub Discussions](https://github.com/plotscript/engine.plotscri.pt/discussions)

---

Part of the [PlotScript Platform](https://plotscri.pt) - empowering interactive storytelling.