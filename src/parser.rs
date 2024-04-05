use crate::store::{Key, Value};

pub enum Command {
    Set(Key, Value),
    Get(Key),
    Del(Key),
    Exit,
    Help,
    Unknown(String),
    Empty,
}

pub fn parse_input(input: String) -> Result<Command, String> {
    let input = input.trim();
    let mut parts = input.split_whitespace();

    let command_case_insensitive = parts.next().map(|s| s.to_lowercase());

    match command_case_insensitive.as_deref() {
        Some("set") => {
            let value_type = parts.next().ok_or("No value type provided")?.to_lowercase();

            let key = parts.next().ok_or("No key provided")?;
            let value = parts.next().ok_or("No value provided")?;

            let typed_value = match value_type.as_str() {
                "str" => Value::Str(value.to_string()),
                "int" => value.parse::<i64>()
                             .map(Value::Int)
                             .map_err(|_| "Invalid integer format for value")?,
                "float" => value.parse::<f64>()
                                .map(Value::Float)
                                .map_err(|_| "Invalid float format for value")?,
                "bool" => match value.to_lowercase().as_str() { // Also handle boolean value case-insensitively
                    "true" => Value::Bool(true),
                    "false" => Value::Bool(false),
                    _ => return Err("Invalid boolean value; expected true or false".into()),
                },
                _ => return Err(format!("Invalid value type: {}", value_type)),
            };

            Ok(Command::Set(key.to_string(), typed_value))
        }
        
        Some("get") => {
            let key = parts.next().ok_or("No key provided")?.to_string();
            Ok(Command::Get(key))
        }
        
        Some("del") => {
            let key = parts.next().ok_or("No key provided")?.to_string();
            Ok(Command::Del(key))
        }
        
        Some("exit") | Some("quit") => Ok(Command::Exit),
        
        Some("help") => Ok(Command::Help),
        
        Some(cmd) => Ok(Command::Unknown(cmd.to_string())),
        
        None => Ok(Command::Empty),
    }
}
