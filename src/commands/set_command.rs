use std::str::SplitWhitespace;

use crate::{
    parser::utils::{ParseError, ParserUtils},
    store::Value,
};

use super::{CommandTrait, CommandWrapper};

#[derive(PartialEq)]
pub enum SetXxNx {
    /// Set the key only if it already exists
    Xx,
    /// Only set the key if it does not already exist
    Nx,
}

pub struct SetCommandOptions {
    /// Set the key only if it already exists or does not exist (`x` is for existence)
    pub x: Option<SetXxNx>,
    /// Get the value of the key before setting it
    pub get: Option<bool>,
    /// timestamp-milliseconds -- Set the specified Unix time at which the key will expire, in milliseconds (a positive integer)
    pub pxat: Option<u64>,
    /// Retain the time to live of the key
    pub keep_ttl: Option<bool>,
}

impl SetCommandOptions {
    fn get_options_from_parts(
        parts: &mut SplitWhitespace<'_>,
    ) -> Result<Option<SetCommandOptions>, String> {
        let mut options = SetCommandOptions {
            x: None,
            get: None,
            pxat: None,
            keep_ttl: None,
        };

        while let Some(part) = parts.next() {
            match part.to_lowercase().as_str() {
                "xx" => {
                    if options.x == Some(SetXxNx::Nx) {
                        return Err(ParseError::InvalidCommandOptions(
                            "Cannot use nx and xx together",
                        )
                        .to_string());
                    }
                    options.x = Some(SetXxNx::Xx);
                }
                "nx" => {
                    if options.x == Some(SetXxNx::Xx) {
                        return Err(ParseError::InvalidCommandOptions(
                            "Cannot use nx and xx together",
                        )
                        .to_string());
                    }
                    options.x = Some(SetXxNx::Nx);
                }
                "get" => options.get = Some(true),
                "keepttl" => options.keep_ttl = Some(true),
                // TODO: ex | px | exat | pxat
                _ => break,
            }
        }

        // TODO: if all options fields are None, return None

        Ok(Some(options))
    }
}

pub struct SetCommand {
    pub key: String,
    pub value: Value,
    pub options: Option<SetCommandOptions>,
}

impl CommandTrait for SetCommand {
    fn from_parts(mut parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        let key = parts.next().ok_or(ParseError::MissingKey.to_string())?;
        let type_option = ParserUtils::get_type_from_key(key);
        let r#type = type_option.ok_or(ParseError::InvalidType.to_string())?;
        let raw_value = parts.next().ok_or(ParseError::MissingValue.to_string())?;
        let parsed_value = ParserUtils::parse_raw_value(raw_value, r#type)?;

        let options = SetCommandOptions::get_options_from_parts(&mut parts)?;

        Ok(CommandWrapper::Set(Self {
            key: key.to_string(),
            value: parsed_value,
            options,
        }))
    }

    async fn execute(self, store: crate::store::ArcMutexStore) -> Result<String, String> {
        // TODO: Implement executing the options
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
