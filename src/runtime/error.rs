use thiserror::Error;

#[derive(Error, Debug)]
pub enum TurnipRuntimeError {
    #[error(
        "Runtime has not been initialized. Please '.run()' the runtime before trying to use it."
    )]
    NotIntializedError(),
}
