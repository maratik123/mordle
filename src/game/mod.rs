mod error;
use crate::{game::error::GameError, Attempt, CharPositions, Dict};

struct Game<'a> {
    dict: &'a Dict,
    word_index: CharPositions,
    max_tries: usize,
    tries: Vec<Attempt>,
}

impl<'a> Game<'a> {
    fn new(dict: &'a Dict, word: &str, max_tries: usize) -> Self {
        Self {
            dict,
            word_index: word.into(),
            max_tries,
            tries: vec![],
        }
    }

    fn try_input(&mut self, input: &str) -> Result<(), GameError> {
        if self.tries.len() <= self.max_tries {
            let attempt =
                Attempt::inspect_input(&input.chars().collect::<Vec<_>>(), &self.word_index)?;
            self.tries.push(attempt);
            Ok(())
        } else {
            return Err(GameError::TriesExhausted);
        }
    }
}
