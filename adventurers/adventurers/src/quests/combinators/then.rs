use std::fmt::Display;

use adventurers_quest::{Quest, QuestStatus};

use crate::quests::game_event::GameEvent;

/// The state of the then quest combinator
#[derive(Debug)]
pub struct Then {
    /// The first quest to be completed
    q1: Box<dyn Quest<GameEvent>>,
    /// The second quest to be completed
    q2: Box<dyn Quest<GameEvent>>,
    /// The status of the then quest
    status: QuestStatus,
}

impl Then {
    /// Create a new then quest given the two sub-quests to be completed in order
    pub fn new(q1: Box<dyn Quest<GameEvent>>, q2: Box<dyn Quest<GameEvent>>) -> Self {
        Self {
            q1,
            q2,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for Then {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q1 = format!("{}", self.q1);
        let q1 = q1.replace('\n', "\n\t");
        let q2 = format!("{}", self.q2);
        let q2 = q2.replace('\n', "\n\t");
        match self.status {
            QuestStatus::Complete => write!(
                f,
                "[✅] You must, in order, complete each of these quests:\n\t{}\n\t{}",
                q1, q2
            ),
            QuestStatus::Ongoing => write!(
                f,
                "[ ] You must, in order, complete each of these quests:\n\t{}\n\t{}",
                q1, q2
            ),
        }
    }
}

impl Quest<GameEvent> for Then {
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        if self.q1.register_event(event) == QuestStatus::Complete
            && self.q2.register_event(event) == QuestStatus::Complete
        {
            self.status = QuestStatus::Complete;
        }

        self.status
    }

    fn reset(&mut self) {
        self.q1.reset();
        self.q2.reset();
        self.status = QuestStatus::Ongoing;
    }
}
