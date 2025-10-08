//! Extension system for adding custom commands and functionality
//!
//! This module provides a plugin architecture that allows developers to extend
//! the PlotScript engine with custom commands, conditions, actions, and more.

use crate::{Engine, Response, Result, Error};
use crate::types::{GameState, Value};
use std::collections::HashMap;

/// Trait for custom command extensions
pub trait Extension: Send + Sync {
    /// Returns the name of this extension
    fn name(&self) -> &str;
    
    /// Returns the version of this extension
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    /// Called when the extension is loaded
    fn on_load(&mut self, _engine: &mut Engine) -> Result<()> {
        Ok(())
    }
    
    /// Called when the extension is unloaded
    fn on_unload(&mut self, _engine: &mut Engine) -> Result<()> {
        Ok(())
    }
    
    /// Process a command if this extension handles it
    fn process_command(&mut self, command: &str, args: &[&str], state: &mut GameState) -> Option<Response>;
    
    /// Register custom verbs this extension handles
    fn get_verbs(&self) -> Vec<&str> {
        vec![]
    }
    
    /// Get extension metadata
    fn metadata(&self) -> ExtensionMetadata {
        ExtensionMetadata {
            name: self.name().to_string(),
            version: self.version().to_string(),
            author: String::new(),
            description: String::new(),
            dependencies: vec![],
        }
    }
}

/// Metadata about an extension
#[derive(Debug, Clone)]
pub struct ExtensionMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

/// Trait for custom conditions
pub trait Condition: Send + Sync {
    /// Evaluate this condition
    fn evaluate(&self, state: &GameState, args: &[Value]) -> Result<bool>;
    
    /// Get the name of this condition
    fn name(&self) -> &str;
    
    /// Get the number of arguments this condition expects
    fn arg_count(&self) -> usize {
        0
    }
}

/// Trait for custom actions
pub trait Action: Send + Sync {
    /// Execute this action
    fn execute(&self, state: &mut GameState, args: &[Value]) -> Result<()>;
    
    /// Get the name of this action
    fn name(&self) -> &str;
    
    /// Get the number of arguments this action expects
    fn arg_count(&self) -> usize {
        0
    }
}

/// Function-based condition implementation
pub struct FunctionCondition<F>
where
    F: Fn(&GameState, &[Value]) -> Result<bool> + Send + Sync,
{
    name: String,
    func: F,
    arg_count: usize,
}

impl<F> FunctionCondition<F>
where
    F: Fn(&GameState, &[Value]) -> Result<bool> + Send + Sync,
{
    pub fn new(name: impl Into<String>, arg_count: usize, func: F) -> Self {
        Self {
            name: name.into(),
            func,
            arg_count,
        }
    }
}

