use crate::attempt::AttemptError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum GameError {
    TriesExhausted,
    AlreadyWin,
    AttemptError(AttemptError),
    GameWordNotInDict,
    IoError(io::Error),
}

#[cfg(test)]
impl PartialEq for GameError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::TriesExhausted => matches!(other, Self::TriesExhausted),
            Self::AlreadyWin => matches!(other, Self::AlreadyWin),
            Self::AttemptError(e) => matches!(other, Self::AttemptError(oe) if e == oe),
            Self::GameWordNotInDict => matches!(other, Self::GameWordNotInDict),
            Self::IoError(io) => {
                matches!(other, Self::IoError(other_io) if io.kind() == other_io.kind())
            }
        }
    }
}

impl Error for GameError {}

impl From<AttemptError> for GameError {
    #[inline]
    fn from(value: AttemptError) -> Self {
        Self::AttemptError(value)
    }
}

impl From<io::Error> for GameError {
    #[inline]
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TriesExhausted => write!(f, "Tries exhausted"),
            Self::AlreadyWin => write!(f, "Already win"),
            Self::AttemptError(attempt_error) => write!(f, "Attempt error: {attempt_error}"),
            Self::GameWordNotInDict => write!(f, "Game initiated with word not in dict"),
            Self::IoError(err) => write!(f, "I/O Error: {err}"),
        }
    }
}
