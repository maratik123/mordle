use crate::CharPos;
use std::collections::{HashMap, HashSet};

const DICT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/mordle-dict.txt"));

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct WordIndex(usize);

pub struct Dict {
    words: Vec<&'static str>,
    words_set: HashSet<&'static str>,
    global_char_index: HashMap<char, HashSet<WordIndex>>,
    _char_at_pos_index: HashMap<(CharPos, char), HashSet<WordIndex>>,
}

impl Default for Dict {
    fn default() -> Self {
        let mut words: Vec<_> = DICT.lines().collect();
        words.sort_unstable();

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
                    HashMap::<_, HashSet<_>>::new(),
                ),
                |(mut global_char_index, mut char_at_pos_index), (index, pos, ch)| {
                    global_char_index.entry(ch).or_default().insert(index);
                    char_at_pos_index
                        .entry((pos, ch))
                        .or_default()
                        .insert(index);
                    (global_char_index, char_at_pos_index)
                },
            );

        Self {
            words,
            words_set,
            global_char_index,
            _char_at_pos_index: char_at_pos_index,
        }
    }
}

impl Dict {
    fn _char_stat(words: impl IntoIterator<Item = &'static str>) -> HashMap<char, usize> {
        words
            .into_iter()
            .flat_map(|s| s.chars())
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            })
    }

    #[inline]
    pub fn words(&self) -> &[&'static str] {
        &self.words
    }

    pub fn word_in_dict(&self, word: &str) -> bool {
        self.words_set.contains(word)
    }

    #[inline]
    pub fn global_char_index(&self) -> &HashMap<char, HashSet<WordIndex>> {
        &self.global_char_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, iter::zip};

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
        let stat = Dict::_char_stat(dict.words.iter().copied());

        let mut stat_v: Vec<_> = stat.iter().map(|(&c, &u)| (c, u)).collect();
        stat_v.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then(a_char.cmp(b_char))
        });
        println!("{stat_v:?}");

        let mut index_size = dict
            .global_char_index()
            .iter()
            .map(|(&ch, set)| (ch, set.len()))
            .collect::<Vec<_>>();
        index_size.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then(a_char.cmp(b_char))
        });
        println!("{index_size:?}");

        assert_eq!(stat_v.len(), index_size.len());
        for ((s_char, s_cnt), (i_char, i_size)) in zip(
            stat_v.iter().copied().collect::<BTreeMap<_, _>>(),
            index_size.iter().copied().collect::<BTreeMap<_, _>>(),
        ) {
            assert_eq!(s_char, i_char);
            assert!(s_cnt >= i_size);
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn word_stat() {
        let dict = Dict::default();
        let stat = Dict::_char_stat(dict.words.iter().copied());

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

        fn sort_and_print<T, F>(word_score: &mut [T], f: F)
        where
            T: Debug + Copy,
            F: FnMut(&T, &T) -> Ordering,
        {
            word_score.sort_unstable_by(f);
            println!(
                "{:?}",
                word_score.iter().copied().take(20).collect::<Vec<_>>()
            );
        }

        sort_and_print(
            &mut word_score,
            |(a_word, a_char_count_in_words, a_words_with_char),
             (b_word, b_char_count_in_words, b_words_with_char)| {
                (a_char_count_in_words, a_words_with_char)
                    .cmp(&(b_char_count_in_words, b_words_with_char))
                    .reverse()
                    .then(a_word.cmp(b_word))
            },
        );
        sort_and_print(
            &mut word_score,
            |(a_word, a_char_count_in_words, a_words_with_char),
             (b_word, b_char_count_in_words, b_words_with_char)| {
                (a_words_with_char, a_char_count_in_words)
                    .cmp(&(b_words_with_char, b_char_count_in_words))
                    .reverse()
                    .then(a_word.cmp(b_word))
            },
        );
    }

    #[test]
    fn char_stat_contains_all_letters() {
        let dict = Dict::default();
        let mut stat = Dict::_char_stat(dict.words.iter().copied());

        for ch in 'а'..='я' {
            assert!(matches!(stat.get(&ch), Some(&n) if n > 0));
            stat.remove(&ch);
        }
        assert_eq!(stat, HashMap::new());
    }
}