impl<F> Condition for FunctionCondition<F>
where
    F: Fn(&GameState, &[Value]) -> Result<bool> + Send + Sync,
{
    fn evaluate(&self, state: &GameState, args: &[Value]) -> Result<bool> {
        if args.len() != self.arg_count {
            return Err(Error::InvalidArguments(format!(
                "Condition '{}' expects {} arguments, got {}",
                self.name,
                self.arg_count,
                args.len()
            )));
        }
        (self.func)(state, args)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn arg_count(&self) -> usize {
        self.arg_count
    }
}

/// Function-based action implementation
pub struct FunctionAction<F>
where
    F: Fn(&mut GameState, &[Value]) -> Result<()> + Send + Sync,
{
    name: String,
    func: F,
    arg_count: usize,
}

impl<F> FunctionAction<F>
where
    F: Fn(&mut GameState, &[Value]) -> Result<()> + Send + Sync,
{
    pub fn new(name: impl Into<String>, arg_count: usize, func: F) -> Self {
        Self {
            name: name.into(),
            func,
            arg_count,
        }
    }
}

impl<F> Action for FunctionAction<F>
where
    F: Fn(&mut GameState, &[Value]) -> Result<()> + Send + Sync,
{
    fn execute(&self, state: &mut GameState, args: &[Value]) -> Result<()> {
        if args.len() != self.arg_count {
            return Err(Error::InvalidArguments(format!(
                "Action '{}' expects {} arguments, got {}",
                self.name,
                self.arg_count,
                args.len()
            )));
        }
        (self.func)(state, args)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn arg_count(&self) -> usize {
        self.arg_count
    }
}

/// Manager for extensions
pub struct ExtensionManager {
    extensions: HashMap<String, Box<dyn Extension>>,
    conditions: HashMap<String, Box<dyn Condition>>,
    actions: HashMap<String, Box<dyn Action>>,
    verb_mapping: HashMap<String, String>, // verb -> extension name
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            conditions: HashMap::new(),
            actions: HashMap::new(),
            verb_mapping: HashMap::new(),
        }
    }
    
    /// Register an extension
    pub fn register_extension(&mut self, extension: Box<dyn Extension>) -> Result<()> {
        let name = extension.name().to_string();
        
        // Register verbs
        for verb in extension.get_verbs() {
            self.verb_mapping.insert(verb.to_string(), name.clone());
        }
        
        // Store extension
        self.extensions.insert(name, extension);
        
        Ok(())
    }
    
    /// Unregister an extension
    pub fn unregister_extension(&mut self, name: &str) -> Result<()> {
        if let Some(_extension) = self.extensions.remove(name) {
            // Remove verb mappings
            self.verb_mapping.retain(|_, ext_name| ext_name != name);
        }
        
        Ok(())
    }
    
    /// Register a custom condition
    pub fn register_condition(&mut self, condition: Box<dyn Condition>) -> Result<()> {
        let name = condition.name().to_string();
        if self.conditions.contains_key(&name) {
            return Err(Error::ExtensionError(format!(
                "Condition '{}' is already registered",
                name
            )));
        }
        self.conditions.insert(name, condition);
        Ok(())
    }
    
    /// Register a custom action
    pub fn register_action(&mut self, action: Box<dyn Action>) -> Result<()> {
        let name = action.name().to_string();
        if self.actions.contains_key(&name) {
            return Err(Error::ExtensionError(format!(
                "Action '{}' is already registered",
                name
            )));
        }
        self.actions.insert(name, action);
        Ok(())
    }
    
    /// Process a command through extensions
    pub fn process_command(&mut self, command: &str, args: &[&str], state: &mut GameState) -> Option<Response> {
        // Check if any extension handles this verb
        if let Some(ext_name) = self.verb_mapping.get(command).cloned() {
            if let Some(extension) = self.extensions.get_mut(&ext_name) {
                return extension.process_command(command, args, state);
            }
        }
        
        // Try all extensions
        for extension in self.extensions.values_mut() {
            if let Some(response) = extension.process_command(command, args, state) {
                return Some(response);
            }
        }
        
        None
    }
    
    /// Evaluate a custom condition
    pub fn evaluate_condition(&self, name: &str, state: &GameState, args: &[Value]) -> Result<bool> {
        if let Some(condition) = self.conditions.get(name) {
            condition.evaluate(state, args)
        } else {
            Err(Error::UnknownCondition(name.to_string()))
        }
    }
    
    /// Execute a custom action
    pub fn execute_action(&self, name: &str, state: &mut GameState, args: &[Value]) -> Result<()> {
        if let Some(action) = self.actions.get(name) {
            action.execute(state, args)
        } else {
            Err(Error::UnknownAction(name.to_string()))
        }
    }
    
    /// Get all registered extensions
    pub fn list_extensions(&self) -> Vec<ExtensionMetadata> {
        self.extensions
            .values()
            .map(|ext| ext.metadata())
            .collect()
    }
    
    /// Check if an extension is loaded
    pub fn has_extension(&self, name: &str) -> bool {
        self.extensions.contains_key(name)
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Example extension that adds magic commands
pub mod examples {
    use super::*;
    
    pub struct MagicExtension {
        spells: HashMap<String, String>,
    }
    
    impl MagicExtension {
        pub fn new() -> Self {
            let mut spells = HashMap::new();
            spells.insert("fireball".to_string(), "You cast a blazing fireball!".to_string());
            spells.insert("heal".to_string(), "You feel refreshed as healing energy flows through you.".to_string());
            spells.insert("teleport".to_string(), "You vanish in a puff of smoke!".to_string());
            
            Self { spells }
        }
    }
    
    impl Extension for MagicExtension {
        fn name(&self) -> &str {
            "magic"
        }
        
        fn version(&self) -> &str {
            "1.0.0"
        }
        
        fn get_verbs(&self) -> Vec<&str> {
            vec!["cast", "enchant", "dispel"]
        }
        
        fn process_command(&mut self, command: &str, args: &[&str], state: &mut GameState) -> Option<Response> {
            match command {
                "cast" => {
                    if args.is_empty() {
                        return Some(Response {
                            text: "Cast what spell?".to_string(),
                            success: false,
                            ..Default::default()
                        });
                    }
                    
                    let spell = args[0];
                    if let Some(effect) = self.spells.get(spell) {
                        // Check mana
                        if let Some(Value::Integer(mana)) = state.get_variable("mana") {
                            if *mana < 10 {
                                return Some(Response {
                                    text: "You don't have enough mana!".to_string(),
                                    success: false,
                                    ..Default::default()
                                });
                            }
                            
                            // Consume mana
                            state.set_variable("mana", Value::Integer(mana - 10));
                        }
                        
                        Some(Response {
                            text: effect.clone(),
                            success: true,
                            ..Default::default()
                        })
                    } else {
                        Some(Response {
                            text: format!("You don't know the spell '{}'", spell),
                            success: false,
                            ..Default::default()
                        })
                    }
                }
                "enchant" => {
                    if args.is_empty() {
                        return Some(Response {
                            text: "Enchant what?".to_string(),
                            success: false,
                            ..Default::default()
                        });
                    }
                    
                    Some(Response {
                        text: format!("You enchant the {} with magical energy!", args[0]),
                        success: true,
                        ..Default::default()
                    })
                }
                "dispel" => {
                    Some(Response {
                        text: "You wave your hand and dispel all magical effects in the area.".to_string(),
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
                description: "Adds magic spells and enchantments to the game".to_string(),
                dependencies: vec![],
            }
        }
    }
    
    /// Create a weather system extension
    pub struct WeatherExtension {
        current_weather: String,
        weather_types: Vec<String>,
    }
    
    impl WeatherExtension {
        pub fn new() -> Self {
            Self {
                current_weather: "sunny".to_string(),
                weather_types: vec![
                    "sunny".to_string(),
                    "cloudy".to_string(),
                    "rainy".to_string(),
                    "stormy".to_string(),
                    "snowy".to_string(),
                ],
            }
        }
    }
    
    impl Extension for WeatherExtension {
        fn name(&self) -> &str {
            "weather"
        }
        
        fn get_verbs(&self) -> Vec<&str> {
            vec!["weather", "forecast"]
        }
        
        fn on_load(&mut self, engine: &mut Engine) -> Result<()> {
            // Set initial weather
            engine.state.set_variable("weather", Value::String(self.current_weather.clone()));
            Ok(())
        }
        
        fn process_command(&mut self, command: &str, _args: &[&str], state: &mut GameState) -> Option<Response> {
            match command {
                "weather" => {
                    Some(Response {
                        text: format!("The weather is currently {}.", self.current_weather),
                        success: true,
                        ..Default::default()
                    })
                }
                "forecast" => {
                    // Change weather randomly
                    use rand::seq::SliceRandom;
                    if let Some(new_weather) = self.weather_types.choose(&mut rand::thread_rng()) {
                        self.current_weather = new_weather.clone();
                        state.set_variable("weather", Value::String(new_weather.clone()));
                    }
                    
                    Some(Response {
                        text: format!("The weather is changing to {}!", self.current_weather),
                        success: true,
                        ..Default::default()
                    })
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GameState;
    
    #[test]
    fn test_extension_registration() {
        let mut manager = ExtensionManager::new();
        let extension = Box::new(examples::MagicExtension::new());
        
        assert!(manager.register_extension(extension).is_ok());
        assert!(manager.has_extension("magic"));
        assert_eq!(manager.list_extensions().len(), 1);
    }
    
    #[test]
    fn test_custom_condition() {
        let mut manager = ExtensionManager::new();
        let condition = Box::new(FunctionCondition::new(
            "is_raining",
            0,
            |state: &GameState, _args: &[Value]| {
                match state.get_variable("weather") {
                    Some(Value::String(w)) => Ok(w == "rainy"),
                    _ => Ok(false),
                }
            },
        ));
        
        assert!(manager.register_condition(condition).is_ok());
        
        let mut state = GameState::new();
        state.set_variable("weather", Value::String("rainy".to_string()));
        
        let result = manager.evaluate_condition("is_raining", &state, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
    
    #[test]
    fn test_custom_action() {
        let mut manager = ExtensionManager::new();
        let action = Box::new(FunctionAction::new(
            "thunder",
            0,
            |state: &mut GameState, _args: &[Value]| {
                state.set_variable("scared", Value::Bool(true));
                Ok(())
            },
        ));
        
        assert!(manager.register_action(action).is_ok());
        
        let mut state = GameState::new();
        let result = manager.execute_action("thunder", &mut state, &[]);
        assert!(result.is_ok());
        assert_eq!(state.get_variable("scared"), Some(&Value::Bool(true)));
    }
}