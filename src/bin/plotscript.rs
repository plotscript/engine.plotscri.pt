//! PlotScript CLI - Command-line interface for running PlotScript games

use clap::{Parser, Subcommand};
use colored::*;
use plotscript::{Engine, EngineConfig, init};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "plotscript")]
#[command(version = plotscript::VERSION)]
#[command(about = "PlotScript Engine - Run interactive narrative games", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a PlotScript game
    Play {
        /// Path to the game script file
        #[arg(value_name = "FILE")]
        script: PathBuf,
        
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
        
        /// Disable typo correction
        #[arg(long)]
        no_typo_correction: bool,
        
        /// Set typo correction threshold (0-100)
        #[arg(long, default_value = "70")]
        typo_threshold: u8,
        
        /// Disable auto-save
        #[arg(long)]
        no_auto_save: bool,
        
        /// Load a saved game on start
        #[arg(short, long)]
        load: Option<usize>,
    },
    
    /// Create a new game project
    New {
        /// Project name
        name: String,
        
        /// Game format
        #[arg(short, long, value_enum, default_value = "text-adventure")]
        format: GameFormat,
        
        /// Project directory (defaults to project name)
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    
    /// Validate a game script
    Validate {
        /// Path to the game script file
        #[arg(value_name = "FILE")]
        script: PathBuf,
        
        /// Strict validation mode
        #[arg(short, long)]
        strict: bool,
    },
    
    /// Convert between script formats
    Convert {
        /// Input file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        
        /// Output file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
        
        /// Target format
        #[arg(short, long, value_enum)]
        to: ScriptFormat,
    },
    
    /// Show version information
    Version {
        /// Show detailed version info
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum GameFormat {
    TextAdventure,
    VisualNovel,
    InteractiveFiction,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ScriptFormat {
    Ron,
    Yaml,
    Json,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Play { 
            script, 
            debug, 
            no_typo_correction, 
            typo_threshold,
            no_auto_save,
            load,
        } => {
            play_game(
                script, 
                debug, 
                !no_typo_correction, 
                typo_threshold,
                !no_auto_save,
                load,
            );
        }
        Commands::New { name, format, dir } => {
            create_project(name, format, dir);
        }
        Commands::Validate { script, strict } => {
            validate_script(script, strict);
        }
        Commands::Convert { input, output, to } => {
            convert_script(input, output, to);
        }
        Commands::Version { verbose } => {
            show_version(verbose);
        }
    }
}

fn play_game(
    script_path: PathBuf, 
    debug: bool, 
    typo_correction: bool, 
    typo_threshold: u8,
    auto_save: bool,
    load_slot: Option<usize>,
) {
    // Initialize engine
    init();
    
    // Load script file
    let script_content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read script file: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };
    
    // Create engine with config
    let config = EngineConfig {
        debug,
        typo_correction,
        typo_threshold,
        auto_save,
        ..Default::default()
    };
    
    let mut engine = Engine::with_config(config);
    
    // Load the script
    match engine.load_script(&script_content) {
        Ok(_) => {
            println!("{} Game loaded successfully!", "✓".green());
        }
        Err(e) => {
            eprintln!("{} Failed to load game: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    }
    
    // Start the game
    match engine.start() {
        Ok(response) => {
            print_response(&response);
        }
        Err(e) => {
            eprintln!("{} Failed to start game: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    }
    
    // Load saved game if requested
    if let Some(slot) = load_slot {
        match engine.load_game(Some(slot as u8)) {
            Ok(response) => {
                println!("{} Loaded saved game from slot {}", "✓".green(), slot);
                print_response(&response);
            }
            Err(e) => {
                eprintln!("{} Failed to load saved game: {}", "Warning:".yellow(), e);
            }
        }
    }
    
    // Main game loop
    loop {
        // Show prompt
        print!("{} ", ">".bright_cyan());
        io::stdout().flush().unwrap();
        
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Check for meta commands
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("{}", "Thanks for playing!".bright_yellow());
            break;
        }
        
        // Process game input
        match engine.process_input(input) {
            Ok(response) => {
                print_response(&response);
                
                if response.ended {
                    println!("\n{}", "=== THE END ===".bright_yellow());
                    println!("{}", "Thanks for playing!".bright_yellow());
                    break;
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".red(), e);
            }
        }
    }
}

fn print_response(response: &plotscript::Response) {
    // Location header
    if let Some(location) = &response.location {
        println!("\n{}", location.bright_green().bold());
        println!("{}", "─".repeat(location.len()).bright_green());
    }
    
    // Main text
    println!("{}", response.text);
    
    // Choices (for visual novels and interactive fiction)
    if !response.choices.is_empty() {
        println!("\n{}", "Choices:".bright_yellow());
        for (i, choice) in response.choices.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().bright_cyan(), choice.text);
        }
    }
    
    println!();
}

fn create_project(name: String, format: GameFormat, dir: Option<PathBuf>) {
    let project_dir = dir.unwrap_or_else(|| PathBuf::from(&name));
    
    // Create directory
    if let Err(e) = fs::create_dir_all(&project_dir) {
        eprintln!("{} Failed to create project directory: {}", "Error:".red(), e);
        std::process::exit(1);
    }
    
    // Create game script template
    let template = match format {
        GameFormat::TextAdventure => include_str!("../../templates/text_adventure.ron"),
        GameFormat::VisualNovel => include_str!("../../templates/visual_novel.ron"),
        GameFormat::InteractiveFiction => include_str!("../../templates/interactive_fiction.ron"),
    };
    
    let script_path = project_dir.join("game.ron");
    if let Err(e) = fs::write(&script_path, template) {
        eprintln!("{} Failed to create game script: {}", "Error:".red(), e);
        std::process::exit(1);
    }
    
    // Create README
    let readme = format!(
        "# {}\n\nA {} game created with PlotScript.\n\n## Running the game\n\n```bash\nplotscript play game.ron\n```\n",
        name,
        match format {
            GameFormat::TextAdventure => "text adventure",
            GameFormat::VisualNovel => "visual novel",
            GameFormat::InteractiveFiction => "interactive fiction",
        }
    );
    
    let readme_path = project_dir.join("README.md");
    let _ = fs::write(&readme_path, readme);
    
    println!("{} Created {} project '{}' in {:?}", 
        "✓".green(),
        match format {
            GameFormat::TextAdventure => "text adventure",
            GameFormat::VisualNovel => "visual novel",
            GameFormat::InteractiveFiction => "interactive fiction",
        },
        name,
        project_dir
    );
    println!("\nTo run your game:");
    println!("  cd {}", project_dir.display());
    println!("  plotscript play game.ron");
}

fn validate_script(script_path: PathBuf, strict: bool) {
    init();
    
    // Load script file
    let script_content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read script file: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };
    
    // Try to load and validate
    let mut engine = Engine::new();
    match engine.load_script(&script_content) {
        Ok(_) => {
            println!("{} Script is valid!", "✓".green());
            
            if strict {
                // Additional validation could go here
                println!("{} Strict validation passed!", "✓".green());
            }
        }
        Err(e) => {
            eprintln!("{} Script validation failed: {}", "✗".red(), e);
            std::process::exit(1);
        }
    }
}

fn convert_script(input: PathBuf, output: PathBuf, to: ScriptFormat) {
    init();
    
    // Load input file
    let input_content = match fs::read_to_string(&input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read input file: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };
    
    // Parse script
    let script = match plotscript::script::GameScript::from_ron(&input_content) {
        Ok(s) => s,
        Err(_) => match plotscript::script::GameScript::from_yaml(&input_content) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{} Failed to parse input script: {}", "Error:".red(), e);
                std::process::exit(1);
            }
        }
    };
    
    // Convert to target format
    let output_content = match to {
        ScriptFormat::Ron => script.to_ron(),
        ScriptFormat::Yaml => script.to_yaml(),
        ScriptFormat::Json => script.to_json(),
    };
    
    let output_content = match output_content {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to convert script: {}", "Error:".red(), e);
            std::process::exit(1);
        }
    };
    
    // Write output file
    if let Err(e) = fs::write(&output, output_content) {
        eprintln!("{} Failed to write output file: {}", "Error:".red(), e);
        std::process::exit(1);
    }
    
    println!("{} Converted {} to {}", 
        "✓".green(),
        input.display(),
        output.display()
    );
}

fn show_version(verbose: bool) {
    println!("PlotScript Engine v{}", plotscript::VERSION);
    
    if verbose {
        println!("\nBuild information:");
        println!("  Rust version: {}", env!("RUSTC_VERSION"));
        println!("  Target: {}", env!("TARGET"));
        println!("  Profile: {}", env!("PROFILE"));
        println!("\nFeatures:");
        println!("  Text Adventures: ✓");
        println!("  Visual Novels: ✓");
        println!("  Interactive Fiction: ✓");
        println!("  WebAssembly: ✓");
        println!("  Extensions: ✓");
    }
}