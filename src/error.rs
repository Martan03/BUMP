use std::fmt::Display;

pub enum Error {
    IO(std::io::Error),
    Tags(audiotags::Error),
    Serde(serde_json::Error),
    Raplay(raplay::Error),
    Msg(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<audiotags::Error> for Error {
    fn from(value: audiotags::Error) -> Self {
        Self::Tags(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<raplay::Error> for Error {
    fn from(value: raplay::Error) -> Self {
        Self::Raplay(value)
    }
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "{e}"),
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::Tags(e) => write!(f, "{e}"),
            Error::Serde(e) => write!(f, "{e}"),
            Error::Raplay(e) => write!(f, "{e}"),
        }
    }
}
