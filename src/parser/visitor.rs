//! Visitor pattern for AST traversal

use crate::parser::ast::*;

/// Visitor trait for traversing the AST
pub trait Visitor {
    /// Visit a script
    fn visit_script(&mut self, script: &Script) {
        if let Some(game) = &script.game {
            self.visit_game_definition(game);
        }
        
        for room in script.rooms.values() {
            self.visit_room(room);
        }
        
        for character in script.characters.values() {
            self.visit_character(character);
        }
        
        for item in script.items.values() {
            self.visit_item(item);
        }
        
        for event in &script.events {
            self.visit_event(event);
        }
        
        for function in script.functions.values() {
            self.visit_function(function);
        }
    }
    
    /// Visit game definition
    fn visit_game_definition(&mut self, _game: &GameDefinition) {}
    
    /// Visit a room
    fn visit_room(&mut self, room: &Room) {
        for stmt in &room.on_enter {
            self.visit_statement(stmt);
        }
        
        for stmt in &room.on_exit {
            self.visit_statement(stmt);
        }
    }
    
    /// Visit a character
    fn visit_character(&mut self, character: &Character) {
        self.visit_dialogue_tree(&character.dialogue);
    }
    
    /// Visit dialogue tree
    fn visit_dialogue_tree(&mut self, tree: &DialogueTree) {
        for node in &tree.nodes {
            self.visit_dialogue_node(node);
        }
    }
    
    /// Visit dialogue node
    fn visit_dialogue_node(&mut self, node: &DialogueNode) {
        for response in &node.responses {
            self.visit_dialogue_response(response);
        }
    }
    
    /// Visit dialogue response
    fn visit_dialogue_response(&mut self, response: &DialogueResponse) {
        for stmt in &response.actions {
            self.visit_statement(stmt);
        }
    }
    
    /// Visit an item
    fn visit_item(&mut self, item: &Item) {
        for stmt in &item.on_take {
            self.visit_statement(stmt);
        }
        
        for stmt in &item.on_use {
            self.visit_statement(stmt);
        }
        
        for stmt in &item.on_examine {
            self.visit_statement(stmt);
        }
    }
    
    /// Visit an event
    fn visit_event(&mut self, event: &Event) {
        for stmt in &event.actions {
            self.visit_statement(stmt);
        }
    }
    
    /// Visit a function
    fn visit_function(&mut self, function: &Function) {
        for stmt in &function.body {
            self.visit_statement(stmt);
        }
    }
    
    /// Visit a statement
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assignment { value, .. } => {
                self.visit_expression(value);
            }
            Statement::If { condition, then_block, else_block } => {
                self.visit_condition(condition);
                for stmt in then_block {
                    self.visit_statement(stmt);
                }
                if let Some(else_block) = else_block {
                    for stmt in else_block {
                        self.visit_statement(stmt);
                    }
                }
            }
            Statement::While { condition, body } => {
                self.visit_condition(condition);
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::For { collection, body, .. } => {
                self.visit_expression(collection);
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::Call { arguments, .. } => {
                for arg in arguments {
                    self.visit_expression(arg);
                }
            }
            Statement::Print(expr) => {
                self.visit_expression(expr);
            }
            Statement::Return(Some(expr)) => {
                self.visit_expression(expr);
            }
            _ => {}
        }
    }
    
    /// Visit a condition
    fn visit_condition(&mut self, condition: &Condition) {
        self.visit_expression(&condition.left);
        self.visit_expression(&condition.right);
    }
    
    /// Visit an expression
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Binary { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::Unary { operand, .. } => {
                self.visit_expression(operand);
            }
            Expression::Call { arguments, .. } => {
                for arg in arguments {
                    self.visit_expression(arg);
                }
            }
            Expression::List(exprs) => {
                for expr in exprs {
                    self.visit_expression(expr);
                }
            }
            Expression::Map(entries) => {
                for (_, expr) in entries {
                    self.visit_expression(expr);
                }
            }
            _ => {}
        }
    }
}

/// Mutable visitor trait
pub trait VisitorMut {
    /// Visit and potentially modify a script
    fn visit_script_mut(&mut self, script: &mut Script) {
        if let Some(game) = &mut script.game {
            self.visit_game_definition_mut(game);
        }
        
        for room in script.rooms.values_mut() {
            self.visit_room_mut(room);
        }
        
        for character in script.characters.values_mut() {
            self.visit_character_mut(character);
        }
        
        for item in script.items.values_mut() {
            self.visit_item_mut(item);
        }
        
        for event in &mut script.events {
            self.visit_event_mut(event);
        }
        
        for function in script.functions.values_mut() {
            self.visit_function_mut(function);
        }
    }
    
