use std::{
    env::{home_dir, set_current_dir},
    path::Path,
    process::exit,
};

use anyhow::Result;

use crate::utils::print_error;

pub fn chnage_directory(arguments: &[String]) -> Result<()> {
    let Some(home_directory) = home_dir() else {
        print_error("Error, you don't seem to have a home directory");
        exit(1);
    };

    let Some(target_path) = arguments.first() else {
        set_current_dir(home_directory)?;
        return Ok(());
    };

    let target_path = target_path.replace("~", home_directory.to_str().unwrap_or_default());
    let target_path = Path::new(&target_path);
    if target_path.is_dir() {
        set_current_dir(target_path)?;
    } else {
        print_error(format!(
            "cd: {}: No such file or directory",
            target_path.to_str().unwrap_or_default()
        ));
    }

    Ok(())
}
