use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub enum AttemptError {
    InputLengthMismatch,
}

impl Display for AttemptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AttemptError::InputLengthMismatch => "Input string length not matched to word",
            }
        )
    }
}

impl Error for AttemptError {}
