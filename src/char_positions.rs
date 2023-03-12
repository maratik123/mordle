use crate::CharPos;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Debug)]
pub struct CharPositions {
    index: HashMap<char, HashSet<CharPos>>,
    word_len: usize,
}

impl From<&str> for CharPositions {
    #[inline]
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl CharPositions {
    fn new(word: &str) -> Self {
        word.chars().enumerate().fold(
            Self {
                index: HashMap::new(),
                word_len: Default::default(),
            },
            |Self {
                 mut index,
                 word_len: _,
             },
             (pos, ch)| {
                index.entry(ch).or_default().insert(CharPos(pos));
                Self {
                    index,
                    word_len: pos + 1,
                }
            },
        )
    }

    pub fn positions(&self, ch: char) -> Option<&HashSet<CharPos>> {
        self.index.get(&ch)
    }

    #[inline]
    pub fn word_len(&self) -> usize {
        self.word_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            CharPositions::new("сазан"),
            CharPositions {
                index: HashMap::from([
                    ('с', HashSet::from([CharPos(0)])),
                    ('а', HashSet::from([CharPos(1), CharPos(3)])),
                    ('з', HashSet::from([CharPos(2)])),
                    ('н', HashSet::from([CharPos(4)])),
                ]),
                word_len: 5
            }
        );
    }

    #[test]
    fn positions() {
        let word_index = CharPositions::new("сазан");
        assert_eq!(
            word_index.positions('а'),
            Some(&HashSet::from([CharPos(1), CharPos(3)]))
        );
        assert_eq!(word_index.positions('б'), None);
    }

    #[test]
    fn len() {
        let word_index = CharPositions::new("сазан");
        assert_eq!(word_index.word_len(), 5);
    }
}
