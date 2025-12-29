use std::fmt::Display;

use crate::{blocks::Blocks, quests::game_event::GameEvent};
use adventurers_quest::{Quest, QuestStatus};

#[derive(Debug)]
pub struct WalkRepeatQuest {
    /// The target block to walk on a certain consecutive number of times
    target_block: Blocks,
    /// The number of consecutive times the target block should be walked on to complete
    /// the quest
    target_count: u32,
    /// The number of times the target block has actually been walked on consecutively
    blocks_walked: u32,
    /// The status of the walk repeat quest
    status: QuestStatus,
}

impl WalkRepeatQuest {
    /// Create a new walk repeat quest given a target block and the number of times this block
    /// should be walked on consecutively
    pub fn new(target_block: Blocks, target_count: u32) -> Self {
        Self {
            target_block,
            target_count,
            blocks_walked: 0,
            status: if target_count == 0 {
                QuestStatus::Complete
            } else {
                QuestStatus::Ongoing
            },
        }
    }
}

impl Display for WalkRepeatQuest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            QuestStatus::Complete => write!(
                f,
                "[✅] Walk on exactly {} blocks of {} in a row",
                self.target_count, self.target_block
            ),
            QuestStatus::Ongoing => write!(
                f,
                "[ ] Walk on exactly {} blocks of {} in a row",
                self.target_count, self.target_block
            ),
        }
    }
}

impl Quest<GameEvent> for WalkRepeatQuest {
    fn register_event(&mut self, event: &GameEvent) -> QuestStatus {
        if self.status != QuestStatus::Complete {
            if let Some(block) = &event.block {
                if *block == self.target_block {
                    self.blocks_walked += 1;
                } else {
                    // a block was walked on, but it was not the target block
                    self.blocks_walked = 0;
                }
            } else {
                // no block was walked on
                self.blocks_walked = 0;
            }

            if self.blocks_walked == self.target_count {
                self.status = QuestStatus::Complete;
            }
        }

        self.status
    }

    fn reset(&mut self) {
        self.blocks_walked = 0;
        self.status = QuestStatus::Ongoing;
    }
}
