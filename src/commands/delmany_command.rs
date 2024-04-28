use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct DelManyCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for DelManyCommand {
    fn from_parts(parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let keys = parts.map(|s| s.to_string()).collect::<Vec<String>>();

        if keys.is_empty() {
            return Err(ParseError::MissingKeys.to_string());
        }

        Ok(CommandWrapper::DelMany(Self { keys }))
    }

    async fn execute(self, store: crate::store::ConcurrentStore) -> Result<String, String> {
        Ok(store.del_many(self.keys).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delmany_command_from_input() {
        let input = "delmany str-x str-y".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelManyCommand::from_parts(parts).unwrap() {
            CommandWrapper::DelMany(cmd) => {
                assert_eq!(cmd.keys, vec!["str-x", "str-y"]);
            }
            _ => panic!("Expected a DelMany command"),
        };
    }

    #[test]
    fn test_delmany_command_from_input_missing_keys() {
        let input = "delmany".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match DelManyCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
