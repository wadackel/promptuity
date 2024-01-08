use thiserror::Error;

/// The error type for promptuity.
#[derive(Error, Debug)]
pub enum Error {
    /// An error representing [`std::io::Error`].
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// An error representing the cancellation of prompt execution.
    #[error("Operation canceled")]
    Cancel,
    /// An error representing invalid configuration in a prompt.
    #[error("Config error: {0}")]
    Config(String),
    /// An unknown error originating from the prompt.
    #[error("Prompt error: {0}")]
    Prompt(String),
}
