mod attempt_char;
mod char_result;
mod error;

pub use attempt_char::AttemptChar;
pub use char_result::CharResult;
pub use error::AttemptError;

use crate::{attempt::error::ParseAttemptError, CharPos, CharPositions, Dict};
use itertools::Itertools;
use std::{
    fmt::{Display, Formatter},
    iter::zip,
    str::FromStr,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Attempt(pub Vec<AttemptChar>);

impl FromStr for Attempt {
    type Err = ParseAttemptError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Attempt(
            s.chars()
                .chunks(2)
                .into_iter()
                .map(|mut chunk| {
                    match (chunk.next(), chunk.next()) {
                        (Some(ch), Some(state)) => (ch, state),
                        (Some(ch), None) => (ch, ' '),
                        _ => unreachable!(),
                    }
                    .try_into()
                })
                .try_collect()?,
        ))
    }
}

impl Attempt {
    pub fn inspect_input(
        input: &str,
        char_positions: &CharPositions,
        dict: &Dict,
    ) -> Result<Self, AttemptError> {
        let chars: Vec<_> = input.chars().collect();
        if chars.len() != char_positions.word_len() {
            Err(AttemptError::InputLengthMismatch)
        } else if !dict.word_in_dict(input) {
            Err(AttemptError::WordNotInDict)
        } else {
            let mut char_positions = char_positions.clone();
            let exact_chars: Vec<_> = chars
                .iter()
                .enumerate()
                .map(|(pos, &ch)| (CharPos(pos), ch))
                .map(
                    |(pos, ch)| match AttemptChar::test_char(&char_positions, ch, pos) {
                        attempt_char @ AttemptChar {
                            state: CharResult::Exact,
                            ..
                        } => {
                            char_positions.remove_char_at_pos(ch, pos);
                            Some(attempt_char)
                        }
                        _ => None,
                    },
                )
                .collect();
            Ok(Self(
                zip(chars, exact_chars)
                    .enumerate()
                    .map(|(pos, (ch, exact_char))| (CharPos(pos), ch, exact_char))
                    .map(|(pos, ch, exact_char)| match exact_char {
                        None => match AttemptChar::test_char(&char_positions, ch, pos) {
                            attempt_char @ AttemptChar {
                                state: CharResult::Unsuccessful,
                                ..
                            } => attempt_char,
                            attempt_char => {
                                char_positions.remove_char_at_pos(ch, pos);
                                attempt_char
                            }
                        },
                        Some(exact_char) => exact_char,
                    })
                    .collect(),
            ))
        }
    }

    #[inline]
    pub fn is_win_attempt(&self) -> bool {
        let Self(attempt_chars) = self;
        attempt_chars.iter().all(|ac| ac.state == CharResult::Exact)
    }
}

impl Display for Attempt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(chars) = self;
        for ch in chars {
            write!(f, "{ch}")?;
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
            Attempt::inspect_input("казна", &"сазан".into(), &Dict::default()),
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
    fn inspect_input_same_letter() {
        assert_eq!(
            Attempt::inspect_input("парад", &"парус".into(), &Dict::default()),
            Ok(Attempt(vec![
                AttemptChar {
                    ch: 'п',
                    state: CharResult::Exact,
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Exact,
                },
                AttemptChar {
                    ch: 'р',
                    state: CharResult::Exact,
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Unsuccessful,
                },
                AttemptChar {
                    ch: 'д',
                    state: CharResult::Unsuccessful,
                },
            ]))
        );
    }

    #[test]
    fn test_input_mismatch_len() {
        assert_eq!(
            Attempt::inspect_input("топ", &"сазан".into(), &Dict::default()),
            Err(AttemptError::InputLengthMismatch)
        );
    }

    #[test]
    fn test_input_not_in_dict() {
        assert_eq!(
            Attempt::inspect_input("сазае", &"сазан".into(), &Dict::default()),
            Err(AttemptError::WordNotInDict)
        );
    }

    #[test]
    fn not_is_win_attempt() {
        assert!(
            !Attempt::inspect_input("казна", &"сазан".into(), &Dict::default())
                .unwrap()
                .is_win_attempt()
        );
    }

    #[test]
    fn is_win_attempt() {
        assert!(
            Attempt::inspect_input("сазан", &"сазан".into(), &Dict::default())
                .unwrap()
                .is_win_attempt()
        );
    }

    #[test]
    fn try_from() {
        assert_eq!(
            "с+а+з?а е ".parse(),
            Ok(Attempt(vec![
                AttemptChar {
                    ch: 'с',
                    state: CharResult::Exact
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Exact
                },
                AttemptChar {
                    ch: 'з',
                    state: CharResult::NotInPosition
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Unsuccessful
                },
                AttemptChar {
                    ch: 'е',
                    state: CharResult::Unsuccessful
                },
            ]))
        );
    }

    #[test]
    fn try_from_short() {
        assert_eq!(
            "с+а+з?а н".parse(),
            Ok(Attempt(vec![
                AttemptChar {
                    ch: 'с',
                    state: CharResult::Exact
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Exact
                },
                AttemptChar {
                    ch: 'з',
                    state: CharResult::NotInPosition
                },
                AttemptChar {
                    ch: 'а',
                    state: CharResult::Unsuccessful
                },
                AttemptChar {
                    ch: 'н',
                    state: CharResult::Unsuccessful
                },
            ]))
        );
    }

    #[test]
    fn try_from_err() {
        assert_eq!(
            "с+а+з?а нб".parse(),
            Err::<Attempt, _>(ParseAttemptError::CharResultUnexpected('б'))
        );
    }
}
