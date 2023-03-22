use crate::{
    attempt::{error::ParseAttemptError, CharResult},
    CharPos, CharPositions,
};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct AttemptChar {
    pub ch: char,
    pub state: CharResult,
}

impl AttemptChar {
    pub fn test_char(char_positions: &CharPositions, ch: char, pos: CharPos) -> Self {
        Self {
            ch,
            state: match char_positions.positions(ch) {
                Some(positions) if positions.contains(&pos) => CharResult::Exact,
                Some(positions) if positions.is_empty() => CharResult::Unsuccessful,
                Some(_) => CharResult::NotInPosition,
                None => CharResult::Unsuccessful,
            },
        }
    }
}

impl TryFrom<(char, char)> for AttemptChar {
    type Error = ParseAttemptError;

    fn try_from((ch, state): (char, char)) -> Result<Self, Self::Error> {
        Ok(AttemptChar {
            ch,
            state: state.try_into()?,
        })
    }
}

impl Display for AttemptChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.ch, self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_char() {
        let attempt_char = AttemptChar {
            ch: 'а',
            state: CharResult::Exact,
        };
        assert_eq!("а+", attempt_char.to_string());
    }

    #[test]
    fn test_char() {
        let word_index = CharPositions::new("сазан");
        assert_eq!(
            AttemptChar::test_char(&word_index, 'а', CharPos(1)),
            AttemptChar {
                ch: 'а',
                state: CharResult::Exact
            }
        );
        assert_eq!(
            AttemptChar::test_char(&word_index, 'з', CharPos(0)),
            AttemptChar {
                ch: 'з',
                state: CharResult::NotInPosition
            }
        );
        assert_eq!(
            AttemptChar::test_char(&word_index, 'б', CharPos(2)),
            AttemptChar {
                ch: 'б',
                state: CharResult::Unsuccessful
            }
        );
    }

    #[test]
    fn try_from() {
        assert_eq!(
            ('а', '+').try_into(),
            Ok(AttemptChar {
                ch: 'а',
                state: CharResult::Exact
            })
        );
    }

    #[test]
    fn try_from_err() {
        assert_eq!(
            ('а', 'б').try_into(),
            Err::<AttemptChar, _>(ParseAttemptError::CharResultUnexpected('б'))
        )
    }
}
