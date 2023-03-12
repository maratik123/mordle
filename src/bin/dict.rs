use anyhow::Result;
use clap::Parser;
use std::{
    collections::BTreeSet,
    io,
    io::{BufRead, BufReader, BufWriter, Write},
};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "Filter dictionary: accepts dictionary on stdin, and outputs to stdout",
    long_about = None
)]
struct Cli {
    /// Word length
    #[arg(short, long, default_value_t = 5)]
    length: usize,
    /// Map cyrillic yo to e
    #[arg(short, long, default_value_t = true)]
    map_e_yo: bool,
    /// Convert to lower
    #[arg(short, long, default_value_t = true)]
    to_lower: bool,
    /// Only words with cyrillic chars
    #[arg(short, long, default_value_t = true)]
    cyrillic: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut stdin = BufReader::new(stdin);
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);
    filter(
        &mut stdin,
        &mut stdout,
        cli.length,
        cli.cyrillic,
        cli.map_e_yo,
        cli.to_lower,
    )?;
    Ok(())
}

fn filter(
    r: &mut impl BufRead,
    w: &mut impl Write,
    filter_len: usize,
    cyrillic: bool,
    map_e_yo: bool,
    lower: bool,
) -> Result<()> {
    let mut result = BTreeSet::new();
    'outer: for line in r.lines() {
        let chars: Vec<_> = line?.chars().take(filter_len + 1).collect();
        if chars.len() != filter_len {
            continue;
        }
        if cyrillic
            && !chars
                .iter()
                .all(|c| matches!(c, 'А'..='Я' | 'а'..='я' | 'ё' | 'Ё'))
        {
            continue;
        }
        let mut s = String::with_capacity(chars.iter().map(|c| c.len_utf8()).sum());
        for c in chars {
            let c = if map_e_yo {
                match c {
                    'ё' => 'е',
                    'Ё' => 'Е',
                    c => c,
                }
            } else {
                c
            };
            let c = if lower {
                let mut c_it = c.to_lowercase();
                let c = c_it.next().unwrap_or_else(|| unreachable!());
                match c_it.next() {
                    None => c,
                    _ => continue 'outer,
                }
            } else {
                c
            };
            s.push(c);
        }
        result.insert(s);
    }
    for line in result {
        writeln!(w, "{line}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const DICT: &str = "\
            \n\
            1\n\
            сазан\n\
            1\n\
            22\n\
            22\n\
            333\n\
            4444\n\
            55555\n\
            666666\n\
            first\n\
            Сазан\n\
            взлет\n\
            взлёт\n\
            55555\
            ";

    #[test]
    fn filter_dict_default() {
        filter_and_test(DICT, "взлет\nсазан\n", 5, true, true, true);
    }

    #[test]
    fn filter_dict_5() {
        filter_and_test(
            DICT,
            "55555\nfirst\nСазан\nвзлет\nвзлёт\nсазан\n",
            5,
            false,
            false,
            false,
        );
    }

    #[test]
    fn filter_dict_4() {
        filter_and_test(DICT, "4444\n", 4, false, false, false);
    }

    #[test]
    fn filter_dict_5_cyr() {
        filter_and_test(DICT, "Сазан\nвзлет\nвзлёт\nсазан\n", 5, true, false, false);
    }

    #[test]
    fn filter_dict_5_e_yo() {
        filter_and_test(
            DICT,
            "55555\nfirst\nСазан\nвзлет\nсазан\n",
            5,
            false,
            true,
            false,
        );
    }

    #[test]
    fn filter_dict_5_lower() {
        filter_and_test(
            DICT,
            "55555\nfirst\nвзлет\nвзлёт\nсазан\n",
            5,
            false,
            false,
            true,
        );
    }

    fn filter_and_test(
        dict: &str,
        expected: &str,
        filter_len: usize,
        cyrillic: bool,
        map_e_yo: bool,
        lower: bool,
    ) {
        let mut out = vec![];
        {
            let mut buf_out = BufWriter::new(&mut out);
            filter(
                &mut Cursor::new(dict),
                &mut buf_out,
                filter_len,
                cyrillic,
                map_e_yo,
                lower,
            )
            .unwrap();
        }
        assert_eq!(String::from_utf8(out).unwrap(), expected);
    }
}
