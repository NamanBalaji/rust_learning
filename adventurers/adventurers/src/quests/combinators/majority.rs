use std::fmt::Display;

use adventurers_quest::{Quest, QuestStatus};

use crate::quests::game_event::GameEvent;

#[derive(Debug)]
pub struct Majority {
    /// The first sub-quest
    q1: Box<dyn Quest<GameEvent>>,
    /// The second sub-quest
    q2: Box<dyn Quest<GameEvent>>,
    /// The third sub-quest
    q3: Box<dyn Quest<GameEvent>>,
    /// Whether the first sub-quest is completed
    q1_complete: bool,
    /// Whether the second sub-quest is completed
    q2_complete: bool,
    /// Whether the third sub-quest is completed
    q3_complete: bool,
    /// The status of the majority quest
    status: QuestStatus,
}

impl Majority {
    /// Create a new majority quest given three sub-quests
    ///
    /// All sub-quests start off as incomplete, and the majority quest itself
    /// starts off as ongoing
    pub fn new(
        q1: Box<dyn Quest<GameEvent>>,
        q2: Box<dyn Quest<GameEvent>>,
        q3: Box<dyn Quest<GameEvent>>,
    ) -> Self {
        Self {
            q1,
            q2,
            q3,
            q1_complete: false,
            q2_complete: false,
            q3_complete: false,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for Majority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q1 = format!("{}", self.q1);
        let q1 = q1.replace('\n', "\n\t");
        let q2 = format!("{}", self.q2);
        let q2 = q2.replace('\n', "\n\t");
        let q3 = format!("{}", self.q3);
        let q3 = q3.replace('\n', "\n\t");
        match self.status {
            QuestStatus::Complete => write!(
                f,
                "[✅] You must complete at least 2 of these quests:\n\t{}\n\t{}\n\t{}",
                q1, q2, q3
            ),
            QuestStatus::Ongoing => write!(
                f,
                "[ ] You must complete at least 2 of these quests:\n\t{}\n\t{}\n\t{}",
                q1, q2, q3
            ),
        }
    }
}

impl Quest<GameEvent> for Majority {
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        // check if sub-quest 1 is complete
        if !self.q1_complete && self.q1.register_event(event) == QuestStatus::Complete {
            self.q1_complete = true;
        }

        // check if sub-quest 2 is complete
        if !self.q2_complete && self.q2.register_event(event) == QuestStatus::Complete {
            self.q2_complete = true;
        }

        // check if sub-quest 3 is complete
        if !self.q3_complete && self.q3.register_event(event) == QuestStatus::Complete {
            self.q3_complete = true;
        }

        // check if the majority of sub-quests are complete
        let completed_count = if self.q1_complete { 1 } else { 0 }
            + if self.q2_complete { 1 } else { 0 }
            + if self.q3_complete { 1 } else { 0 };

        if completed_count >= 2 {
            self.status = QuestStatus::Complete;
        }

        self.status
    }

    fn reset(&mut self) {
        self.q1.reset();
        self.q2.reset();
        self.q3.reset();
        self.q1_complete = false;
        self.q2_complete = false;
        self.q3_complete = false;
        self.status = QuestStatus::Ongoing;
    }
}
