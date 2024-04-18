use crate::parser::utils::ParseError;

use super::{CommandTrait, CommandWrapper};

pub struct TouchCommand {
    pub key: String,
}

impl CommandTrait for TouchCommand {
    fn from_input(input: String) -> Result<CommandWrapper, String> {
        let mut parts = input.trim().split_whitespace();
        parts.next(); // Skip the command

        let key = parts
            .next()
            .ok_or(ParseError::MissingKey.to_string())?
            .to_string();

        Ok(CommandWrapper::Touch(Self {
            key: key.to_string(),
        }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        Ok(store.lock().await.touch(&self.key).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_command_from_input() {
        let input = "touch str-key".to_string();
        match TouchCommand::from_input(input).unwrap() {
            CommandWrapper::Touch(cmd) => {
                assert_eq!(cmd.key, "str-key");
            }
            _ => panic!("Expected a Touch command"),
        };
    }

    #[test]
    fn test_touch_command_from_input_missing_key() {
        let input = "touch".to_string();
        match TouchCommand::from_input(input) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
