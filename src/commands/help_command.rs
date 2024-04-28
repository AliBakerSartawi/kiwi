use std::str::SplitWhitespace;

use super::{CommandTrait, CommandWrapper};

pub struct HelpCommand;

impl CommandTrait for HelpCommand {
    fn from_parts(_parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String> {
        Ok(CommandWrapper::Help(Self))
    }

    async fn execute(self, _store: crate::store::ConcurrentStore) -> Result<String, String> {
        Ok(HELP_MESSAGE.to_string())
    }
}

const HELP_MESSAGE: &str = "\
Commands:
  set <key> <value>        - Set a key-value pair
  get <key>                - Get the value associated with a key
  del <key>                - Delete a key-value pair
  exit                     - Exit the shell
  help                     - Show this help message";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_command_from_input() {
        let input = "help".to_string();
        let mut parts = input.split_whitespace();
        parts.next(); // Skip the command
        match HelpCommand::from_parts(parts).unwrap() {
            CommandWrapper::Help(_cmd) => (),
            _ => panic!("Expected a Help command"),
        };
    }
}
