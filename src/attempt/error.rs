use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Eq, PartialEq, Debug)]
pub enum AttemptError {
    InputLengthMismatch,
    WordNotInDict,
}

impl Display for AttemptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InputLengthMismatch => "Input string length not matched to word",
                Self::WordNotInDict => "Word not in dictionary",
            }
        )
    }
}

impl Error for AttemptError {}
