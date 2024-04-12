use strum::EnumIter;

use crate::commands::{
    del_command::DelCommand, get_command::GetCommand, help_command::HelpCommand,
    set_command::SetCommand, Command, CommandTrait,
};

pub mod utils;

#[derive(Debug, PartialEq, EnumIter)]
pub enum ValueType {
    Str,
    Int,
    Float,
    Bool,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueType::Str => write!(f, "String"),
            ValueType::Int => write!(f, "Integer"),
            ValueType::Float => write!(f, "Float"),
            ValueType::Bool => write!(f, "Boolean"),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_input(input: String) -> Result<Command, String> {
        let mut parts = input.trim().split_whitespace();

        let command_case_insensitive = parts.next().map(|s| s.to_lowercase());

        match command_case_insensitive.as_deref() {
            Some("set") => SetCommand::from_input(input),
            Some("get") => GetCommand::from_input(input),
            Some("del") => DelCommand::from_input(input),
            Some("help") => HelpCommand::from_input(input),
            Some(cmd) => parse_unknown_command(cmd),
            None => Ok(Command::Empty),
        }
    }
}

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
        match Parser::parse_input(input) {
            Ok(Command::Set(..)) => (),
            _ => panic!("Expected Command::Set"),
        }
    }

    #[test]
    fn test_parse_input_of_get_command() {
        let input = "get x".to_string();
        match Parser::parse_input(input) {
            Ok(Command::Get(..)) => (),
            _ => panic!("Expected Command::Get"),
        }
    }

    #[test]
    fn test_parse_input_of_del_command() {
        let input = "del x".to_string();
        match Parser::parse_input(input) {
            Ok(Command::Del(..)) => (),
            _ => panic!("Expected Command::Del"),
        }
    }

    #[test]
    fn test_parse_input_of_help_command() {
        let input = "help".to_string();
        match Parser::parse_input(input) {
            Ok(Command::Help(..)) => (),
            _ => panic!("Expected Command::Help"),
        }
    }

    #[test]
    fn test_parse_input_of_unknown_command() {
        let input = "not-a-command".to_string();
        match Parser::parse_input(input) {
            Ok(Command::Unknown(cmd)) => {
                assert_eq!(cmd, "not-a-command");
            }
            _ => panic!("Expected Command::Unknown"),
        }
    }

    #[test]
    fn test_parse_input_of_empty_command() {
        let input = "".to_string();
        match Parser::parse_input(input) {
            Ok(Command::Empty) => (),
            _ => panic!("Expected Command::Empty"),
        }
    }
}
