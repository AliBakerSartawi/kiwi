use crate::{
    commands::{CommandTrait, CommandWrapper},
    parser::Parser,
    store::ConcurrentStore,
};

pub async fn handle_command(input: String, store: ConcurrentStore) -> Result<String, String> {
    match Parser::parse_input(input)? {
        CommandWrapper::Set(cmd) => cmd.execute(store).await,
        CommandWrapper::Get(cmd) => cmd.execute(store).await,
        CommandWrapper::Del(cmd) => cmd.execute(store).await,
        CommandWrapper::DelMany(cmd) => cmd.execute(store).await,
        CommandWrapper::Touch(cmd) => cmd.execute(store).await,
        CommandWrapper::TouchMany(cmd) => cmd.execute(store).await,
        CommandWrapper::Help(cmd) => cmd.execute(store).await,
        CommandWrapper::Unknown(cmd) => Ok(format!("Unknown command: {}", cmd)),
        CommandWrapper::Empty => Ok("".to_string()),
    }
}
