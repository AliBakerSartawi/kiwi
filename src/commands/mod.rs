use std::{future::Future, str::SplitWhitespace};

use crate::store::ConcurrentStore;

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

pub enum CommandWrapper {
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
///
/// # References
///
/// - https://users.rust-lang.org/t/performance-implications-of-box-trait-vs-enum-delegation/11957
///   - It seems that the enum approach and static dispatch is faster because it is stack-based, whereas the `dyn Trait` or `Box<dyn Trait>` approach is heap-based
pub trait CommandTrait {
    /// The parts do not include the command itself
    fn from_parts(parts: SplitWhitespace<'_>) -> Result<CommandWrapper, String>;
    fn execute(self, store: ConcurrentStore)
        -> impl Future<Output = Result<String, String>> + Send;
}
