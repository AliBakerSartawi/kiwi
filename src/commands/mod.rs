use std::future::Future;

use crate::store::ArcMutexStore;

use self::{
    del_command::DelCommand, delmany_command::DelManyCommand, get_command::GetCommand,
    help_command::HelpCommand, set_command::SetCommand, touch_command::TouchCommand,
    touchmany_command::TouchManyCommand,
};

pub mod del_command;
pub mod delmany_command;
pub mod get_command;
pub mod help_command;
pub mod set_command;
pub mod touch_command;
pub mod touchmany_command;

pub enum Command {
    Set(SetCommand),
    Get(GetCommand),
    Del(DelCommand),
    DelMany(DelManyCommand),
    Touch(TouchCommand),
    TouchMany(TouchManyCommand),
    Help(HelpCommand),
    Unknown(String),
    Empty,
}

/// Used as an interface, not as a trait
///
/// I'm just afraid that dynamic dispatches might slow things down a little
///
/// For example, I'm statically dispatching here:
///
/// ```ignore
/// match command_case_insensitive.as_deref() {
///     Some("set") => SetCommand::from_input(input),
///     Some("get") => GetCommand::from_input(input),
///     Some("del") => DelCommand::from_input(input),
///     None => Ok(Command::Empty),
/// }
/// ```
///
/// And here:
///
/// ```ignore
/// match Parser::parse_input(input)? {
///     Command::Set(cmd) => cmd.execute(store).await,
///     Command::Get(cmd) => cmd.execute(store).await,
///     Command::Del(cmd) => cmd.execute(store).await,
///     Command::Empty => Ok("".to_string()),
/// }
/// ```
///
/// It doesn't look the cleanest, but the performance might be worth it
pub trait CommandTrait {
    // TODO refactor this to be from_parts instead from_input to avoid splitting twice
    fn from_input(input: String) -> Result<Command, String>;
    fn execute(self, store: ArcMutexStore) -> impl Future<Output = Result<String, String>> + Send;
}
