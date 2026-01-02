mod builtin_commands;
mod errors;
pub mod utils;

use anyhow::Result;

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, echo::echo, pwd::pwd,
        run_external_executable::run_external_executable,
    },
    errors::CustomErrors,
    utils::{find_executable_file, get_command, get_path, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    let path = get_path()?;
    let current_dir = std::env::current_dir().expect("Error: no current working directory");

    loop {
        print_prompt();

        let command = get_command()?;

        match command {
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice()),
            BuiltinCommand::Exit => break,
            BuiltinCommand::Pwd => pwd(&current_dir),
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    run_external_executable(executable, &arguments);
                } else {
                    let error = CustomErrors::CommandNotFound(command_string);
                    print_error(error);
                }
            }
        }
    }

    Ok(())
}
