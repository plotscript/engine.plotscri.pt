//! Parser module for PlotScript language

use pest::Parser;
use pest_derive::Parser;
use crate::error::{Error, Result};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct PlotScriptParser;

mod ast;
mod visitor;
mod fuzzy;
mod command_parser;

pub use ast::*;
pub use visitor::*;
pub use fuzzy::*;
pub use command_parser::{CommandParser, ParsedCommand};

/// Parse a PlotScript source file
pub fn parse_script(source: &str) -> Result<Script> {
    let pairs = PlotScriptParser::parse(Rule::file, source)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    
    let mut script = Script::default();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::game_definition => {
                script.game = Some(parse_game_definition(pair)?);
            }
            Rule::room => {
                let room = parse_room(pair)?;
                script.rooms.insert(room.id.clone(), room);
            }
            Rule::character => {
                let character = parse_character(pair)?;
                script.characters.insert(character.id.clone(), character);
            }
            Rule::item => {
                let item = parse_item(pair)?;
                script.items.insert(item.id.clone(), item);
            }
            Rule::event => {
                let event = parse_event(pair)?;
                script.events.push(event);
            }
            Rule::function => {
                let function = parse_function(pair)?;
                script.functions.insert(function.name.clone(), function);
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    
    Ok(script)
}

fn parse_game_definition(pair: pest::iterators::Pair<Rule>) -> Result<GameDefinition> {
    let mut inner = pair.into_inner();
    let title = parse_string(inner.next().unwrap())?;
    
    let mut game = GameDefinition {
        title,
        ..Default::default()
    };
    
    for property in inner {
        let mut prop_inner = property.into_inner();
        match prop_inner.next().unwrap().as_str() {
            "author" => {
                game.author = Some(parse_string(prop_inner.next().unwrap())?);
            }
            "version" => {
                game.version = Some(parse_string(prop_inner.next().unwrap())?);
            }
            "mode" => {
                game.mode = parse_game_mode(prop_inner.next().unwrap())?;
            }
            "description" => {
                game.description = Some(parse_string(prop_inner.next().unwrap())?);
            }
            _ => {}
        }
    }
    
    Ok(game)
}

fn parse_room(pair: pest::iterators::Pair<Rule>) -> Result<Room> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    
    let mut room = Room {
        id: id.clone(),
        ..Default::default()
    };
    
    // Optional title
    if let Some(next) = inner.peek() {
        if next.as_rule() == Rule::string {
            room.title = Some(parse_string(inner.next().unwrap())?);
        }
    }
    
    // Properties
    for property in inner {
        match property.as_rule() {
            Rule::room_property => {
                parse_room_property(&mut room, property)?;
            }
            _ => {}
        }
    }
    
    Ok(room)
}

fn parse_room_property(room: &mut Room, pair: pest::iterators::Pair<Rule>) -> Result<()> {
    let mut inner = pair.into_inner();
    let prop_name = inner.next().unwrap().as_str();
    
    match prop_name {
        "description" => {
            room.description = Some(parse_string(inner.next().unwrap())?);
        }
        "exits" => {
            room.exits = parse_exits(inner.next().unwrap())?;
        }
        "items" => {
            room.items = parse_identifier_list(inner.next().unwrap())?;
        }
        "dark" => {
            room.dark = parse_boolean(inner.next().unwrap())?;
        }
        _ => {}
    }
    
    Ok(())
}

fn parse_string(pair: pest::iterators::Pair<Rule>) -> Result<String> {
    let s = pair.as_str();
    // Remove quotes
    Ok(s[1..s.len()-1].to_string())
}

fn parse_boolean(pair: pest::iterators::Pair<Rule>) -> Result<bool> {
    Ok(pair.as_str() == "true")
}

fn parse_game_mode(pair: pest::iterators::Pair<Rule>) -> Result<crate::types::GameMode> {
    match pair.as_str() {
        "text_adventure" => Ok(crate::types::GameMode::TextAdventure),
        "visual_novel" => Ok(crate::types::GameMode::VisualNovel),
        "interactive_fiction" => Ok(crate::types::GameMode::InteractiveFiction),
        _ => Err(Error::ParseError(format!("Unknown game mode: {}", pair.as_str()))),
    }
}

fn parse_exits(pair: pest::iterators::Pair<Rule>) -> Result<std::collections::HashMap<crate::types::Direction, String>> {
    let mut exits = std::collections::HashMap::new();
    
    for exit in pair.into_inner() {
        let mut inner = exit.into_inner();
        let direction = parse_direction(inner.next().unwrap())?;
        let target = inner.next().unwrap().as_str().to_string();
        exits.insert(direction, target);
    }
    
    Ok(exits)
}

fn parse_direction(pair: pest::iterators::Pair<Rule>) -> Result<crate::types::Direction> {
    crate::types::Direction::from_str(pair.as_str())
        .ok_or_else(|| Error::ParseError(format!("Unknown direction: {}", pair.as_str())))
}

fn parse_identifier_list(pair: pest::iterators::Pair<Rule>) -> Result<Vec<String>> {
    Ok(pair.into_inner()
        .map(|p| p.as_str().to_string())
        .collect())
}

fn parse_character(pair: pest::iterators::Pair<Rule>) -> Result<Character> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    
    let mut character = Character {
        id: id.clone(),
        ..Default::default()
    };
    
    // Optional name
    if let Some(next) = inner.peek() {
        if next.as_rule() == Rule::string {
            character.name = Some(parse_string(inner.next().unwrap())?);
        }
    }
    
    Ok(character)
}

fn parse_item(pair: pest::iterators::Pair<Rule>) -> Result<Item> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    
    let mut item = Item {
        id: id.clone(),
        ..Default::default()
    };
    
    // Optional name
    if let Some(next) = inner.peek() {
        if next.as_rule() == Rule::string {
            item.name = Some(parse_string(inner.next().unwrap())?);
        }
    }
    
    Ok(item)
}

fn parse_event(pair: pest::iterators::Pair<Rule>) -> Result<Event> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    
    Ok(Event {
        id,
        trigger: Trigger::Always,
        actions: Vec::new(),
    })
}

fn parse_function(pair: pest::iterators::Pair<Rule>) -> Result<Function> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    
    Ok(Function {
        name,
        parameters: Vec::new(),
        body: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_game() {
        let script = r#"
            game "Test Adventure" {
                author: "Test Author"
                mode: text_adventure
            }
            
            room kitchen "Kitchen" {
                description: "A cozy kitchen."
                exits: {
                    north: hallway
                }
            }
        "#;
        
        let result = parse_script(script);
        assert!(result.is_ok());
        
        let script = result.unwrap();
        assert_eq!(script.game.as_ref().unwrap().title, "Test Adventure");
        assert_eq!(script.rooms.len(), 1);
        assert!(script.rooms.contains_key("kitchen"));
    }
}