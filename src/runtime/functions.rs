//! Built-in functions for PlotScript

use rand::Rng;
use crate::error::{Error, Result};
use crate::types::Value;
use crate::world::World;

/// Math: min(a, b)
pub fn builtin_min(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 2 {
        return Err(Error::RuntimeError("min() expects 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(*a.min(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).min(*b))),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.min(*b as f64))),
        _ => Err(Error::RuntimeError("min() requires numeric arguments".to_string())),
    }
}

/// Math: max(a, b)
pub fn builtin_max(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 2 {
        return Err(Error::RuntimeError("max() expects 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(*a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).max(*b))),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.max(*b as f64))),
        _ => Err(Error::RuntimeError("max() requires numeric arguments".to_string())),
    }
}

/// Math: abs(n)
pub fn builtin_abs(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("abs() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        _ => Err(Error::RuntimeError("abs() requires a numeric argument".to_string())),
    }
}

/// Math: round(n)
pub fn builtin_round(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("round() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::Integer(n) => Ok(Value::Integer(*n)),
        Value::Float(f) => Ok(Value::Integer(f.round() as i64)),
        _ => Err(Error::RuntimeError("round() requires a numeric argument".to_string())),
    }
}

/// Math: random(min, max)
pub fn builtin_random(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 2 {
        return Err(Error::RuntimeError("random() expects 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(min), Value::Integer(max)) => {
            if min > max {
                return Err(Error::RuntimeError("random() min must be <= max".to_string()));
            }
            let mut rng = rand::thread_rng();
            Ok(Value::Integer(rng.gen_range(*min..=*max)))
        }
        _ => Err(Error::RuntimeError("random() requires integer arguments".to_string())),
    }
}

/// String: len(s)
pub fn builtin_len(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("len() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
        Value::List(l) => Ok(Value::Integer(l.len() as i64)),
        Value::Map(m) => Ok(Value::Integer(m.len() as i64)),
        _ => Err(Error::RuntimeError("len() requires a string, list, or map".to_string())),
    }
}

/// String: upper(s)
pub fn builtin_upper(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("upper() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(Error::RuntimeError("upper() requires a string argument".to_string())),
    }
}

/// String: lower(s)
pub fn builtin_lower(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("lower() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(Error::RuntimeError("lower() requires a string argument".to_string())),
    }
}

/// String: contains(haystack, needle)
pub fn builtin_contains(args: Vec<Value>, _world: &mut World) -> Result<Value> {
    if args.len() != 2 {
        return Err(Error::RuntimeError("contains() expects 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(haystack), Value::String(needle)) => {
            Ok(Value::Bool(haystack.contains(needle)))
        }
        (Value::List(list), item) => {
            Ok(Value::Bool(list.iter().any(|v| v == item)))
        }
        _ => Err(Error::RuntimeError("contains() requires string or list arguments".to_string())),
    }
}

/// Game: has_item(item_id)
pub fn builtin_has_item(args: Vec<Value>, world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("has_item() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(item_id) => {
            Ok(Value::Bool(world.inventory.contains(item_id)))
        }
        _ => Err(Error::RuntimeError("has_item() requires a string argument".to_string())),
    }
}

/// Game: has_flag(flag_name)
pub fn builtin_has_flag(args: Vec<Value>, world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("has_flag() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(flag_name) => {
            Ok(Value::Bool(world.get_flag(flag_name)))
        }
        _ => Err(Error::RuntimeError("has_flag() requires a string argument".to_string())),
    }
}

/// Game: visited(room_id)
pub fn builtin_visited(args: Vec<Value>, world: &mut World) -> Result<Value> {
    if args.len() != 1 {
        return Err(Error::RuntimeError("visited() expects 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::String(room_id) => {
            let visited = world.rooms.get(room_id)
                .map(|room| room.visited)
                .unwrap_or(false);
            Ok(Value::Bool(visited))
        }
        _ => Err(Error::RuntimeError("visited() requires a string argument".to_string())),
    }
}

/// Game: score()
pub fn builtin_score(args: Vec<Value>, world: &mut World) -> Result<Value> {
    if !args.is_empty() {
        return Err(Error::RuntimeError("score() expects no arguments".to_string()));
    }
    
    Ok(Value::Integer(world.score as i64))
}

/// Game: turns()
pub fn builtin_turns(args: Vec<Value>, world: &mut World) -> Result<Value> {
    if !args.is_empty() {
        return Err(Error::RuntimeError("turns() expects no arguments".to_string()));
    }
    
    Ok(Value::Integer(world.turn_count as i64))
}