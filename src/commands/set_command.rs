use std::str::SplitWhitespace;

use crate::{
    parser::utils::{ParseError, ParserUtils},
    store::Value,
};

use super::{CommandTrait, CommandWrapper};

pub struct SetCommand {
    pub key: String,
    pub value: Value,
}

impl CommandTrait for SetCommand {
    fn from_parts(mut parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let key = parts.next().ok_or(ParseError::MissingKey.to_string())?;
        let type_option = ParserUtils::get_type_from_key(key);
        let r#type = type_option.ok_or(ParseError::InvalidType.to_string())?;
        let raw_value = parts.next().ok_or(ParseError::MissingValue.to_string())?;
        let parsed_value = ParserUtils::parse_raw_value(raw_value, r#type)?;

        Ok(CommandWrapper::Set(Self {
            key: key.to_string(),
            value: parsed_value,
        }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        store.lock().await.set(self.key, self.value);
        Ok("OK".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command_from_input() {
        let input = "set str-key value".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match SetCommand::from_parts(parts).unwrap() {
            CommandWrapper::Set(cmd) => {
                assert_eq!(cmd.key, "str-key");
                assert_eq!(cmd.value, Value::Str("value".to_string()));
            }
            _ => panic!("Expected a Set command"),
        };
    }

    #[test]
    fn test_set_command_from_input_missing_key() {
        let input = "set".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match SetCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingKey.to_string()),
            _ => panic!("Expected an error"),
        };
    }

    #[test]
    fn test_set_command_from_input_missing_value() {
        let input = "set str-key".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match SetCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::MissingValue.to_string()),
            _ => panic!("Expected an error"),
        };
    }

    #[test]
    fn test_set_command_from_input_invalid_type() {
        let input = "set x-key 123".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match SetCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::InvalidType.to_string()),
            _ => panic!("Expected an error"),
        };
    }

    #[test]
    fn test_set_command_from_input_missing_type() {
        let input = "set key value".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match SetCommand::from_parts(parts) {
            Err(e) => assert_eq!(e, ParseError::InvalidType.to_string()),
            _ => panic!("Expected an error"),
        };
    }
}
