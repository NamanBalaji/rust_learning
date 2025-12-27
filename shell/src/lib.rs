mod builtin_commands;
mod errors;
pub mod utils;

use anyhow::Result;

use crate::{
    builtin_commands::{BuiltinCommand, builtin_type::builtin_type, echo::echo},
    errors::CustomErrors,
    utils::{get_command, get_path, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    let path = get_path()?;

    loop {
        print_prompt();

        let command = get_command()?;

        match command {
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice()),
            BuiltinCommand::Exit => break,
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::NotFound(command_string) => {
                let error = CustomErrors::CommandNotFound(command_string);
                print_error(error);
            }
        }
    }

    Ok(())
}
