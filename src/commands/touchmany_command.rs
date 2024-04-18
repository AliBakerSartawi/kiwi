use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct TouchManyCommand {
    pub keys: Vec<String>,
}

impl CommandTrait for TouchManyCommand {
    fn from_input(input: String) -> Result<CommandWrapper, String> {
        let mut parts = input.trim().split_whitespace();
        parts.next(); // Skip the command

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
        match TouchManyCommand::from_input(input).unwrap() {
            CommandWrapper::TouchMany(cmd) => {
                assert_eq!(cmd.keys, vec!["str-x", "str-y"]);
            }
            _ => panic!("Expected a TouchMany command"),
        };
    }

    #[test]
    fn test_touchmany_command_from_input_missing_keys() {
        let input = "touchmany".to_string();
        match TouchManyCommand::from_input(input) {
            Err(e) => assert_eq!(e, ParseError::MissingKeys.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
