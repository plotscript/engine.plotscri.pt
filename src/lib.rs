//! PlotScript Engine - Core library for interactive narratives
//!
//! This crate provides a complete engine for creating and running interactive
//! stories in three formats: text adventures, visual novels, and interactive fiction.

#![warn(missing_docs)]

#[cfg(target_arch = "wasm32")]
use wee_alloc::WeeAlloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub mod engine;
pub mod error;
pub mod parser;
pub mod runtime;
pub mod script;
pub mod scripting;
pub mod types;
pub mod world;
pub mod visual_novel;
pub mod interactive_fiction;
pub mod extensions;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub mod io;

// Re-exports
pub use engine::{Engine, EngineConfig};
pub use error::{Error, Result};
pub use types::{GameMode, Response};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the engine (sets up panic hook for WASM)
pub fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    
    INIT.call_once(|| {
        #[cfg(target_arch = "wasm32")]
        {
            console_error_panic_hook::set_once();
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = env_logger::try_init();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}