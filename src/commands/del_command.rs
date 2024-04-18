use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct DelCommand {
    pub key: String,
}

impl CommandTrait for DelCommand {
    fn from_parts(mut parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let key = parts
            .next()
            .ok_or(ParseError::MissingKey.to_string())?
            .to_string();

        Ok(CommandWrapper::Del(Self {
            key: key.to_string(),
        }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        match store.lock().await.del(&self.key) {
            Some(value) => Ok(value.to_string()),
            None => Ok("Key not found".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_command_from_input() {
        let input = "del str-key".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelCommand::from_parts(parts).unwrap() {
            CommandWrapper::Del(cmd) => {
                assert_eq!(cmd.key, "str-key");
            }
            _ => panic!("Expected a Del command"),
        };
    }

    #[test]
    fn test_del_command_from_input_missing_key() {
        let input = "del".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
