mod game;
mod ray_attacks;

use game::*;

fn main() {    
    println!("{}", Game::initalize().to_string());
}
