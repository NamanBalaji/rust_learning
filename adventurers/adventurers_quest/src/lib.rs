#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing,
}

pub trait Quest<Event>: std::fmt::Display + std::fmt::Debug {
    /// Call this when something happens in the game
    fn register_event(&mut self, event: &Event) -> QuestStatus;

    /// Reset the quest to initial state
    fn reset(&mut self);
}