    /// Visit and modify game definition
    fn visit_game_definition_mut(&mut self, _game: &mut GameDefinition) {}
    
    /// Visit and modify a room
    fn visit_room_mut(&mut self, room: &mut Room) {
        for stmt in &mut room.on_enter {
            self.visit_statement_mut(stmt);
        }
        
        for stmt in &mut room.on_exit {
            self.visit_statement_mut(stmt);
        }
    }
    
    /// Visit and modify a character
    fn visit_character_mut(&mut self, character: &mut Character) {
        self.visit_dialogue_tree_mut(&mut character.dialogue);
    }
    
    /// Visit and modify dialogue tree
    fn visit_dialogue_tree_mut(&mut self, tree: &mut DialogueTree) {
        for node in &mut tree.nodes {
            self.visit_dialogue_node_mut(node);
        }
    }
    
    /// Visit and modify dialogue node
    fn visit_dialogue_node_mut(&mut self, node: &mut DialogueNode) {
        for response in &mut node.responses {
            self.visit_dialogue_response_mut(response);
        }
    }
    
    /// Visit and modify dialogue response
    fn visit_dialogue_response_mut(&mut self, response: &mut DialogueResponse) {
        for stmt in &mut response.actions {
            self.visit_statement_mut(stmt);
        }
    }
    
    /// Visit and modify an item
    fn visit_item_mut(&mut self, item: &mut Item) {
        for stmt in &mut item.on_take {
            self.visit_statement_mut(stmt);
        }
        
        for stmt in &mut item.on_use {
            self.visit_statement_mut(stmt);
        }
        
        for stmt in &mut item.on_examine {
            self.visit_statement_mut(stmt);
        }
    }
    
    /// Visit and modify an event
    fn visit_event_mut(&mut self, event: &mut Event) {
        for stmt in &mut event.actions {
            self.visit_statement_mut(stmt);
        }
    }
    
    /// Visit and modify a function
    fn visit_function_mut(&mut self, function: &mut Function) {
        for stmt in &mut function.body {
            self.visit_statement_mut(stmt);
        }
    }
    
    /// Visit and modify a statement
    fn visit_statement_mut(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::Assignment { value, .. } => {
                self.visit_expression_mut(value);
            }
            Statement::If { condition, then_block, else_block } => {
                self.visit_condition_mut(condition);
                for stmt in then_block {
                    self.visit_statement_mut(stmt);
                }
                if let Some(else_block) = else_block {
                    for stmt in else_block {
                        self.visit_statement_mut(stmt);
                    }
                }
            }
            Statement::While { condition, body } => {
                self.visit_condition_mut(condition);
                for stmt in body {
                    self.visit_statement_mut(stmt);
                }
            }
            Statement::For { collection, body, .. } => {
                self.visit_expression_mut(collection);
                for stmt in body {
                    self.visit_statement_mut(stmt);
                }
            }
            Statement::Call { arguments, .. } => {
                for arg in arguments {
                    self.visit_expression_mut(arg);
                }
            }
            Statement::Print(expr) => {
                self.visit_expression_mut(expr);
            }
            Statement::Return(Some(expr)) => {
                self.visit_expression_mut(expr);
            }
            _ => {}
        }
    }
    
    /// Visit and modify a condition
    fn visit_condition_mut(&mut self, condition: &mut Condition) {
        self.visit_expression_mut(&mut condition.left);
        self.visit_expression_mut(&mut condition.right);
    }
    
    /// Visit and modify an expression
    fn visit_expression_mut(&mut self, expr: &mut Expression) {
        match expr {
            Expression::Binary { left, right, .. } => {
                self.visit_expression_mut(left);
                self.visit_expression_mut(right);
            }
            Expression::Unary { operand, .. } => {
                self.visit_expression_mut(operand);
            }
            Expression::Call { arguments, .. } => {
                for arg in arguments {
                    self.visit_expression_mut(arg);
                }
            }
            Expression::List(exprs) => {
                for expr in exprs {
                    self.visit_expression_mut(expr);
                }
            }
            Expression::Map(entries) => {
                for (_, expr) in entries {
                    self.visit_expression_mut(expr);
                }
            }
            _ => {}
        }
    }
}