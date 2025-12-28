use crate::{blocks::Blocks, map, movement::Movement, player::Player, point::Point};
use termgame::{
    Controller, Game, GameEvent, GameStyle, Message, SimpleEvent, StyledCharacter, ViewportLocation,
};

pub const VP_SIZE: (i32, i32) = (78, 22);
pub const VP_BUFFER: i32 = 1;

pub struct AdventureGame {
    map: map::Map,
    player: Player,
    over: bool,
}

impl AdventureGame {
    pub fn new(map: map::Map) -> Self {
        AdventureGame {
            map,
            player: Player::new(),
            over: false,
        }
    }

    fn render_on_start(&self, game: &mut Game) {
        self.map.iter().for_each(|((x, y), b)| {
            game.set_screen_char(*x, *y, Some(b.clone().into()));
        });

        let pos = self.player.get_position();
        let mv = Movement {
            start_point: *pos,
            dest_point: *pos,
            to_block: self.map.get(&(pos.x, pos.y)).cloned(),
        };
        self.render_move(game, &mv);
    }

    fn move_viewport(&self, game: &mut Game) {
        let vp = game.get_viewport();
        let mut vp_x = vp.x;
        let mut vp_y = vp.y;

        let pos = self.player.get_position();
        if pos.x - vp.x == VP_BUFFER {
            vp_x -= 1;
        } else if vp.x + VP_SIZE.0 - 1 - pos.x == VP_BUFFER {
            vp_x += 1;
        }

        if pos.y - vp.y == VP_BUFFER {
            vp_y -= 1;
        } else if vp.y + VP_SIZE.1 - 1 - pos.y == VP_BUFFER {
            vp_y += 1;
        }

        game.set_viewport(ViewportLocation { x: vp_x, y: vp_y });
    }

    fn on_movement(&mut self, game: &mut Game, mv: &Movement) {
        if !mv.can_move() {
            return;
        }

        self.apply_move(mv);
        self.render_move(game, mv);
    }

    fn apply_move(&mut self, mv: &Movement) {
        self.player.move_to(&mv.dest_point);
        match mv.to_block {
            Some(Blocks::Water) => {
                self.over = self.player.inc_water_count();
            }
            Some(Blocks::Object(o)) => {
                self.player.collect(o);
                self.map.remove(&(mv.dest_point.x, mv.dest_point.y));
            }
            _ => {}
        }
        if !matches!(mv.to_block, Some(Blocks::Water)) {
            self.player.reset_water_count();
        }
    }

    fn render_move(&self, game: &mut Game, mv: &Movement) {
        self.reset_block(game, &mv.start_point);
        self.format_dest_block(game, mv);

        if let Some(Blocks::Sign(s)) = &mv.to_block {
            game.set_message(Some(Message::new(s.clone())));
        }

        self.move_viewport(game);

        if self.over {
            game.set_message(Some(Message::new(
                "You drowned! Press any key to exit.".to_owned(),
            )));
        }
    }

    fn format_dest_block(&self, game: &mut Game, mv: &Movement) {
        let block_style: StyledCharacter;
        if let Some(b) = &mv.to_block {
            block_style = StyledCharacter::new(self.player.get_symbol())
                .style(GameStyle::new().background_color(b.get_color()));
        } else {
            block_style = StyledCharacter::new(self.player.get_symbol());
        }
        game.set_screen_char(mv.dest_point.x, mv.dest_point.y, Some(block_style));
    }

    fn reset_block(&self, game: &mut Game, p: &Point) {
        let b = self.map.get(&(p.x, p.y));
        if let Some(b) = b {
            game.set_screen_char(p.x, p.y, Some(b.clone().into()));
        } else {
            game.set_screen_char(p.x, p.y, None);
        }
    }
}

impl Controller for AdventureGame {
    fn on_start(&mut self, game: &mut Game) {
        self.render_on_start(game);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        if let SimpleEvent::Just(key_event) = event.into() {
            game.set_message(None);
            if self.over {
                game.end_game();
            }

            match Movement::new(*self.player.get_position(), key_event, &self.map) {
                Ok(m) => self.on_movement(game, &m),
                Err(e) => game.set_message(Some(Message::new(e))),
            }
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {
        // Called every tick (for animations, timers, etc.)
    }
}
