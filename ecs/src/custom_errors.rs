use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Attempting to add components to an entity without calling create component first")]
    CreateComponentNeverCalled,
    #[error("attempting to insert data for component that wasn't registered")]
    ComponentNotRegistered,
}
