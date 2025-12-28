use serde::Deserialize;
use termgame::{GameColor, GameStyle, StyledCharacter};

#[derive(Clone, Deserialize)]
pub enum Blocks {
    Barrier,
    Cinderblock,
    Flowerbush,
    Grass,
    Object(char),
    Rock,
    Sand,
    Sign(String),
    Water,
}

impl Blocks {
    pub fn get_color(&self) -> Option<GameColor> {
        match self {
            Blocks::Barrier => Some(GameColor::White),
            Blocks::Cinderblock => Some(GameColor::LightRed),
            Blocks::Flowerbush => Some(GameColor::Magenta),
            Blocks::Grass => Some(GameColor::Green),
            Blocks::Object(_) => None,
            Blocks::Rock => Some(GameColor::Gray),
            Blocks::Sand => Some(GameColor::Yellow),
            Blocks::Sign(_) => None,
            Blocks::Water => Some(GameColor::Blue),
        }
    }
}

impl From<Blocks> for StyledCharacter {
    fn from(val: Blocks) -> Self {
        match val {
            Blocks::Barrier => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::White))),
            Blocks::Cinderblock => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::LightRed))),
            Blocks::Flowerbush => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
            Blocks::Grass => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Green))),
            Blocks::Object(ch) => StyledCharacter::new(ch),
            Blocks::Rock => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Gray))),
            Blocks::Sand => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
            Blocks::Sign(_) => StyledCharacter::new('💬'),
            Blocks::Water => StyledCharacter::new(' ')
                .style(GameStyle::new().background_color(Some(GameColor::Blue))),
        }
    }
}
