#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Tags(#[from] audiotags::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Raplay(#[from] raplay::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error("{0}")]
    Msg(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Msg(value)
    }
}
