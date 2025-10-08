//! Expression evaluation for the runtime

use crate::error::{Error, Result};
use crate::parser::{Expression, Condition, BinaryOp, UnaryOp, ComparisonOp};
use crate::runtime::Runtime;
use crate::scripting::{math, comparison};
use crate::types::Value;
use crate::world::World;

impl Runtime {
    /// Evaluate an expression
    pub fn evaluate_expression(&mut self, expr: &Expression, world: &mut World) -> Result<Value> {
        match expr {
            Expression::Literal(value) => Ok(value.clone()),
            
            Expression::Variable(name) => {
                world.get_variable(name)
                    .cloned()
                    .ok_or_else(|| Error::RuntimeError(format!("Undefined variable: {}", name)))
            }
            
            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(left, world)?;
                let right_val = self.evaluate_expression(right, world)?;
                self.evaluate_binary_op(&left_val, *op, &right_val)
            }
            
            Expression::Unary { op, operand } => {
                let val = self.evaluate_expression(operand, world)?;
                self.evaluate_unary_op(*op, &val)
            }
            
            Expression::Call { function, arguments } => {
                let args: Vec<Value> = arguments.iter()
                    .map(|arg| self.evaluate_expression(arg, world))
                    .collect::<Result<Vec<_>>>()?;
                
                self.call_function(function, args, world)
            }
            
            Expression::List(items) => {
                let values: Vec<Value> = items.iter()
                    .map(|item| self.evaluate_expression(item, world))
                    .collect::<Result<Vec<_>>>()?;
                
                Ok(Value::List(values))
            }
            
            Expression::Map(entries) => {
                let mut map = std::collections::HashMap::new();
                
                for (key, expr) in entries {
                    let value = self.evaluate_expression(expr, world)?;
                    map.insert(key.clone(), value);
                }
                
                Ok(Value::Map(map))
            }
        }
    }
    
    /// Evaluate a condition
    pub fn evaluate_condition(&mut self, condition: &Condition, world: &mut World) -> Result<bool> {
        let left_val = self.evaluate_expression(&condition.left, world)?;
        let right_val = self.evaluate_expression(&condition.right, world)?;
        
        match condition.operator {
            ComparisonOp::Eq => Ok(comparison::equals(&left_val, &right_val)),
            ComparisonOp::Ne => Ok(!comparison::equals(&left_val, &right_val)),
            ComparisonOp::Lt => {
                comparison::less_than(&left_val, &right_val)
                    .ok_or_else(|| Error::RuntimeError("Cannot compare these types".to_string()))
            }
            ComparisonOp::Le => {
                comparison::less_equal(&left_val, &right_val)
                    .ok_or_else(|| Error::RuntimeError("Cannot compare these types".to_string()))
            }
            ComparisonOp::Gt => {
                comparison::greater_than(&left_val, &right_val)
                    .ok_or_else(|| Error::RuntimeError("Cannot compare these types".to_string()))
            }
            ComparisonOp::Ge => {
                comparison::greater_equal(&left_val, &right_val)
                    .ok_or_else(|| Error::RuntimeError("Cannot compare these types".to_string()))
            }
        }
    }
    
    /// Evaluate a binary operation
    fn evaluate_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value> {
        match op {
            BinaryOp::Add => math::add(left, right),
            BinaryOp::Sub => math::subtract(left, right),
            BinaryOp::Mul => math::multiply(left, right),
            BinaryOp::Div => math::divide(left, right),
            BinaryOp::Mod => math::modulo(left, right),
            BinaryOp::And => self.evaluate_logical_and(left, right),
            BinaryOp::Or => self.evaluate_logical_or(left, right),
        }
    }
    
    /// Evaluate a unary operation
    fn evaluate_unary_op(&self, op: UnaryOp, operand: &Value) -> Result<Value> {
        match op {
            UnaryOp::Not => Ok(Value::Bool(!self.is_truthy(operand))),
            UnaryOp::Neg => match operand {
                Value::Integer(n) => Ok(Value::Integer(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(Error::RuntimeError("Cannot negate non-numeric value".to_string())),
            },
        }
    }
    
    /// Evaluate logical AND
    fn evaluate_logical_and(&self, left: &Value, right: &Value) -> Result<Value> {
        Ok(Value::Bool(self.is_truthy(left) && self.is_truthy(right)))
    }
    
    /// Evaluate logical OR
    fn evaluate_logical_or(&self, left: &Value, right: &Value) -> Result<Value> {
        Ok(Value::Bool(self.is_truthy(left) || self.is_truthy(right)))
    }
    
    /// Check if a value is truthy
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Null => false,
        }
    }
}