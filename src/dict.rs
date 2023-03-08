use std::collections::{HashMap, HashSet};

const DICT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/mordle-dict.txt"));

pub struct Dict {
    words: Vec<&'static str>,
    words_set: HashSet<&'static str>,
}

impl Default for Dict {
    fn default() -> Self {
        let mut words: Vec<_> = DICT.lines().collect();
        words.sort_unstable();
        let words_set: HashSet<_> = words.iter().copied().collect();
        Self { words, words_set }
    }
}

impl Dict {
    fn char_stat(words: impl IntoIterator<Item = &'static str>) -> HashMap<char, usize> {
        words
            .into_iter()
            .flat_map(|s| s.chars())
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut stat_v: Vec<_> = stat.iter().collect();
        stat_v.sort_unstable_by(|(a_char, a_cnt), (b_char, b_cnt)| {
            a_cnt.cmp(b_cnt).reverse().then(a_char.cmp(b_char))
        });
        println!("{stat_v:?}");
    }

    #[test]
    fn word_stat() {
        let dict = Dict::default();
        let stat = Dict::char_stat(dict.words.iter().copied());

        let mut word_score: Vec<_> = dict
            .words
            .iter()
            .map(|word| (word, word.chars().collect::<Vec<_>>()))
            .filter(|(_, chars)| {
                let mut found_chars = HashSet::with_capacity(chars.len());
                chars.iter().all(|c| found_chars.insert(c))
            })
            .map(|(word, chars)| {
                let score: usize = chars.iter().filter_map(|ch| stat.get(ch)).sum();
                (word, score)
            })
            .collect();
        word_score.sort_unstable_by(|(a_word, a_score), (b_word, b_score)| {
            a_score.cmp(b_score).reverse().then(a_word.cmp(b_word))
        });
        println!("{:?}", word_score.iter().take(20).collect::<Vec<_>>());
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
}
