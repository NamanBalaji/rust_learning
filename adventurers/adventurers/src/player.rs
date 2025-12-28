use termgame::KeyCode;

use crate::point::{Direction, Point};

const PLAYER_SYMBOL: char = '♟';

pub struct Player {
    pub position: Point,
    pub symbol: char,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Point::new(2, 2),
            symbol: PLAYER_SYMBOL,
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn move_player(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => self.position.move_in_direction(Direction::Up),
            KeyCode::Down => self.position.move_in_direction(Direction::Down),
            KeyCode::Left => self.position.move_in_direction(Direction::Left),
            KeyCode::Right => self.position.move_in_direction(Direction::Right),
            _ => {}
        };
    }
}
