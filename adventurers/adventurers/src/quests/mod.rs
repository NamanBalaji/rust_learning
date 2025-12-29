use std::env;

use adventurers_quest::Quest;

use crate::{
    blocks::Blocks,
    quests::{
        combinators::{majority::Majority, repeat::Repeat, then::Then},
        game_event::GameEvent,
        walk::WalkQuest,
        walk_repeated::WalkRepeatQuest,
    },
};

pub mod combinators;
pub mod game_event;
mod walk;
mod walk_repeated;

const Q1: &str = "q1";
const Q2: &str = "q2";
const Q3: &str = "q3";

pub fn get_quest() -> Result<Box<dyn Quest<GameEvent>>, String> {
    let quest_no = env::args().nth(2);
    if quest_no.is_none() {
        return Err("no quest provided".to_owned());
    }
    let quest = quest_no.unwrap();

    match quest.as_str() {
        Q1 => Ok(Box::new(Repeat::new(
            Box::new(WalkQuest::new(Blocks::Sand)),
            5,
        ))),
        Q2 => Ok(Box::new(Then::new(
            Box::new(Repeat::new(
                Box::new(WalkQuest::new(Blocks::Object('x'))),
                5,
            )),
            Box::new(Repeat::new(
                Box::new(WalkQuest::new(Blocks::Object('y'))),
                3,
            )),
        ))),
        Q3 => Ok(Box::new(Majority::new(
            Box::new(Then::new(
                Box::new(Repeat::new(Box::new(WalkQuest::new(Blocks::Sand)), 5)),
                Box::new(WalkQuest::new(Blocks::Object('x'))),
            )),
            Box::new(Then::new(
                Box::new(WalkQuest::new(Blocks::Object('x'))),
                Box::new(WalkQuest::new(Blocks::Grass)),
            )),
            Box::new(Repeat::new(
                Box::new(WalkRepeatQuest::new(Blocks::Water, 9)),
                2,
            )),
        ))),

        _ => Err("provided quest does not exist".to_string()),
    }
}
