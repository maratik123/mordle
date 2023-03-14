mod error;
mod status;

pub use error::GameError;
pub use status::GameFinishStatus;
use std::collections::BTreeSet;
use std::io::{BufRead, Write};

use crate::{Attempt, CharPositions, CharResult, Dict, LineReader};

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
        } else if let Some(true) = self.tries.last().map(|attempt| attempt.is_win_attempt()) {
            Some(GameFinishStatus::Win)
        } else if self.tries.len() == self.max_tries {
            Some(GameFinishStatus::Fail)
        } else {
            None
        }
    }

    #[inline]
    pub fn max_tries(&self) -> usize {
        self.max_tries
    }

    pub fn main_loop(
        &mut self,
        r: &mut impl BufRead,
        w: &mut impl Write,
    ) -> Result<GameFinishStatus, GameError> {
        let mut line_reader = LineReader::default();
        let mut avail_chars: BTreeSet<_> = self.dict.global_char_index().keys().copied().collect();
        for t in 1usize.. {
            loop {
                write!(w, "Available chars: ")?;
                for ch in &avail_chars {
                    write!(w, "{ch}")?;
                }
                writeln!(w)?;

                write!(w, "Enter try {t} of {}: ", self.max_tries())?;
                w.flush()?;

                match self.try_input(line_reader.read(r)?.trim()) {
                    Ok(attempt) => {
                        let Attempt(attempt_chars) = attempt;
                        for ch in attempt_chars
                            .iter()
                            .filter(|attempt_char| attempt_char.state == CharResult::Unsuccessful)
                            .map(|attempt_char| attempt_char.ch)
                        {
                            avail_chars.remove(&ch);
                        }
                        writeln!(w, "{attempt}")?;
                        break;
                    }
                    Err(err @ GameError::AttemptError(_)) => {
                        writeln!(w, "{err}")?;
                    }
                    other => {
                        other?;
                    }
                }
            }
            if let Some(status) = self.finish_status() {
                return Ok(status);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attempt::AttemptError;
    use std::io::Cursor;

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
            Err(AttemptError::InputLengthMismatch.into())
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
            Err(AttemptError::WordNotInDict.into())
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

    #[test]
    fn main_loop_win() {
        let dict = Dict::default();
        let mut out = vec![];
        let word = "сазан";
        let mut inp = Cursor::new("сазан\n");
        let mut game = Game::new(&dict, word, 6).unwrap();
        assert_eq!(
            game.main_loop(&mut inp, &mut out),
            Ok(GameFinishStatus::Win)
        );
        assert_eq!(
            String::from_utf8(out).unwrap().as_str(),
            "\
            Available chars: абвгдежзийклмнопрстуфхцчшщъыьэюя\n\
            Enter try 1 of 6: с+а+з+а+н+\n\
            "
        );
    }

    #[test]
    fn main_loop_win_at_edge() {
        let dict = Dict::default();
        let mut out = vec![];
        let word = "сазан";
        let mut inp = Cursor::new("казан\nсазан\n");
        let mut game = Game::new(&dict, word, 2).unwrap();
        assert_eq!(
            game.main_loop(&mut inp, &mut out),
            Ok(GameFinishStatus::Win)
        );
        assert_eq!(
            String::from_utf8(out).unwrap().as_str(),
            "\
            Available chars: абвгдежзийклмнопрстуфхцчшщъыьэюя\n\
            Enter try 1 of 2: к а+з+а+н+\n\
            Available chars: абвгдежзийлмнопрстуфхцчшщъыьэюя\n\
            Enter try 2 of 2: с+а+з+а+н+\n\
            "
        );
    }

    #[test]
    fn main_loop_fail_after_edge() {
        let dict = Dict::default();
        let mut out = vec![];
        let word = "сазан";
        let mut inp = Cursor::new("казан\nфазан\nсазан\n");
        let mut game = Game::new(&dict, word, 2).unwrap();
        assert_eq!(
            game.main_loop(&mut inp, &mut out),
            Ok(GameFinishStatus::Fail)
        );
        assert_eq!(
            String::from_utf8(out).unwrap().as_str(),
            "\
            Available chars: абвгдежзийклмнопрстуфхцчшщъыьэюя\n\
            Enter try 1 of 2: к а+з+а+н+\n\
            Available chars: абвгдежзийлмнопрстуфхцчшщъыьэюя\n\
            Enter try 2 of 2: ф а+з+а+н+\n\
            "
        );
    }

    #[test]
    fn main_loop_fail() {
        let dict = Dict::default();
        let mut out = vec![];
        let word = "сазан";
        let mut inp = Cursor::new("казан\nфазан\n");
        let mut game = Game::new(&dict, word, 2).unwrap();
        assert_eq!(
            game.main_loop(&mut inp, &mut out),
            Ok(GameFinishStatus::Fail)
        );
        assert_eq!(
            String::from_utf8(out).unwrap().as_str(),
            "\
            Available chars: абвгдежзийклмнопрстуфхцчшщъыьэюя\n\
            Enter try 1 of 2: к а+з+а+н+\n\
            Available chars: абвгдежзийлмнопрстуфхцчшщъыьэюя\n\
            Enter try 2 of 2: ф а+з+а+н+\n\
            "
        );
    }
}
