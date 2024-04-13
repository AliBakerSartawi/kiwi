use std::future::Future;

use crate::store::ArcMutexStore;

use self::{
    del_command::DelCommand, delmany_command::DelManyCommand, get_command::GetCommand,
    help_command::HelpCommand, set_command::SetCommand,
};

pub mod del_command;
pub mod delmany_command;
pub mod get_command;
pub mod help_command;
pub mod set_command;

pub enum Command {
    Set(SetCommand),
    Get(GetCommand),
    Del(DelCommand),
    DelMany(DelManyCommand),
    Help(HelpCommand),
    Unknown(String),
    Empty,
}

pub trait CommandTrait {
    fn from_input(input: String) -> Result<Command, String>;
    fn execute(self, store: ArcMutexStore) -> impl Future<Output = Result<String, String>> + Send;
}
