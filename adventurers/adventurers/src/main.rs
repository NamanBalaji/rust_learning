mod adventure_game;
mod blocks;
mod map;
mod movement;
mod player;
mod point;
mod quests;

use adventure_game::AdventureGame;
use std::error::Error;
use std::time::Duration;
use termgame::{run_game, GameSettings, KeyCode, SimpleEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let map = map::parse_map()?;
    let quest = quests::get_quest()?;

    let mut game = AdventureGame::new(map, quest);

    run_game(
        &mut game,
        GameSettings::new()
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");
    Ok(())
}
