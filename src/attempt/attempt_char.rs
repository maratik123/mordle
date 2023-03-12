use crate::{attempt::CharResult, CharPos, CharPositions};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct AttemptChar {
    pub ch: char,
    pub state: CharResult,
}

impl AttemptChar {
    pub fn test_char(char_positions: &CharPositions, ch: char, pos: CharPos) -> Self {
        Self {
            ch,
            state: match char_positions.positions(ch) {
                Some(set) if set.contains(&pos) => CharResult::Exact,
                Some(_) => CharResult::NotInPosition,
                None => CharResult::Unsuccessful,
            },
        }
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
        let word_index = CharPositions::from("сазан");
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
}
