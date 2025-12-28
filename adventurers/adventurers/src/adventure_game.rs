use crate::{player::Player, point::Point};
use termgame::{
    Controller, Game, GameEvent, KeyCode, SimpleEvent, StyledCharacter, ViewportLocation,
};

const VIEW_W: i32 = 78;
const VIEW_H: i32 = 22;
const MARGIN: i32 = 2;

pub struct AdventureGame {
    player: Player,
}

impl AdventureGame {
    pub fn new() -> Self {
        AdventureGame {
            player: Player::new(),
        }
    }

    fn update_player_pos(&mut self, direction: KeyCode) -> Point {
        let old_pos = self.player.position;
        self.player.move_player(direction);

        old_pos
    }

    fn player_needs_viewport_scroll(&self, game: &Game) -> bool {
        let vp = game.get_viewport();
        let pos = self.player.get_position();

        let rx = pos.x - vp.x;
        let ry = pos.y - vp.y;

        let left = MARGIN;
        let right = VIEW_W - 1 - MARGIN;
        let top = MARGIN;
        let bottom = VIEW_H - 1 - MARGIN;

        rx < left || rx > right || ry < top || ry > bottom
    }

    fn compute_adjusted_viewport(&self, game: &Game) -> ViewportLocation {
        let mut vp = game.get_viewport();

        let pos = self.player.get_position();

        let rx = pos.x - vp.x;
        let ry = pos.y - vp.y;

        let left = MARGIN;
        let right = VIEW_W - 1 - MARGIN;
        let top = MARGIN;
        let bottom = VIEW_H - 1 - MARGIN;

        if rx < left {
            vp.x += rx - left;
        } else if rx > right {
            vp.x += rx - right;
        }

        if ry < top {
            vp.y += ry - top;
        } else if ry > bottom {
            vp.y += ry - bottom;
        }

        vp
    }

    fn render_player(&self, game: &mut Game, old_position: Option<&Point>) {
        if let Some(p) = old_position {
            game.set_screen_char(p.x, p.y, None);
        }

        if self.player_needs_viewport_scroll(game) {
            let new_vp = self.compute_adjusted_viewport(game);
            game.set_viewport(new_vp);
        }

        let pos = self.player.get_position();

        let px = pos.x;
        let py = pos.y;
        game.set_screen_char(px, py, Some(StyledCharacter::new(self.player.symbol)));
    }
}

impl Controller for AdventureGame {
    fn on_start(&mut self, game: &mut Game) {
        self.render_player(game, None);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        if let SimpleEvent::Just(key_event) = event.into() {
            let old_pos = self.update_player_pos(key_event);
            self.render_player(game, Some(&old_pos));
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {
        // Called every tick (for animations, timers, etc.)
    }
}
