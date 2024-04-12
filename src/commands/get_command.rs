use crate::parser::utils::ParseError;

use super::{Command, CommandTrait};

pub struct GetCommand {
    pub key: String,
}

impl CommandTrait for GetCommand {
    fn from_input(input: String) -> Result<Command, String> {
        let mut parts = input.trim().split_whitespace();
        parts.next(); // Skip the command

        let key = parts
            .next()
            .ok_or(ParseError::MissingKey.to_string())?
            .to_string();

        Ok(Command::Get(GetCommand {
            key: key.to_string(),
        }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        match store.lock().await.get(&self.key) {
            Some(value) => Ok(value.to_string()),
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
        match GetCommand::from_input(input).unwrap() {
            Command::Get(cmd) => {
                assert_eq!(cmd.key, "str-key");
            }
            _ => panic!("Expected a Get command"),
        };
    }

    #[test]
    fn test_get_command_from_input_missing_key() {
        let input = "get".to_string();
        match GetCommand::from_input(input) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
