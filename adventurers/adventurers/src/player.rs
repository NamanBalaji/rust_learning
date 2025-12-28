use crate::point::Point;

const PLAYER_SYMBOL: char = '♟';

pub struct Player {
    position: Point,
    symbol: char,
    water_count: u8,
    collected: Vec<char>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Point::new(2, 2),
            symbol: PLAYER_SYMBOL,
            water_count: 0,
            collected: vec![],
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn move_to(&mut self, p: &Point) {
        self.position.x = p.x;
        self.position.y = p.y;
    }

    pub fn inc_water_count(&mut self) -> bool {
        if self.water_count == 10 {
            return true;
        }
        self.water_count += 1;

        self.water_count == 10
    }

    pub fn reset_water_count(&mut self) {
        self.water_count = 0;
    }

    pub fn collect(&mut self, c: char) {
        self.collected.push(c);
    }
}
