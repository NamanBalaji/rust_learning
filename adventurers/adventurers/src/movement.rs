use termgame::KeyCode;

use crate::{
    blocks::{self, Blocks},
    map::Map,
    point::{Direction, Point},
};

pub struct Movement {
    pub start_point: Point,
    pub dest_point: Point,
    pub to_block: Option<Blocks>,
}

impl Movement {
    pub fn new(start_point: Point, direction: KeyCode, map: &Map) -> Result<Self, String> {
        let mut dest_point = start_point;
        match direction {
            KeyCode::Up => dest_point.move_in_direction(Direction::Up),
            KeyCode::Down => dest_point.move_in_direction(Direction::Down),
            KeyCode::Left => dest_point.move_in_direction(Direction::Left),
            KeyCode::Right => dest_point.move_in_direction(Direction::Right),
            _ => {
                return Err("invalid input".to_owned());
            }
        };

        let mut to_block = None;
        if let Some(b) = map.get(&(dest_point.x, dest_point.y)) {
            to_block = Some(b.clone());
        }

        Ok(Movement {
            start_point,
            dest_point,
            to_block,
        })
    }

    pub fn can_move(&self) -> bool {
        if let Some(b) = &self.to_block {
            !matches!(b, blocks::Blocks::Barrier)
        } else {
            true
        }
    }
}
