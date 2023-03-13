mod error;
mod status;

pub use error::GameError;
pub use status::GameFinishStatus;

use crate::{Attempt, CharPositions, Dict};

pub struct Game<'a> {
    dict: &'a Dict,
    word_index: CharPositions,
    max_tries: usize,
    tries: Vec<Attempt>,
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dict, word: &str, max_tries: usize) -> Result<Self, GameError> {
        if dict.word_in_dict(word) {
            Ok(Self {
                dict,
                word_index: word.into(),
                max_tries,
                tries: vec![],
            })
        } else {
            Err(GameError::GameWordNotInDict)
        }
    }

    pub fn try_input(&mut self, input: &str) -> Result<&Attempt, GameError> {
        match self.finish_status() {
            None => {
                let attempt = Attempt::inspect_input(input, &self.word_index, self.dict)?;
                self.tries.push(attempt);
                Ok(self.tries.last().unwrap_or_else(|| unreachable!()))
            }
            Some(GameFinishStatus::Win) => Err(GameError::AlreadyWin),
            Some(GameFinishStatus::Fail) => Err(GameError::TriesExhausted),
        }
    }

    #[inline]
    pub fn finish_status(&self) -> Option<GameFinishStatus> {
        if self.tries.len() > self.max_tries {
            Some(GameFinishStatus::Fail)
        } else {
            self.tries
                .last()
                .filter(|attempt| attempt.is_win_attempt())
                .map(|_| GameFinishStatus::Win)
                .or_else(|| {
                    Some(GameFinishStatus::Fail).filter(|_| self.tries.len() == self.max_tries)
                })
        }
    }

    #[inline]
    pub fn max_tries(&self) -> usize {
        self.max_tries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attempt::AttemptError;

    #[test]
    fn new_ok() {
        let dict = Dict::default();
        assert_eq!(Game::new(&dict, "сазан", 5).map(|_| ()), Ok(()));
    }

    #[test]
    fn new_not_in_dict() {
        let dict = Dict::default();
        assert_eq!(
            Game::new(&dict, "абвгд", 5).map(|_| ()),
            Err(GameError::GameWordNotInDict)
        );
    }

    #[test]
    fn try_input_first() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 5).unwrap();
        let old_game_tries = game.tries.clone();
        assert_eq!(game.try_input("казан").map(|_| ()), Ok(()));
        assert_eq!(game.finish_status(), None);
        assert_ne!(game.tries, old_game_tries);
    }

    #[test]
    fn try_input_size_mismatch() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 5).unwrap();
        let old_game_tries = game.tries.clone();
        assert_eq!(
            game.try_input("абвг"),
            Err(GameError::AttemptError(AttemptError::InputLengthMismatch))
        );
        assert_eq!(game.finish_status(), None);
        assert_eq!(game.tries, old_game_tries);
    }

    #[test]
    fn try_input_not_in_dict() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 5).unwrap();
        let old_game_tries = game.tries.clone();
        assert_eq!(
            game.try_input("абвгд"),
            Err(GameError::AttemptError(AttemptError::WordNotInDict))
        );
        assert_eq!(game.finish_status(), None);
        assert_eq!(game.tries, old_game_tries);
    }

    #[test]
    fn try_input_success() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 5).unwrap();
        let old_game_tries = game.tries.clone();
        assert_eq!(game.try_input("сазан").map(|_| ()), Ok(()));
        assert_eq!(game.finish_status(), Some(GameFinishStatus::Win));
        assert_ne!(game.tries, old_game_tries);
        assert!(game.tries.last().unwrap().is_win_attempt());
    }

    #[test]
    fn try_input_prev_success() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 5).unwrap();
        let old_game_tries = game.tries.clone();
        assert_eq!(game.try_input("сазан").map(|_| ()), Ok(()));
        assert_eq!(game.finish_status(), Some(GameFinishStatus::Win));
        assert_ne!(game.tries, old_game_tries);
        assert!(game.tries.last().unwrap().is_win_attempt());
        let old_game_tries = game.tries.clone();
        assert_eq!(game.try_input("фазан"), Err(GameError::AlreadyWin));
        assert_eq!(game.finish_status(), Some(GameFinishStatus::Win));
        assert_eq!(game.tries, old_game_tries);
        assert!(game.tries.last().unwrap().is_win_attempt());
    }

    #[test]
    fn tries_exhausted() {
        let dict = Dict::default();
        let mut game = Game::new(&dict, "сазан", 2).unwrap();
        assert_eq!(game.try_input("фазан").map(|_| ()), Ok(()));
        assert_eq!(game.try_input("бедро").map(|_| ()), Ok(()));
        assert_eq!(game.finish_status(), Some(GameFinishStatus::Fail));
        assert_eq!(game.try_input("сазан"), Err(GameError::TriesExhausted));
    }
}
