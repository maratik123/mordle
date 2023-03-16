use crate::{CharPos, Dict};
use std::collections::HashMap;

pub struct Solver<'a> {
    dict: &'a Dict,
}

impl<'a> From<&'a Dict> for Solver<'a> {
    #[inline]
    fn from(value: &'a Dict) -> Self {
        Self::new(value)
    }
}

impl<'a> Solver<'a> {
    pub fn new(dict: &'a Dict) -> Self {
        Self { dict }
    }

    pub fn pos_stats(&self) -> HashMap<CharPos, HashMap<char, usize>> {
        self.dict
            .char_at_pos_index()
            .iter()
            .map(|(&pos, chars)| {
                (
                    pos,
                    chars
                        .iter()
                        .map(|(&char, word_indices)| (char, word_indices.len()))
                        .collect(),
                )
            })
            .collect()
    }
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
        let solver = Solver::from(&dict);
        let mut sorted_stats: Vec<(_, _, _)> = solver
            .pos_stats()
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
        for (char, lengths, lengths_sum) in sorted_stats {
            let inv_lengths_sum = 100f64 / lengths_sum as f64;
            writeln!(
                stdout,
                "{char:?}: {:?}",
                lengths
                    .iter()
                    .map(|&(ch, len)| (ch, len, (len as f64 * inv_lengths_sum) as u8))
                    .collect::<Vec<_>>()
            )
            .unwrap();
        }
    }
}
