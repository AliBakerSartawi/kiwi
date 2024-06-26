use crate::commands::{
    del_command::DelCommand, get_command::GetCommand, help_command::HelpCommand,
    set_command::SetCommand, touch_command::TouchCommand, CommandTrait, CommandWrapper,
};

pub mod utils;

pub struct Parser;

impl Parser {
    pub fn parse_input(input: String) -> Result<CommandWrapper, String> {
        let mut parts = input.trim().split_whitespace();

        let command_case_insensitive = parts.next().map(|s| s.to_lowercase());

        match command_case_insensitive.as_deref() {
            Some("set") => SetCommand::from_parts(parts),
            Some("get") => GetCommand::from_parts(parts),
            Some("del") => DelCommand::from_parts(parts),
            Some("touch") => TouchCommand::from_parts(parts),
            Some("help") => HelpCommand::from_parts(parts),
            Some(cmd) => parse_unknown_command(cmd),
            None => Ok(CommandWrapper::Empty),
        }
    }
}

fn parse_unknown_command(cmd: &str) -> Result<CommandWrapper, String> {
    Ok(CommandWrapper::Unknown(cmd.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_of_set_command() {
        let input = "set x y".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Set(..)) => (),
            _ => panic!("Expected Command::Set"),
        }
    }

    #[test]
    fn test_parse_input_of_get_command() {
        let input = "get x".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Get(..)) => (),
            _ => panic!("Expected Command::Get"),
        }
    }

    #[test]
    fn test_parse_input_of_del_command() {
        let input = "del x".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Del(..)) => (),
            _ => panic!("Expected Command::Del"),
        }
    }

    #[test]
    fn test_parse_input_of_touch_command() {
        let input = "touch x".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Touch(..)) => (),
            _ => panic!("Expected Command::Touch"),
        }
    }

    #[test]
    fn test_parse_input_of_help_command() {
        let input = "help".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Help(..)) => (),
            _ => panic!("Expected Command::Help"),
        }
    }

    #[test]
    fn test_parse_input_of_unknown_command() {
        let input = "not-a-command".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Unknown(cmd)) => {
                assert_eq!(cmd, "not-a-command");
            }
            _ => panic!("Expected Command::Unknown"),
        }
    }

    #[test]
    fn test_parse_input_of_empty_command() {
        let input = "".to_string();
        match Parser::parse_input(input) {
            Ok(CommandWrapper::Empty) => (),
            _ => panic!("Expected Command::Empty"),
        }
    }
}
