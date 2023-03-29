use crate::CharPos;
use itertools::Itertools;
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
        let mut words = DICT.lines().collect_vec();
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
    pub fn empty() -> Self {
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

    pub fn deny_chars_at_poses(&mut self, poses: &HashSet<CharPos>, chars: &HashSet<char>) {
        self.deny_chars_helper(self.word_index_by_poses_and_chars(poses, chars));
    }

    pub fn only_chars_at_poses(&mut self, poses: &HashSet<CharPos>, chars: &HashSet<char>) {
        self.only_chars_helper(self.word_index_by_poses_and_chars(poses, chars));
    }

    pub fn deny_chars(&mut self, chars: &HashSet<char>) {
        self.deny_chars_helper(self.word_index_by_char(chars));
    }

    fn deny_chars_helper(&mut self, word_indices_to_remove: HashSet<WordIndex>) {
        if !word_indices_to_remove.is_empty() {
            *self = self.remove_indices(word_indices_to_remove);
        }
    }

    pub fn only_chars(&mut self, chars: &HashSet<char>) {
        self.only_chars_helper(self.word_index_by_char(chars));
    }

    fn only_chars_helper(&mut self, word_indices_to_save: HashSet<WordIndex>) {
        *self = if word_indices_to_save.is_empty() {
            Dict::empty()
        } else {
            self.save_indices(word_indices_to_save)
        }
    }

    fn word_index_by_poses_and_chars(
        &self,
        poses: &HashSet<CharPos>,
        chars: &HashSet<char>,
    ) -> HashSet<WordIndex> {
        poses
            .iter()
            .filter_map(|pos| self.char_at_pos_index.get(pos))
            .flat_map(|word_index_by_char| {
                chars
                    .iter()
                    .filter_map(|ch| word_index_by_char.get(ch))
                    .flat_map(|word_indices| word_indices.iter().copied())
            })
            .collect()
    }

    fn word_index_by_char(&self, chars: &HashSet<char>) -> HashSet<WordIndex> {
        chars
            .iter()
            .filter_map(|ch| self.global_char_index.get(ch))
            .flat_map(|word_indices| word_indices.iter().copied())
            .collect()
    }

    fn remove_indices(&self, word_indices_to_remove: HashSet<WordIndex>) -> Self {
        self.words
            .iter()
            .enumerate()
            .map(|(index, &s)| (WordIndex(index), s))
            .filter(|(word_index, _)| !word_indices_to_remove.contains(word_index))
            .map(|(_, s)| s)
            .collect()
    }

    fn save_indices(&self, word_indices_to_save: HashSet<WordIndex>) -> Self {
        self.words
            .iter()
            .enumerate()
            .map(|(index, &s)| (WordIndex(index), s))
            .filter(|(word_index, _)| word_indices_to_save.contains(word_index))
            .map(|(_, s)| s)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
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
        let dict = Dict::default();
        let stat = Dict::char_stat(dict.words.iter().copied());

        let mut stat_v = stat.iter().map(|(&c, &u)| (c, u)).collect_vec();
        stat_v.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then_with(|| a_char.cmp(b_char))
        });
        let mut stdout = BufWriter::new(io::stdout().lock());
        writeln!(stdout, "{stat_v:?}").unwrap();

        let mut index_size = dict
            .global_char_index()
            .iter()
            .map(|(&ch, set)| (ch, set.len()))
            .collect_vec();
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
        writeln!(w, "{:?}", word_score.iter().copied().take(20).collect_vec()).unwrap();
    }

    //noinspection DuplicatedCode
    #[test]
    fn word_stat() {
        let dict = Dict::default();
        let stat = Dict::char_stat(dict.words.iter().copied());

        let mut word_score = dict
            .words
            .iter()
            .map(|&word| (word, word.chars().collect_vec()))
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
            .collect_vec();
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
        dict.deny_chars_at_poses(&[CharPos(0)].into(), &['а'].into());
        assert_eq!(
            dict.char_at_pos_index().get(&CharPos(0)).unwrap().get(&'а'),
            None
        );
    }

    #[test]
    fn deny_chars_at_pos_empty() {
        let mut dict = Dict::default();
        let old_dict = dict.clone();
        dict.deny_chars_at_poses(&[CharPos(0)].into(), &HashSet::new());
        assert_eq!(old_dict, dict);
    }

    #[test]
    fn deny_chars() {
        let mut dict = Dict::default();
        dict.deny_chars(&['а'].into());
        assert_eq!(dict.global_char_index.get(&'а'), None);
    }

    #[test]
    fn deny_chars_empty() {
        let mut dict = Dict::default();
        let old_dict = dict.clone();
        dict.deny_chars(&[].into());
        assert_eq!(old_dict, dict);
    }

    #[test]
    fn only_chars_at_pos() {
        let mut dict = Dict::default();
        dict.only_chars_at_poses(&[CharPos(0)].into(), &['а'].into());
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
        dict.only_chars_at_poses(&[CharPos(0)].into(), &HashSet::new());
        assert_eq!(dict, Dict::empty());
    }

    #[test]
    fn only_chars() {
        let mut dict = Dict::default();
        dict.only_chars(&['а'].into());
        assert_eq!(
            dict.words
                .iter()
                .copied()
                .filter(|word| word.chars().any(|ch| ch == 'а'))
                .collect_vec(),
            dict.words
        );
        assert!(matches!(
            dict.global_char_index().get(&'а'),
            Some(set) if !set.is_empty()
        ));
    }

    #[test]
    fn only_chars_empty() {
        let mut dict = Dict::default();
        dict.only_chars(&HashSet::new());
        assert_eq!(dict, Dict::empty());
    }

    #[test]
    fn only_chars_with_deny_chars() {
        let ch = 'а';
        let mut dict = Dict::default();
        let chars: HashSet<_> = [ch].into();
        dict.only_chars(&chars);
        assert!(!dict.words.is_empty());
        dict.deny_chars(&chars);
        assert_eq!(dict, Dict::empty());
    }

    #[test]
    fn deny_chars_with_only_chars() {
        let ch = 'а';
        let mut dict = Dict::default();
        let chars: HashSet<_> = [ch].into();
        dict.deny_chars(&chars);
        assert!(!dict.words.is_empty());
        dict.only_chars(&chars);
        assert_eq!(dict, Dict::empty());
    }
}
