use crate::{CharPos, Dict};
use num_rational::Ratio;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

pub fn pos_stats(dict: &Dict) -> HashMap<CharPos, HashMap<char, usize>> {
    dict.char_at_pos_index()
        .iter()
        .map(|(&pos, chars)| {
            (
                pos,
                chars
                    .iter()
                    .map(|(&ch, word_indices)| (ch, word_indices.len()))
                    .collect(),
            )
        })
        .collect()
}

pub fn find_pos_char_with_max_weight_in_pos(
    dict: &Dict,
    positions: &HashSet<CharPos>,
) -> Option<(CharPos, char)> {
    dict.char_at_pos_index()
        .iter()
        .filter(|(pos, _)| positions.contains(pos))
        .filter_map(|(&pos, chars)| {
            chars
                .iter()
                .filter(|(_, word_indices)| !word_indices.is_empty())
                .map(|(&ch, word_indices)| {
                    let word_count = word_indices.len();
                    (ch, word_count, word_count)
                })
                .reduce(
                    |(acc_ch, acc_max_word_count, acc_total_word_count),
                     (ch, max_word_count, total_word_count)| {
                        let (result_max_word_count, result_ch) =
                            max((acc_max_word_count, acc_ch), (max_word_count, ch));
                        (
                            result_ch,
                            result_max_word_count,
                            acc_total_word_count + total_word_count,
                        )
                    },
                )
                .map(|(ch, max_word_count, total_word_count)| {
                    (
                        pos,
                        ch,
                        max_word_count,
                        Ratio::new(max_word_count, total_word_count),
                    )
                })
        })
        .max_by_key(|&(pos, ch, max_word_count, probability)| {
            (probability, max_word_count, ch, pos)
        })
        .map(|(pos, ch, _, _)| (pos, ch))
}

struct SuggestWordState {
    positions: HashSet<CharPos>,
    letters: Vec<char>,
    dict: Dict,
}

pub fn suggest_word(mut dict: Dict) -> Option<Vec<char>> {
    let mut positions: HashSet<_> = dict.char_at_pos_index().keys().copied().collect();
    if positions.is_empty() {
        return None;
    }
    let mut letters = vec![char::default(); positions.len()];
    loop {
        let (pos, ch) = find_pos_char_with_max_weight_in_pos(&dict, &positions)?;
        positions.remove(&pos);
        let chars = HashSet::from([ch]);
        dict.only_chars_at_poses(&[pos].into(), &chars);
        dict.deny_chars_at_poses(&positions, &chars);
        let CharPos(pos) = pos;
        *letters.get_mut(pos).unwrap_or_else(|| unreachable!()) = ch;
        if positions.is_empty() {
            break Some(letters);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::{
        io,
        io::{BufWriter, Write},
    };

    #[test]
    fn pos_stats() {
        let dict = Dict::default();
        let mut sorted_stats = super::pos_stats(&dict)
            .iter()
            .map(|(&pos, lengths)| {
                let mut lengths_sum = 0;
                let mut lengths = lengths
                    .iter()
                    .map(|(&ch, &len)| {
                        lengths_sum += len;
                        (ch, len)
                    })
                    .collect_vec();
                lengths.sort_unstable_by(|(a_char, a_len), (b_char, b_len)| {
                    a_len.cmp(b_len).reverse().then_with(|| a_char.cmp(b_char))
                });
                (pos, lengths, lengths_sum)
            })
            .collect_vec();
        sorted_stats.sort_unstable();
        let mut stdout = BufWriter::new(io::stdout().lock());
        for (pos, lengths, lengths_sum) in sorted_stats {
            let inv_lengths_sum = 100f64 / lengths_sum as f64;
            writeln!(
                stdout,
                "{pos:?}: {:?}",
                lengths
                    .iter()
                    .map(|&(ch, len)| (ch, len, (len as f64 * inv_lengths_sum) as u8))
                    .collect_vec()
            )
            .unwrap();
        }
    }

    #[test]
    fn find_pos_char_with_max_weight_in_pos() {
        let dict = Dict::default();
        assert_eq!(
            super::find_pos_char_with_max_weight_in_pos(
                &dict,
                &dict.char_at_pos_index().keys().copied().collect()
            ),
            Some((CharPos(4), 'а'))
        );
    }

    #[test]
    fn suggest_word() {
        let dict = Dict::default();
        let suggest_word = super::suggest_word(dict);
        assert_eq!(suggest_word.as_ref().and_then(|w| w.get(4)), Some(&'а'));
        assert_eq!(suggest_word, Some("щетка".chars().collect()));
    }

    #[test]
    fn find_pos_char_with_max_weight_in_pos_empty() {
        let dict = Dict::empty();
        assert_eq!(
            super::find_pos_char_with_max_weight_in_pos(
                &dict,
                &dict.char_at_pos_index().keys().copied().collect()
            ),
            None
        );
    }
}
