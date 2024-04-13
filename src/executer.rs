use crate::{
    commands::{Command, CommandTrait},
    parser::Parser,
    store::ArcMutexStore,
};

pub async fn handle_command(input: String, store: ArcMutexStore) -> Result<String, String> {
    match Parser::parse_input(input)? {
        Command::Set(cmd) => cmd.execute(store).await,
        Command::Get(cmd) => cmd.execute(store).await,
        Command::Del(cmd) => cmd.execute(store).await,
        Command::DelMany(cmd) => cmd.execute(store).await,
        Command::Help(cmd) => cmd.execute(store).await,
        Command::Unknown(cmd) => Ok(format!("Unknown command: {}", cmd)),
        Command::Empty => Ok("".to_string()),
    }
}
