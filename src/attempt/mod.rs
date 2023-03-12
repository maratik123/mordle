mod attempt_char;
mod char_result;
mod error;

pub use attempt_char::AttemptChar;
pub use char_result::CharResult;
pub use error::AttemptError;

use crate::{CharPos, CharPositions};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Attempt(pub Vec<AttemptChar>);

impl Attempt {
    pub fn inspect_input(
        input: &[char],
        char_positions: &CharPositions,
    ) -> Result<Self, AttemptError> {
        if input.len() == char_positions.word_len() {
            Ok(Self(
                input
                    .iter()
                    .enumerate()
                    .map(|(pos, &ch)| AttemptChar::test_char(char_positions, ch, CharPos(pos)))
                    .collect(),
            ))
        } else {
            Err(AttemptError::InputLengthMismatch)
        }
    }
}

impl Display for Attempt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Attempt(chars) = self;
        for ch in chars {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_attempt() {
        let attempt = Attempt(vec![
            AttemptChar {
                ch: 'а',
                state: CharResult::Exact,
            },
            AttemptChar {
                ch: 'б',
                state: CharResult::NotInPosition,
            },
            AttemptChar {
                ch: 'в',
                state: CharResult::Unsuccessful,
            },
            AttemptChar {
                ch: 'г',
                state: CharResult::Exact,
            },
            AttemptChar {
                ch: 'д',
                state: CharResult::NotInPosition,
            },
        ]);
        assert_eq!("а+б?в г+д?", attempt.to_string());
    }

    #[test]
    fn inspect_input() {
        assert_eq!(
            Attempt::inspect_input(
                &"казна".chars().collect::<Vec<_>>(),
                &CharPositions::from("сазан")
            ),
            Ok(Attempt(vec![
                AttemptChar {
                    ch: 'к',
                    state: CharResult::Unsuccessful,
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Exact,
                },
                AttemptChar {
                    ch: 'з',
                    state: CharResult::Exact,
                },
                AttemptChar {
                    ch: 'н',
                    state: CharResult::NotInPosition,
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::NotInPosition,
                },
            ]))
        );
    }

    #[test]
    fn test_input_mismatch_len() {
        assert_eq!(
            Attempt::inspect_input(
                &"топ".chars().collect::<Vec<_>>(),
                &CharPositions::from("сазан")
            ),
            Err(AttemptError::InputLengthMismatch)
        );
    }
}
