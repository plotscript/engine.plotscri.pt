//! WebAssembly bindings for PlotScript engine

use wasm_bindgen::prelude::*;
use crate::{Engine, EngineConfig, GameMode};

/// WASM-compatible engine wrapper
#[wasm_bindgen]
pub struct WasmEngine {
    engine: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    /// Create a new engine instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize panic hook for better error messages
        console_error_panic_hook::set_once();
        
        Self {
            engine: Engine::new(),
        }
    }
    
    /// Create engine with config (JSON string)
    #[wasm_bindgen(js_name = "withConfig")]
    pub fn with_config(config_json: &str) -> Result<WasmEngine, JsValue> {
        let config: EngineConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(Self {
            engine: Engine::with_config(config),
        })
    }
    
    /// Load a script
    #[wasm_bindgen(js_name = "loadScript")]
    pub fn load_script(&mut self, source: &str) -> Result<(), JsValue> {
        self.engine.load_script(source)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Start the game
    pub fn start(&mut self) -> Result<String, JsValue> {
        let response = self.engine.start()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        serde_json::to_string(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Process player input
    #[wasm_bindgen(js_name = "processInput")]
    pub fn process_input(&mut self, input: &str) -> Result<String, JsValue> {
        let response = self.engine.process_input(input)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        serde_json::to_string(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Save game state
    pub fn save(&mut self, slot: Option<u8>) -> Result<String, JsValue> {
        let response = self.engine.save_game(slot)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        serde_json::to_string(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Load game state
    pub fn load(&mut self, slot: Option<u8>) -> Result<String, JsValue> {
        let response = self.engine.load_game(slot)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        serde_json::to_string(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// Get current game mode
    #[wasm_bindgen(js_name = "getGameMode")]
    pub fn get_game_mode(&self) -> String {
        match self.engine.config.mode {
            GameMode::TextAdventure => "text_adventure",
            GameMode::VisualNovel => "visual_novel",
            GameMode::InteractiveFiction => "interactive_fiction",
        }.to_string()
    }
}

/// Export the version string
#[wasm_bindgen]
pub fn version() -> String {
    crate::VERSION.to_string()
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    crate::init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_wasm_engine_creation() {
        let engine = WasmEngine::new();
        assert_eq!(engine.get_game_mode(), "interactive_fiction");
    }
}