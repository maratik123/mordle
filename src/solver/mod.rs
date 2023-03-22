use crate::{CharPos, Dict};
use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::{FromPrimitive, One};
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
) -> Option<(CharPos, char, Ratio<usize>)> {
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
        .map(|(pos, ch, _, probability)| (pos, ch, probability))
}

pub fn suggest_word(mut dict: Dict) -> Option<(Vec<char>, Ratio<BigUint>)> {
    let mut positions: HashSet<_> = dict.char_at_pos_index().keys().copied().collect();
    if positions.is_empty() {
        return None;
    }
    let mut letters = vec![char::default(); positions.len()];
    let mut total_probability = Ratio::one();
    loop {
        let (pos, ch, probability) = find_pos_char_with_max_weight_in_pos(&dict, &positions)?;
        if !positions.remove(&pos) {
            unreachable!()
        }
        dict.only_chars_at_pos(pos, &[ch].into());
        let CharPos(pos) = pos;
        *letters.get_mut(pos).unwrap_or_else(|| unreachable!()) = ch;
        total_probability *= from_ratio_usize(probability);
        if positions.is_empty() {
            break Some((letters, total_probability));
        }
    }
}

fn from_ratio_usize(ratio: Ratio<usize>) -> Ratio<BigUint> {
    Ratio::new_raw(
        BigUint::from_usize(*ratio.numer()).unwrap_or_else(|| unreachable!()),
        BigUint::from_usize(*ratio.denom()).unwrap_or_else(|| unreachable!()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io,
        io::{BufWriter, Write},
    };

    #[test]
    fn pos_stats() {
        let dict = Dict::default();
        let mut sorted_stats: Vec<(_, _, _)> = super::pos_stats(&dict)
            .iter()
            .map(|(&pos, lengths)| {
                let mut lengths_sum = 0;
                let mut lengths: Vec<_> = lengths
                    .iter()
                    .map(|(&ch, &len)| {
                        lengths_sum += len;
                        (ch, len)
                    })
                    .collect();
                lengths.sort_unstable_by(|(a_char, a_len), (b_char, b_len)| {
                    a_len.cmp(b_len).reverse().then_with(|| a_char.cmp(b_char))
                });
                (pos, lengths, lengths_sum)
            })
            .collect();
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
                    .collect::<Vec<_>>()
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
            Some((CharPos(4), 'а', Ratio::new(861, 3812)))
        );
    }

    #[test]
    fn suggest_word() {
        let dict = Dict::default();
        let suggest_word = super::suggest_word(dict);
        assert_eq!(
            suggest_word.as_ref().and_then(|(w, _)| w.get(4)),
            Some(&'а')
        );
        assert_eq!(
            suggest_word,
            Some((
                "шайка".chars().collect(),
                Ratio::new(BigUint::one(), BigUint::from(3812usize))
            ))
        );
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
