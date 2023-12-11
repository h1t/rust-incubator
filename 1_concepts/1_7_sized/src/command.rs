use std::{error::Error, fmt::Display};

use crate::{User, UserRepository};

pub trait Command {}

pub trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository<u64>;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        ctx.set(cmd.id, self.clone());

        Ok(())
    }
}

pub struct CreateUser {
    pub id: u64,
}

impl Command for CreateUser {}

#[derive(Debug)]
pub struct UserError;

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("user error")
    }
}

impl Error for UserError {}
