use crate::CharPos;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
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
    pub fn new(word: &str) -> Self {
        word.chars().enumerate().fold(
            Self::default(),
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

    pub fn remove_char_at_pos(&mut self, ch: char, pos: CharPos) {
        if let Some(set) = self.index.get_mut(&ch) {
            if set.remove(&pos) && set.is_empty() {
                self.index.remove(&ch);
            }
        }
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
                index: [
                    ('с', [CharPos(0)].into()),
                    ('а', [CharPos(1), CharPos(3)].into()),
                    ('з', [CharPos(2)].into()),
                    ('н', [CharPos(4)].into()),
                ]
                .into(),
                word_len: 5
            }
        );
    }

    #[test]
    fn positions() {
        let word_index = CharPositions::new("сазан");
        assert_eq!(
            word_index.positions('а'),
            Some(&[CharPos(1), CharPos(3)].into())
        );
        assert_eq!(word_index.positions('б'), None);
    }

    #[test]
    fn len() {
        let word_index = CharPositions::new("сазан");
        assert_eq!(word_index.word_len(), 5);
    }
}
