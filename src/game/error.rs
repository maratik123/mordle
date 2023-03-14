use crate::attempt::AttemptError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Eq, PartialEq)]
pub enum GameError {
    TriesExhausted,
    AlreadyWin,
    AttemptError(AttemptError),
    GameWordNotInDict,
}

impl Error for GameError {}

impl From<AttemptError> for GameError {
    #[inline]
    fn from(value: AttemptError) -> Self {
        Self::AttemptError(value)
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TriesExhausted => write!(f, "Tries exhausted"),
            Self::AlreadyWin => write!(f, "Already win"),
            Self::AttemptError(attempt_error) => write!(f, "Attempt error: {attempt_error}"),
            Self::GameWordNotInDict => write!(f, "Game initiated with word not in dict"),
        }
    }
}
