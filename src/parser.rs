use std::str::SplitWhitespace;

use strum::EnumIter;

use crate::{parser_utils::{get_type_from_key, parse_raw_value}, store::{Key, Value}};

pub enum Command {
    Set(Key, Value),
    Get(Key),
    Del(Key),
    Help,
    Unknown(String),
    Empty,
}

#[derive(Debug, PartialEq, EnumIter)]
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

/* ------------------------------------------------------ */
/*                           SET                          */
/* ------------------------------------------------------ */
fn parse_set_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?;
    let r#type = get_type_from_key(key);
    let raw_value = parts.next().ok_or("No value provided")?;
    let parsed_value = parse_raw_value(raw_value, r#type)?;

    Ok(Command::Set(key.to_string(), parsed_value))
}

/* ------------------------------------------------------ */
/*                           GET                          */
/* ------------------------------------------------------ */
fn parse_get_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?.to_string();
    Ok(Command::Get(key))
}

/* ------------------------------------------------------ */
/*                           DEL                          */
/* ------------------------------------------------------ */
fn parse_del_command(parts: &mut SplitWhitespace<'_>) -> Result<Command, String> {
    let key = parts.next().ok_or("No key provided")?.to_string();
    Ok(Command::Del(key))
}

/* ------------------------------------------------------ */
/*                          HELP                          */
/* ------------------------------------------------------ */
fn parse_help_command() -> Result<Command, String> {
    Ok(Command::Help)
}

/* ------------------------------------------------------ */
/*                     Unknown Command                    */
/* ------------------------------------------------------ */
fn parse_unknown_command(cmd: &str) -> Result<Command, String> {
    Ok(Command::Unknown(cmd.to_string()))
}

/* ------------------------------------------------------ */
/*                          Tests                         */
/* ------------------------------------------------------ */
#[cfg(test)]
mod tests {
    use super::*;

    /* ------------------------------------------------------ */
    /*                       parse_input                      */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_input_of_set_command() {
        let input = "set str-x y".to_string();
        match parse_input(input) {
            Ok(Command::Set(..)) => (),
            _ => panic!("Expected Command::Set"),
        }
    }

    #[test]
    fn test_parse_input_of_get_command() {
        let input = "get x".to_string();
        match parse_input(input) {
            Ok(Command::Get(..)) => (),
            _ => panic!("Expected Command::Get"),
        }
    }

    #[test]
    fn test_parse_input_of_del_command() {
        let input = "del x".to_string();
        match parse_input(input) {
            Ok(Command::Del(..)) => (),
            _ => panic!("Expected Command::Del"),
        }
    }

    #[test]
    fn test_parse_input_of_help_command() {
        let input = "help".to_string();
        match parse_input(input) {
            Ok(Command::Help) => (),
            _ => panic!("Expected Command::Help"),
        }
    }

    #[test]
    fn test_parse_input_of_unknown_command() {
        let input = "not-a-command".to_string();
        match parse_input(input) {
            Ok(Command::Unknown(..)) => (),
            _ => panic!("Expected Command::Unknown"),
        }
    }

    #[test]
    fn test_parse_input_of_empty_command() {
        let input = "".to_string();
        match parse_input(input) {
            Ok(Command::Empty) => (),
            _ => panic!("Expected Command::Empty"),
        }
    }

    /* ------------------------------------------------------ */
    /*                    parse_set_command                   */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_set_command_with_key_and_value() {
        let mut parts = "set x y".split_whitespace();
        parts.next(); // Skip the command
        match parse_set_command(&mut parts) {
            Ok(Command::Set(key, value)) => {
                assert_eq!(key, "x");
                assert_eq!(value, Value::Str("y".to_string()));
            }
            _ => panic!("Expected Command::Set"),
        }
    }

    #[test]
    fn test_parse_set_command_without_key_or_value() {
        let mut parts = "set".split_whitespace();
        parts.next(); // Skip the command
        match parse_set_command(&mut parts) {
            Err(e) => assert_eq!(e, "No key provided"),
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_parse_set_command_with_key_without_value() {
        let mut parts = "set x".split_whitespace();
        parts.next(); // Skip the command
        match parse_set_command(&mut parts) {
            Err(e) => assert_eq!(e, "No value provided"),
            _ => panic!("Expected an error"),
        }
    }

    /* ------------------------------------------------------ */
    /*                    parse_get_command                   */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_get_command_with_key() {
        let mut parts = "get x".split_whitespace();
        parts.next(); // Skip the command=
        match parse_get_command(&mut parts) {
            Ok(Command::Get(key)) => assert_eq!(key, "x"),
            _ => panic!("Expected Command::Get"),
        }
    }

    #[test]
    fn test_parse_get_command_without_key() {
        let mut parts = "get".split_whitespace();
        parts.next(); // Skip the command
        match parse_get_command(&mut parts) {
            Err(e) => assert_eq!(e, "No key provided"),
            _ => panic!("Expected an error"),
        }
    }

    /* ------------------------------------------------------ */
    /*                    prase_del_command                   */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_del_command_with_key() {
        let mut parts = "del x".split_whitespace();
        parts.next(); // Skip the command
        match parse_del_command(&mut parts) {
            Ok(Command::Del(key)) => assert_eq!(key, "x"),
            _ => panic!("Expected Command::Del"),
        }
    }

    #[test]
    fn test_parse_del_command_without_key() {
        let mut parts = "del".split_whitespace();
        parts.next(); // Skip the command
        match parse_del_command(&mut parts) {
            Err(e) => assert_eq!(e, "No key provided"),
            _ => panic!("Expected an error"),
        }
    }

    /* ------------------------------------------------------ */
    /*                   parse_help_command                   */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_help_command() {
        match parse_help_command() {
            Ok(Command::Help) => (),
            _ => panic!("Expected Command::Help"),
        }
    }

    /* ------------------------------------------------------ */
    /*                  parse_unknown_command                 */
    /* ------------------------------------------------------ */
    #[test]
    fn test_parse_unknown_command() {
        let cmd = "not-a-command";
        match parse_unknown_command(cmd) {
            Ok(Command::Unknown(unknown_cmd)) => assert_eq!(unknown_cmd, cmd),
            _ => panic!("Expected Command::Unknown"),
        }
    }
}
