use std::collections::{HashMap, HashSet};

const DICT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/mordle-dict.txt"));

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
    fn char_stat_contains_all_letters() {
        let mut stat = Dict::char_stat(Dict::default().words);

        let mut stat_v: Vec<_> = stat.iter().collect();
        stat_v.sort_unstable_by_key(|(&ch, &n)| (n, ch));
        println!("{stat_v:?}");

        for ch in 'а'..='я' {
            assert!(matches!(stat.get(&ch), Some(&n) if n > 0));
            stat.remove(&ch);
        }
        assert_eq!(stat, HashMap::new());
    }
}
