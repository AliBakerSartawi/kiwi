use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct GetCommand {
    pub key: String,
}

impl CommandTrait for GetCommand {
    fn from_parts(mut parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let key = parts
            .next()
            .ok_or(ParseError::MissingKey.to_string())?
            .to_string();

        Ok(CommandWrapper::Get(Self {
            key: key.to_string(),
        }))
    }

    async fn execute(self, store: crate::store::ConcurrentStore) -> Result<String, String> {
        match store.get(&self.key) {
            Some(response) => Ok(response),
            None => Ok("Key not found".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_input() {
        let input = "get str-key".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match GetCommand::from_parts(parts).unwrap() {
            CommandWrapper::Get(cmd) => {
                assert_eq!(cmd.key, "str-key");
            }
            _ => panic!("Expected a Get command"),
        };
    }

    #[test]
    fn test_get_command_from_input_missing_key() {
        let input = "get".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match GetCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
