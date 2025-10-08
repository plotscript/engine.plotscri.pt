//! Scripting and variable management system

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::Value;
use crate::error::{Error, Result};

/// Variable scope for managing nested contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    /// Variables in this scope
    variables: HashMap<String, Value>,
    /// Parent scope (if any)
    #[serde(skip)]
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Create a new empty scope
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    /// Create a child scope
    pub fn child(parent: Scope) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }
    
    /// Set a variable value
    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// Update an existing variable (searches parent scopes)
    pub fn update(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.update(name, value)
        } else {
            Err(Error::RuntimeError(format!("Undefined variable: {}", name)))
        }
    }
    
    /// Check if variable exists
    pub fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name) ||
            self.parent.as_ref().map_or(false, |p| p.has(name))
    }
}

/// Variable interpolation for text
pub fn interpolate_text(text: &str, scope: &Scope) -> String {
    let mut result = text.to_string();
    
    // Find all variable references [variable_name]
    let var_pattern = regex::Regex::new(r"\[([a-zA-Z_][a-zA-Z0-9_]*)\]").unwrap();
    
    for cap in var_pattern.captures_iter(text) {
        if let Some(var_name) = cap.get(1) {
            if let Some(value) = scope.get(var_name.as_str()) {
                let replacement = format_value(value);
                result = result.replace(&cap[0], &replacement);
            }
        }
    }
    
    // Find conditional text [if condition]text[/if]
    let if_pattern = regex::Regex::new(r"\[if\s+([^\]]+)\](.*?)\[/if\]").unwrap();
    
    for cap in if_pattern.captures_iter(text) {
        if let (Some(condition), Some(content)) = (cap.get(1), cap.get(2)) {
            if evaluate_condition(condition.as_str(), scope) {
                result = result.replace(&cap[0], content.as_str());
            } else {
                result = result.replace(&cap[0], "");
            }
        }
    }
    
    result
}

/// Format a value for display
fn format_value(value: &Value) -> String {
    match value {
        Value::Bool(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::List(items) => {
            let formatted: Vec<String> = items.iter()
                .map(format_value)
                .collect();
            formatted.join(", ")
        }
        Value::Map(_) => "[object]".to_string(),
        Value::Null => "".to_string(),
    }
}

/// Evaluate a simple condition
fn evaluate_condition(condition: &str, scope: &Scope) -> bool {
    // Simple variable check
    if let Some(value) = scope.get(condition) {
        match value {
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            _ => true,
        }
    } else {
        false
    }
}

/// Math operations on values
pub mod math {
    use crate::types::Value;
    use crate::error::{Error, Result};
    
    /// Add two values
    pub fn add(a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x + *y as f64)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            _ => Err(Error::RuntimeError("Cannot add these types".to_string())),
        }
    }
    
    /// Subtract two values
    pub fn subtract(a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x - y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 - y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x - *y as f64)),
            _ => Err(Error::RuntimeError("Cannot subtract these types".to_string())),
        }
    }
    
    /// Multiply two values
    pub fn multiply(a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x * y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 * y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x * (*y as f64))),
            _ => Err(Error::RuntimeError("Cannot multiply these types".to_string())),
        }
    }
    
    /// Divide two values
    pub fn divide(a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (_, Value::Integer(0)) => {
                Err(Error::RuntimeError("Division by zero".to_string()))
            }
            (_, Value::Float(y)) if *y == 0.0 => {
                Err(Error::RuntimeError("Division by zero".to_string()))
            }
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Float(*x as f64 / *y as f64)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x / y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 / y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x / (*y as f64))),
            _ => Err(Error::RuntimeError("Cannot divide these types".to_string())),
        }
    }
    
    /// Modulo operation
    pub fn modulo(a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) if *y != 0 => Ok(Value::Integer(x % y)),
            _ => Err(Error::RuntimeError("Modulo requires two non-zero integers".to_string())),
        }
    }
}

/// Comparison operations
pub mod comparison {
    use crate::types::Value;
    
    /// Check equality
    pub fn equals(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => (x - y).abs() < f64::EPSILON,
            (Value::Integer(x), Value::Float(y)) => (*x as f64 - y).abs() < f64::EPSILON,
            (Value::Float(x), Value::Integer(y)) => (x - *y as f64).abs() < f64::EPSILON,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    
    /// Less than comparison
    pub fn less_than(a: &Value, b: &Value) -> Option<bool> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Some(x < y),
            (Value::Float(x), Value::Float(y)) => Some(x < y),
            (Value::Integer(x), Value::Float(y)) => Some((*x as f64) < *y),
            (Value::Float(x), Value::Integer(y)) => Some(*x < (*y as f64)),
            (Value::String(x), Value::String(y)) => Some(x < y),
            _ => None,
        }
    }
    
    /// Greater than comparison
    pub fn greater_than(a: &Value, b: &Value) -> Option<bool> {
        less_than(b, a)
    }
    
    /// Less than or equal
    pub fn less_equal(a: &Value, b: &Value) -> Option<bool> {
        match (less_than(a, b), equals(a, b)) {
            (Some(true), _) => Some(true),
            (Some(false), true) => Some(true),
            (Some(false), false) => Some(false),
            _ => None,
        }
    }
    
    /// Greater than or equal
    pub fn greater_equal(a: &Value, b: &Value) -> Option<bool> {
        less_equal(b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scope() {
        let mut scope = Scope::new();
        scope.set("x".to_string(), Value::Integer(42));
        
        assert_eq!(scope.get("x"), Some(&Value::Integer(42)));
        assert_eq!(scope.get("y"), None);
        
        // Test child scope
        let mut child = Scope::child(scope);
        assert_eq!(child.get("x"), Some(&Value::Integer(42)));
        
        child.set("y".to_string(), Value::String("hello".to_string()));
        assert_eq!(child.get("y"), Some(&Value::String("hello".to_string())));
    }
    
    #[test]
    fn test_interpolation() {
        let mut scope = Scope::new();
        scope.set("name".to_string(), Value::String("Alice".to_string()));
        scope.set("score".to_string(), Value::Integer(100));
        
        let text = "Hello [name], your score is [score]!";
        let result = interpolate_text(text, &scope);
        assert_eq!(result, "Hello Alice, your score is 100!");
    }
    
    #[test]
    fn test_math_operations() {
        use math::*;
        
        let a = Value::Integer(10);
        let b = Value::Integer(3);
        
        assert_eq!(add(&a, &b).unwrap(), Value::Integer(13));
        assert_eq!(subtract(&a, &b).unwrap(), Value::Integer(7));
        assert_eq!(multiply(&a, &b).unwrap(), Value::Integer(30));
        
        match divide(&a, &b).unwrap() {
            Value::Float(f) => assert!((f - 3.333333).abs() < 0.001),
            _ => panic!("Expected float"),
        }
    }
}