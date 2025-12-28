use std::{collections::HashMap, env, fs};

use crate::blocks::Blocks;

pub type Map = HashMap<(i32, i32), Blocks>;

pub fn parse_map() -> Result<Map, String> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| "Usage: cargo run <map_file>".to_owned())?;

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read '{}': {}", path, e))?;

    ron::from_str(&contents).map_err(|e| format!("Failed to parse map: {}", e))
}
