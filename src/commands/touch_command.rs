use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct TouchCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for TouchCommand {
    fn from_parts(parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let keys = parts.map(|s| s.to_string()).collect::<Vec<String>>();

        if keys.is_empty() {
            return Err(ParseError::MissingKeys.to_string());
        }

        Ok(CommandWrapper::Touch(Self { keys }))
    }

    async fn execute(self, store: crate::store::ConcurrentStore) -> Result<String, String> {
        Ok(store.touch_many(self.keys).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_command_from_input() {
        let input = "touch x y".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchCommand::from_parts(parts).unwrap() {
            CommandWrapper::Touch(cmd) => {
                assert_eq!(cmd.keys, vec!["x", "y"]);
            }
            _ => panic!("Expected a Touch command"),
        };
    }

    #[test]
    fn test_touch_command_from_input_missing_keys() {
        let input = "touch".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
