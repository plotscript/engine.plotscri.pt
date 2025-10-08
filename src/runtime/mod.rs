//! Script runtime for executing PlotScript code

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::parser::{self, Script, Statement, Command, Function};
use crate::types::Value;
use crate::world::World;

mod evaluator;
mod functions;

pub use functions::*;

/// Runtime environment for script execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runtime {
    /// User-defined functions
    functions: HashMap<String, Function>,
    /// Built-in functions
    #[serde(skip)]
    builtins: HashMap<String, BuiltinFunction>,
    /// Call stack for debugging
    #[serde(skip)]
    call_stack: Vec<String>,
    /// Maximum call stack depth
    max_call_depth: usize,
}

impl Runtime {
    /// Create a new runtime
    pub fn new() -> Self {
        let mut runtime = Self {
            functions: HashMap::new(),
            builtins: HashMap::new(),
            call_stack: Vec::new(),
            max_call_depth: 100,
        };
        
        // Register built-in functions
        runtime.register_builtins();
        runtime
    }
    
    /// Load functions from a script
    pub fn load_script(&mut self, script: &Script) -> Result<()> {
        // Clear existing functions
        self.functions.clear();
        
        // Load all functions
        for (name, function) in &script.functions {
            self.functions.insert(name.clone(), function.clone());
        }
        
        Ok(())
    }
    
    /// Register built-in functions
    fn register_builtins(&mut self) {
        // Math functions
        self.builtins.insert("min".to_string(), builtin_min);
        self.builtins.insert("max".to_string(), builtin_max);
        self.builtins.insert("abs".to_string(), builtin_abs);
        self.builtins.insert("round".to_string(), builtin_round);
        self.builtins.insert("random".to_string(), builtin_random);
        
        // String functions
        self.builtins.insert("len".to_string(), builtin_len);
        self.builtins.insert("upper".to_string(), builtin_upper);
        self.builtins.insert("lower".to_string(), builtin_lower);
        self.builtins.insert("contains".to_string(), builtin_contains);
        
        // Game functions
        self.builtins.insert("has_item".to_string(), builtin_has_item);
        self.builtins.insert("has_flag".to_string(), builtin_has_flag);
        self.builtins.insert("visited".to_string(), builtin_visited);
        self.builtins.insert("score".to_string(), builtin_score);
        self.builtins.insert("turns".to_string(), builtin_turns);
    }
    
    /// Execute a statement
    pub fn execute_statement(&mut self, stmt: &Statement, world: &mut World) -> Result<Option<Value>> {
        match stmt {
            Statement::Assignment { variable, value } => {
                let val = self.evaluate_expression(value, world)?;
                world.set_variable(variable.clone(), val);
                Ok(None)
            }
            
            Statement::If { condition, then_block, else_block } => {
                let cond_result = self.evaluate_condition(condition, world)?;
                
                if cond_result {
                    for stmt in then_block {
                        if let Some(ret) = self.execute_statement(stmt, world)? {
                            return Ok(Some(ret));
                        }
                    }
                } else if let Some(else_block) = else_block {
                    for stmt in else_block {
                        if let Some(ret) = self.execute_statement(stmt, world)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                
                Ok(None)
            }
            
            Statement::While { condition, body } => {
                let mut iterations = 0;
                const MAX_ITERATIONS: usize = 10000;
                
                while self.evaluate_condition(condition, world)? {
                    iterations += 1;
                    if iterations > MAX_ITERATIONS {
                        return Err(Error::RuntimeError("Infinite loop detected".to_string()));
                    }
                    
                    for stmt in body {
                        if let Some(ret) = self.execute_statement(stmt, world)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                
                Ok(None)
            }
            
            Statement::For { variable, collection, body } => {
                let coll_value = self.evaluate_expression(collection, world)?;
                
                match coll_value {
                    Value::List(items) => {
                        for item in items {
                            world.set_variable(variable.clone(), item);
                            
                            for stmt in body {
                                if let Some(ret) = self.execute_statement(stmt, world)? {
                                    return Ok(Some(ret));
                                }
                            }
                        }
                    }
                    _ => return Err(Error::RuntimeError("For loop requires a list".to_string())),
                }
                
                Ok(None)
            }
            
            Statement::Call { function, arguments } => {
                let args: Vec<Value> = arguments.iter()
                    .map(|arg| self.evaluate_expression(arg, world))
                    .collect::<Result<Vec<_>>>()?;
                
                self.call_function(function, args, world)?;
                Ok(None)
            }
            
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr, world)?;
                // TODO: Handle print output properly
                println!("{:?}", value);
                Ok(None)
            }
            
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.evaluate_expression(expr, world)?;
                    Ok(Some(value))
                } else {
                    Ok(Some(Value::Null))
                }
            }
            
            Statement::Command(cmd) => {
                self.execute_command(cmd, world)?;
                Ok(None)
            }
        }
    }
    
    /// Execute a command
    fn execute_command(&mut self, cmd: &Command, world: &mut World) -> Result<()> {
        match cmd {
            Command::Move { object, location } => {
                // Move an item to a new location
                if let Some(item) = world.items.get_mut(object) {
                    item.location = Some(location.clone());
                }
                Ok(())
            }
            
            Command::Give { item, character: _ } => {
                // Transfer item to character
                if world.inventory.contains(&item) {
                    world.inventory.retain(|i| i != item);
                    // TODO: Add to character inventory
                }
                Ok(())
            }
            
            Command::Unlock(target) => {
                // TODO: Implement unlock logic
                world.set_flag(format!("{}_unlocked", target), true);
                Ok(())
            }
            
            Command::Lock(target) => {
                // TODO: Implement lock logic
                world.set_flag(format!("{}_unlocked", target), false);
                Ok(())
            }
            
            Command::Reveal(target) => {
                world.set_flag(format!("{}_revealed", target), true);
                Ok(())
            }
            
            Command::Hide(target) => {
                world.set_flag(format!("{}_revealed", target), false);
                Ok(())
            }
            
            Command::PlaySound(_sound) => {
                // TODO: Handle sound playback
                Ok(())
            }
            
            Command::ShowImage { path: _, position: _ } => {
                // TODO: Handle image display
                Ok(())
            }
            
            Command::SetVariable { name, value } => {
                world.set_variable(name.clone(), value.clone());
                Ok(())
            }
        }
    }
    
