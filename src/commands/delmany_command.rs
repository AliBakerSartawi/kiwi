use crate::parser::utils::ParseError;

use super::{Command, CommandTrait};

pub struct DelManyCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for DelManyCommand {
    fn from_input(input: String) -> Result<Command, String> {
        let mut parts = input.trim().split_whitespace();
        parts.next(); // Skip the command

        let keys = parts.map(|s| s.to_string()).collect::<Vec<String>>();

        if keys.is_empty() {
            return Err(ParseError::MissingKeys.to_string());
        }

        Ok(Command::DelMany(Self { keys }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        Ok(store.lock().await.del_many(self.keys).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delmany_command_from_input() {
        let input = "delmany str-x str-y".to_string();
        match DelManyCommand::from_input(input).unwrap() {
            Command::DelMany(cmd) => {
                assert_eq!(cmd.keys, vec!["str-x", "str-y"]);
            }
            _ => panic!("Expected a DelMany command"),
        };
    }

    #[test]
    fn test_delmany_command_from_input_missing_keys() {
        let input = "delmany".to_string();
        match DelManyCommand::from_input(input) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
