use std::str::SplitWhitespace;

use crate::store::{Key, Value};

pub enum Command {
    Set(Key, Value),
    Get(Key),
    Del(Key),
    Help,
    Unknown(String),
    Empty,
}

#[derive(Debug)]
pub enum ValueType {
    Str,
    Int,
    Float,
    Bool,
}

pub fn parse_input(input: String) -> Result<Command, String> {
    let mut parts = input.trim().split_whitespace();

    let command_case_insensitive = parts.next().map(|s| s.to_lowercase());

    match command_case_insensitive.as_deref() {
        Some("set") => parse_set_command(&mut parts),
        Some("get") => parse_get_command(&mut parts),
        Some("del") => parse_del_command(&mut parts),
        Some("help") => parse_help_command(),
        Some(cmd) => parse_unknown_command(cmd),
        None => Ok(Command::Empty),
    }
}

fn parse_set_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?;
    let value_type = match key.split('-').next() {
        Some(t) => match t.to_lowercase().as_str() {
            "str" => ValueType::Str,
            "int" => ValueType::Int,
            "float" => ValueType::Float,
            "bool" => ValueType::Bool,
            _ => ValueType::Str,
        },
        None => ValueType::Str,
    };
    let value = parts.next().ok_or("No value provided")?;

    let typed_value = match value_type {
        ValueType::Str => Value::Str(value.to_string()),
        ValueType::Int => value
            .parse::<i64>()
            .map(Value::Int)
            .map_err(|_| format!("{value} cannot be parsed as an integer"))?,
        ValueType::Float => value
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("{value} cannot be parsed as a float"))?,
        ValueType::Bool => match value.to_lowercase().as_str() {
            // Also handle boolean value case-insensitively
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => return Err("Invalid boolean value; expected true or false".into()),
        },
        _ => return Err(format!("Invalid value type: {:?}", value_type)),
    };

    Ok(Command::Set(key.to_string(), typed_value))
}

fn parse_get_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?.to_string();
    Ok(Command::Get(key))
}

fn parse_del_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?.to_string();
    Ok(Command::Del(key))
}

fn parse_help_command() -> Result<Command, String> {
    Ok(Command::Help)
}

fn parse_unknown_command(cmd: &str) -> Result<Command, String> {
    Ok(Command::Unknown(cmd.to_string()))
}