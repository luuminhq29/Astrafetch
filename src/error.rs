use thiserror::Error;

#[derive(Debug, Error)]
pub enum AstraError {
    #[error("invalid configuration: {0}")]
    Config(String),
    #[error("terminal error: {0}")]
    Terminal(String),
    #[error("system information error: {0}")]
    System(String),
}
