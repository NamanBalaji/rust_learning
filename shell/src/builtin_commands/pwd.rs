use std::env::current_dir;

use anyhow::Result;

use crate::builtin_commands::echo::echo;

pub fn pwd() -> Result<()> {
    let path = current_dir()?;
    let stringified_path = path.as_os_str().to_str().unwrap_or_default();
    echo(&[stringified_path]);

    Ok(())
}
