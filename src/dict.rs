use crate::CharPos;
use std::collections::{HashMap, HashSet};

const DICT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/mordle-dict.txt"));

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct WordIndex(pub usize);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Dict {
    words: Vec<&'static str>,
    words_set: HashSet<&'static str>,
    global_char_index: HashMap<char, HashSet<WordIndex>>,
    char_at_pos_index: HashMap<CharPos, HashMap<char, HashSet<WordIndex>>>,
}

impl Default for Dict {
    fn default() -> Self {
        let mut words: Vec<_> = DICT.lines().collect();
        words.sort_unstable();

        Self::from_words_vec(words)
    }
}

impl FromIterator<&'static str> for Dict {
    fn from_iter<T: IntoIterator<Item = &'static str>>(iter: T) -> Self {
        Self::from_words_vec(iter.into_iter().collect())
    }
}

impl Dict {
    fn empty() -> Self {
        Self {
            words: Default::default(),
            words_set: Default::default(),
            global_char_index: Default::default(),
            char_at_pos_index: Default::default(),
        }
    }

    #[cfg(test)]
    fn char_stat(words: impl IntoIterator<Item = &'static str>) -> HashMap<char, usize> {
        words
            .into_iter()
            .flat_map(|s| s.chars())
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            })
    }

    pub fn from_words_vec(words: Vec<&'static str>) -> Self {
        let words_set: HashSet<_> = words.iter().copied().collect();

        let (global_char_index, char_at_pos_index) = words
            .iter()
            .enumerate()
            .map(|(index, word)| (WordIndex(index), word))
            .flat_map(|(index, word)| {
                word.chars()
                    .enumerate()
                    .map(move |(pos, ch)| (index, CharPos(pos), ch))
            })
            .fold(
                (
                    HashMap::<_, HashSet<_>>::new(),
                    HashMap::<_, HashMap<_, HashSet<_>>>::new(),
                ),
                |(mut global_char_index, mut char_at_pos_index), (index, pos, ch)| {
                    global_char_index.entry(ch).or_default().insert(index);
                    char_at_pos_index
                        .entry(pos)
                        .or_default()
                        .entry(ch)
                        .or_default()
                        .insert(index);
                    (global_char_index, char_at_pos_index)
                },
            );

        Self {
            words,
            words_set,
            global_char_index,
            char_at_pos_index,
        }
    }

    #[inline]
    pub fn words(&self) -> &[&'static str] {
        &self.words
    }

    #[inline]
    pub fn words_set(&self) -> &HashSet<&'static str> {
        &self.words_set
    }

    #[inline]
    pub fn global_char_index(&self) -> &HashMap<char, HashSet<WordIndex>> {
        &self.global_char_index
    }

    #[inline]
    pub fn char_at_pos_index(&self) -> &HashMap<CharPos, HashMap<char, HashSet<WordIndex>>> {
        &self.char_at_pos_index
    }

    pub fn word_in_dict(&self, word: &str) -> bool {
        self.words_set.contains(word)
    }

    fn word_index_by_pos_and_char(
        &self,
        pos: CharPos,
        chars: &HashSet<char>,
    ) -> Option<HashSet<WordIndex>> {
        self.char_at_pos_index.get(&pos).map(|word_index_by_char| {
            chars
                .iter()
                .filter_map(|ch| word_index_by_char.get(ch))
                .flat_map(|word_indices| word_indices.iter().copied())
                .collect::<HashSet<_>>()
        })
    }

    pub fn deny_chars_at_pos(&mut self, pos: CharPos, chars: &HashSet<char>) {
        match self.word_index_by_pos_and_char(pos, chars) {
            Some(word_indices_to_remove) if !word_indices_to_remove.is_empty() => {
                *self = self
                    .words
                    .iter()
                    .enumerate()
                    .map(|(index, &s)| (WordIndex(index), s))
                    .filter(|(word_index, _)| !word_indices_to_remove.contains(word_index))
                    .map(|(_, s)| s)
                    .collect()
            }
            _ => {}
        }
    }

    pub fn only_chars_at_pos(&mut self, pos: CharPos, chars: &HashSet<char>) {
        *self = match self.word_index_by_pos_and_char(pos, chars) {
            Some(word_indices_to_save) if !word_indices_to_save.is_empty() => Dict::from_words_vec(
                self.words
                    .iter()
                    .enumerate()
                    .map(|(index, &s)| (WordIndex(index), s))
                    .filter(|(word_index, _)| word_indices_to_save.contains(word_index))
                    .map(|(_, s)| s)
                    .collect(),
            ),
            _ => Dict::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cmp::Ordering,
        collections::BTreeMap,
        fmt::Debug,
        io,
        io::{BufWriter, Write},
        iter::zip,
    };

    #[test]
    fn words_contains_sazan() {
        assert!(matches!(
            Dict::default().words.binary_search(&"сазан"),
            Ok(_)
        ));
    }

    #[test]
    fn words_set_contains_sazan() {
        assert!(Dict::default().words_set.contains("сазан"));
    }

    #[test]
    fn char_stat() {
        let mut stdout = BufWriter::new(io::stdout().lock());
        let dict = Dict::default();
        let stat = Dict::char_stat(dict.words.iter().copied());

        let mut stat_v: Vec<_> = stat.iter().map(|(&c, &u)| (c, u)).collect();
        stat_v.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then_with(|| a_char.cmp(b_char))
        });
        writeln!(stdout, "{stat_v:?}").unwrap();

        let mut index_size = dict
            .global_char_index()
            .iter()
            .map(|(&ch, set)| (ch, set.len()))
            .collect::<Vec<_>>();
        index_size.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then_with(|| a_char.cmp(b_char))
        });
        writeln!(stdout, "{index_size:?}").unwrap();

        assert_eq!(stat_v.len(), index_size.len());
        for ((s_char, s_cnt), (i_char, i_size)) in zip(
            stat_v.iter().copied().collect::<BTreeMap<_, _>>(),
            index_size.iter().copied().collect::<BTreeMap<_, _>>(),
        ) {
            assert_eq!(s_char, i_char);
            assert!(s_cnt >= i_size);
        }
    }

    fn sort_and_print<T, F>(w: &mut impl Write, word_score: &mut [T], f: F)
    where
        T: Debug + Copy,
        F: FnMut(&T, &T) -> Ordering,
    {
        word_score.sort_unstable_by(f);
        writeln!(
            w,
            "{:?}",
            word_score.iter().copied().take(20).collect::<Vec<_>>()
        )
        .unwrap();
    }

    //noinspection DuplicatedCode
    #[test]
    fn word_stat() {
        let dict = Dict::default();
        let stat = Dict::char_stat(dict.words.iter().copied());

        let mut word_score: Vec<_> = dict
            .words
            .iter()
            .map(|&word| (word, word.chars().collect::<Vec<_>>()))
            .filter(|(_, chars)| {
                let mut found_chars = HashSet::with_capacity(chars.len());
                chars.iter().all(|&c| found_chars.insert(c))
            })
            .map(|(word, chars)| {
                let char_count_in_words: usize = chars.iter().filter_map(|ch| stat.get(ch)).sum();
                let words_with_char: usize = chars
                    .iter()
                    .filter_map(|ch| dict.global_char_index().get(ch))
                    .map(|set| set.len())
                    .sum();
                (word, char_count_in_words, words_with_char)
            })
            .collect();
        let mut stdout = BufWriter::new(io::stdout().lock());
        sort_and_print(
            &mut stdout,
            &mut word_score,
            |(a_word, a_char_count_in_words, a_words_with_char),
             (b_word, b_char_count_in_words, b_words_with_char)| {
                (a_char_count_in_words, a_words_with_char)
                    .cmp(&(b_char_count_in_words, b_words_with_char))
                    .reverse()
                    .then_with(|| a_word.cmp(b_word))
            },
        );
        sort_and_print(
            &mut stdout,
            &mut word_score,
            |(a_word, a_char_count_in_words, a_words_with_char),
             (b_word, b_char_count_in_words, b_words_with_char)| {
                (a_words_with_char, a_char_count_in_words)
                    .cmp(&(b_words_with_char, b_char_count_in_words))
                    .reverse()
                    .then_with(|| a_word.cmp(b_word))
            },
        );
    }

    #[test]
    fn char_stat_contains_all_letters() {
        let dict = Dict::default();
        let mut stat = Dict::char_stat(dict.words.iter().copied());

        for ch in 'а'..='я' {
            assert!(matches!(stat.get(&ch), Some(&n) if n > 0));
            stat.remove(&ch);
        }
        assert_eq!(stat, HashMap::new());
    }

    #[test]
    fn deny_chars_at_pos() {
        let mut dict = Dict::default();
        dict.deny_chars_at_pos(CharPos(0), &['а'].into());
        assert_eq!(
            dict.char_at_pos_index().get(&CharPos(0)).unwrap().get(&'а'),
            None
        );
    }

    #[test]
    fn deny_chars_at_pos_empty() {
        let mut dict = Dict::default();
        let old_dict = dict.clone();
        dict.deny_chars_at_pos(CharPos(0), &HashSet::new());
        assert_eq!(old_dict, dict);
    }

    #[test]
    fn only_chars_at_pos() {
        let mut dict = Dict::default();
        dict.only_chars_at_pos(CharPos(0), &['а'].into());
        for ch in 'б'..='я' {
            assert_eq!(
                dict.char_at_pos_index().get(&CharPos(0)).unwrap().get(&ch),
                None
            );
        }
        assert!(matches!(
            dict.char_at_pos_index().get(&CharPos(0)).unwrap().get(&'а'),
            Some(set) if !set.is_empty()
        ));
    }

    #[test]
    fn only_chars_at_pos_empty() {
        let mut dict = Dict::default();
        dict.only_chars_at_pos(CharPos(0), &HashSet::new());
        assert_eq!(dict, Dict::empty());
    }
}
