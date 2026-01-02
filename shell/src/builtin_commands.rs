pub mod builtin_type;
pub mod echo;
pub mod pwd;
pub mod run_external_executable;

pub type CommandArguments = Vec<String>;

pub enum BuiltinCommand {
    Echo(CommandArguments),
    Exit,
    Pwd,
    Type(CommandArguments),
    NotFound(String, CommandArguments),
}

impl From<(String, CommandArguments)> for BuiltinCommand {
    fn from((command, arguments): (String, CommandArguments)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            "pwd" => Self::Pwd,
            "type" => Self::Type(arguments),
            _ => Self::NotFound(command, arguments),
        }
    }
}
