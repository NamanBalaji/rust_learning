use std::fmt::Display;

use adventurers_quest::{Quest, QuestStatus};

use crate::{blocks::Blocks, quests::game_event::GameEvent};

#[derive(Debug)]
pub struct WalkQuest {
    /// The target block to walk to
    target_block: Blocks,
    /// The status of the walk quest
    status: QuestStatus,
}

impl WalkQuest {
    /// Create a new walk quest given the target block
    ///
    /// The quest starts as ongoing
    pub fn new(target_block: Blocks) -> Self {
        Self {
            target_block,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for WalkQuest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            QuestStatus::Complete => write!(f, "[✅] Walk on a {} block", self.target_block),
            QuestStatus::Ongoing => write!(f, "[ ] Walk on a {} block", self.target_block),
        }
    }
}

impl Quest<GameEvent> for WalkQuest {
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        if self.status != QuestStatus::Complete {
            if let Some(block) = &event.block {
                if *block == self.target_block {
                    self.status = QuestStatus::Complete;
                }
            }
        }

        self.status
    }

    fn reset(&mut self) {
        self.status = QuestStatus::Ongoing;
    }
}
