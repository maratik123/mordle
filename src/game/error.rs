use crate::attempt::AttemptError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum GameError {
    TriesExhausted,
    AttemptError(AttemptError),
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
            GameError::TriesExhausted => write!(f, "Tries exhausted"),
            GameError::AttemptError(attemptError) => write!(f, "Attempt error: {}", attemptError),
        }
    }
}
