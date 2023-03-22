use crate::attempt::error::ParseAttemptError;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum CharResult {
    Exact,
    NotInPosition,
    Unsuccessful,
}

impl Display for CharResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Exact => '+',
                Self::NotInPosition => '?',
                Self::Unsuccessful => ' ',
            }
        )
    }
}

impl TryFrom<char> for CharResult {
    type Error = ParseAttemptError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(match ch {
            '+' => Self::Exact,
            '?' => Self::NotInPosition,
            ' ' => Self::Unsuccessful,
            ch => return Err(ParseAttemptError::CharResultUnexpected(ch)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_exact() {
        assert_eq!("+", CharResult::Exact.to_string());
    }

    #[test]
    fn display_not_in_position() {
        assert_eq!("?", CharResult::NotInPosition.to_string());
    }

    #[test]
    fn display_unsuccessful() {
        assert_eq!(" ", CharResult::Unsuccessful.to_string());
    }

    #[test]
    fn try_from_exact() {
        assert_eq!(Ok(CharResult::Exact), '+'.try_into());
    }

    #[test]
    fn try_from_not_in_position() {
        assert_eq!(Ok(CharResult::NotInPosition), '?'.try_into());
    }

    #[test]
    fn try_from_unsuccessful() {
        assert_eq!(Ok(CharResult::Unsuccessful), ' '.try_into());
    }

    #[test]
    fn try_from_err() {
        assert_eq!(
            Err::<CharResult, _>(ParseAttemptError::CharResultUnexpected('а')),
            'а'.try_into()
        );
    }
}
