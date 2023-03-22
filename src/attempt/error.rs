use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Eq, PartialEq, Debug)]
pub enum ParseAttemptError {
    CharResultUnexpected(char),
}

impl Display for ParseAttemptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharResultUnexpected(ch) => write!(f, "Unexpected char result: '{ch}'"),
        }
    }
}

impl Error for ParseAttemptError {}

#[derive(Eq, PartialEq, Debug)]
pub enum AttemptError {
    InputLengthMismatch,
    WordNotInDict,
    ParseError(ParseAttemptError),
}

impl Display for AttemptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputLengthMismatch => write!(f, "Input string length not matched to word"),
            Self::WordNotInDict => write!(f, "Word not in dictionary"),
            Self::ParseError(parse_attempt_error) => {
                write!(f, "Parse error: {parse_attempt_error}")
            }
        }
    }
}

impl Error for AttemptError {}