    /// Check a condition
    pub fn check_condition(&mut self, condition: &parser::Condition, world: &World) -> Result<bool> {
        // Need mutable world for evaluate_expression
        let mut world_copy = world.clone();
        let left = self.evaluate_expression(&condition.left, &mut world_copy)?;
        let right = self.evaluate_expression(&condition.right, &mut world_copy)?;
        
        Ok(match condition.operator {
            parser::ComparisonOp::Eq => left == right,
            parser::ComparisonOp::Ne => left != right,
            parser::ComparisonOp::Lt => {
                match (left, right) {
                    (Value::Integer(l), Value::Integer(r)) => l < r,
                    (Value::Float(l), Value::Float(r)) => l < r,
                    _ => false,
                }
            }
            parser::ComparisonOp::Le => {
                match (left, right) {
                    (Value::Integer(l), Value::Integer(r)) => l <= r,
                    (Value::Float(l), Value::Float(r)) => l <= r,
                    _ => false,
                }
            }
            parser::ComparisonOp::Gt => {
                match (left, right) {
                    (Value::Integer(l), Value::Integer(r)) => l > r,
                    (Value::Float(l), Value::Float(r)) => l > r,
                    _ => false,
                }
            }
            parser::ComparisonOp::Ge => {
                match (left, right) {
                    (Value::Integer(l), Value::Integer(r)) => l >= r,
                    (Value::Float(l), Value::Float(r)) => l >= r,
                    _ => false,
                }
            }
        })
    }
    
    /// Execute an action (Statement)
    pub fn execute_action(&mut self, action: &parser::Statement, world: &mut World) -> Result<()> {
        self.execute_statement(action, world)?;
        Ok(())
    }
    
    
    /// Call a function
    pub fn call_function(&mut self, name: &str, args: Vec<Value>, world: &mut World) -> Result<Value> {
        // Check for stack overflow
        if self.call_stack.len() >= self.max_call_depth {
            return Err(Error::RuntimeError("Stack overflow".to_string()));
        }
        
        // Check built-ins first
        if let Some(builtin) = self.builtins.get(name) {
            return builtin(args, world);
        }
        
        // Check user-defined functions
        if let Some(function) = self.functions.get(name).cloned() {
            // Push to call stack
            self.call_stack.push(name.to_string());
            
            // Check parameter count
            if args.len() != function.parameters.len() {
                self.call_stack.pop();
                return Err(Error::RuntimeError(format!(
                    "Function '{}' expects {} arguments, got {}",
                    name, function.parameters.len(), args.len()
                )));
            }
            
            // Create local scope by saving current variables
            let saved_vars = world.variables.clone();
            
            // Bind parameters
            for (param, arg) in function.parameters.iter().zip(args.iter()) {
                world.set_variable(param.clone(), arg.clone());
            }
            
            // Execute function body
            let mut result = Value::Null;
            for stmt in &function.body {
                if let Some(ret) = self.execute_statement(stmt, world)? {
                    result = ret;
                    break;
                }
            }
            
            // Restore variables
            world.variables = saved_vars;
            
            // Pop from call stack
            self.call_stack.pop();
            
            Ok(result)
        } else {
            Err(Error::RuntimeError(format!("Unknown function: {}", name)))
        }
    }
    
    /// Run an event by name
    pub fn run_event(&mut self, _event_name: &str, _world: &mut World) -> Result<()> {
        // TODO: Implement event system
        Ok(())
    }
    
    /// Run room events
    pub fn run_room_event(&mut self, room_id: &str, event_type: &str, world: &mut World) -> Result<()> {
        let statements = if let Some(room) = world.rooms.get(room_id) {
            match event_type {
                "on_enter" => room.on_enter.clone(),
                "on_exit" => room.on_exit.clone(),
                _ => return Ok(()),
            }
        } else {
            return Ok(());
        };
        
        for stmt in &statements {
            self.execute_statement(stmt, world)?;
        }
        
        Ok(())
    }
    
    /// Run item events
    pub fn run_item_event(&mut self, item_id: &str, event_type: &str, world: &mut World) -> Result<()> {
        let statements = if let Some(item) = world.items.get(item_id) {
            match event_type {
                "on_take" => item.on_take.clone(),
                "on_use" => item.on_use.clone(),
                "on_examine" => item.on_examine.clone(),
                _ => return Ok(()),
            }
        } else {
            return Ok(());
        };
        
        for stmt in &statements {
            self.execute_statement(stmt, world)?;
        }
        
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Type for built-in functions
type BuiltinFunction = fn(Vec<Value>, &mut World) -> Result<Value>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    
    #[test]
    fn test_variable_assignment() {
        let mut runtime = Runtime::new();
        let mut world = World::new();
        
        let stmt = Statement::Assignment {
            variable: "test".to_string(),
            value: Expression::Literal(Value::Integer(42)),
        };
        
        runtime.execute_statement(&stmt, &mut world).unwrap();
        assert_eq!(world.get_variable("test"), Some(&Value::Integer(42)));
    }
}