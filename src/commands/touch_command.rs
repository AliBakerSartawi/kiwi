use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct TouchCommand {
    pub key: String,
}

impl CommandTrait for TouchCommand {
    fn from_parts(mut parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let key = parts
            .next()
            .ok_or(ParseError::MissingKey.to_string())?
            .to_string();

        Ok(CommandWrapper::Touch(Self {
            key: key.to_string(),
        }))
    }

    async fn execute(self, store: crate::store::ConcurrentStore) -> Result<String, String> {
        Ok(store.touch(&self.key).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_command_from_input() {
        let input = "touch str-key".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchCommand::from_parts(parts).unwrap() {
            CommandWrapper::Touch(cmd) => {
                assert_eq!(cmd.key, "str-key");
            }
            _ => panic!("Expected a Touch command"),
        };
    }

    #[test]
    fn test_touch_command_from_input_missing_key() {
        let input = "touch".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
