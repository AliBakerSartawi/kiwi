use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct DelCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for DelCommand {
    fn from_parts(parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let keys = parts.map(|s| s.to_string()).collect::<Vec<String>>();

        if keys.is_empty() {
            return Err(ParseError::MissingKeys.to_string());
        }

        Ok(CommandWrapper::Del(Self { keys }))
    }

    async fn execute(self, store: crate::store::ConcurrentStore) -> Result<String, String> {
        Ok(store.del_many(self.keys).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_command_from_input() {
        let input = "del x y".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelCommand::from_parts(parts).unwrap() {
            CommandWrapper::Del(cmd) => {
                assert_eq!(cmd.keys, vec!["x", "y"]);
            }
            _ => panic!("Expected a Del command"),
        };
    }

    #[test]
    fn test_del_command_from_input_missing_keys() {
        let input = "del".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
