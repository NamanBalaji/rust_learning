use termgame::KeyCode;

use crate::{
    blocks::Blocks,
    map::Map,
    point::{Direction, Point},
};

pub struct Movement {
    pub start_point: Point,
    pub dest_point: Point,
}

impl Movement {
    pub fn new(start_point: Point, direction: KeyCode) -> Result<Self, String> {
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

        Ok(Movement {
            start_point,
            dest_point,
        })
    }

    pub fn get_dest_block<'a>(&self, map: &'a Map) -> Option<&'a Blocks> {
        map.get(&(self.dest_point.x, self.dest_point.y))
    }

    pub fn can_move(&self, map: &Map) -> bool {
        !matches!(self.get_dest_block(map), Some(Blocks::Barrier))
    }
}
