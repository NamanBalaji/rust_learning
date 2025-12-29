use crate::blocks::Blocks;

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub block: Option<Blocks>,
}
