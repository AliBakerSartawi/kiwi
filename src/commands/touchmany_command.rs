use std::str::SplitWhitespace;

use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct TouchManyCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for TouchManyCommand {
    fn from_parts(parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let keys = parts.map(|s| s.to_string()).collect::<Vec<String>>();

        if keys.is_empty() {
            return Err(ParseError::MissingKeys.to_string());
        }

        Ok(CommandWrapper::TouchMany(Self { keys }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        Ok(store.lock().await.touch_many(self.keys).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touchmany_command_from_input() {
        let input = "touchmany str-x str-y".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchManyCommand::from_parts(parts).unwrap() {
            CommandWrapper::TouchMany(cmd) => {
                assert_eq!(cmd.keys, vec!["str-x", "str-y"]);
            }
            _ => panic!("Expected a TouchMany command"),
        };
    }

    #[test]
    fn test_touchmany_command_from_input_missing_keys() {
        let input = "touchmany".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match TouchManyCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
