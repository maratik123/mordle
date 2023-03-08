use anyhow::Result;
use clap::Parser;
use std::{
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
    #[arg(short, long, default_value_t = true)]
    map_e_yo: bool,
    #[arg(short, long, default_value_t = true)]
    to_lower: bool,
    #[arg(short, long, default_value_t = true)]
    cyrillic: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let stdin = stdin.lock();
        let mut buf_stdin = BufReader::new(stdin);
        let stdout = stdout.lock();
        let mut buf_stdout = BufWriter::new(stdout);
        filter(
            &mut buf_stdin,
            &mut buf_stdout,
            cli.length,
            cli.cyrillic,
            cli.map_e_yo,
            cli.to_lower,
        )?;
    }
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
            write!(w, "{c}")?;
        }
        writeln!(w)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn filter_dict() {
        let dict = "\
        \n\
        1\n\
        1\n\
        22\n\
        22\n\
        333\n\
        4444\n\
        55555\n\
        666666\n\
        first\n\
        Сазан\n\
        взлёт\n\
        55555\
        ";
        let mut out = Vec::new();
        {
            let mut buf_out = BufWriter::new(&mut out);
            filter(&mut Cursor::new(dict), &mut buf_out, 5, true, true, true).unwrap();
        }
        assert_eq!(String::from_utf8(out).unwrap(), "сазан\nвзлет\n");
    }
}
