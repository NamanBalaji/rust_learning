use std::{fs::DirEntry, process::Command};

pub fn run_external_executable(executable: DirEntry, arguments: &[String]) {
    let name = executable.file_name();
    let name = name.to_str().unwrap();

    let mut command = Command::new(name);
    command.args(arguments);

    let Ok(mut process_child) = command.spawn() else {
        return;
    };

    let Ok(_result) = process_child.wait() else {
        return;
    };
}
