use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Attempting to add component to an entity without calling create component entity")]
    CreateComponentNeverCalled,
    #[error("attempted to use a component that wasn't registered")]
    ComponentNotRegistered,
}
