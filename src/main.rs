use mordle::{Attempt, CharResult, Dict, Game, GameError, GameFinishStatus};
use rand::seq::SliceRandom;
use std::io::BufWriter;
use std::{
    collections::BTreeSet,
    error::Error,
    fmt::{Debug, Display, Formatter},
    io,
    io::Write,
};

#[derive(Debug)]
enum MainErrors {
    EmptyDict,
}

impl Display for MainErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MainErrors::EmptyDict => write!(f, "Empty dictionary"),
        }
    }
}

impl Error for MainErrors {}

fn main() -> anyhow::Result<()> {
    let dict = Dict::default();
    let &word = dict
        .words()
        .choose(&mut rand::thread_rng())
        .ok_or(MainErrors::EmptyDict)?;
    let mut game = Game::new(&dict, word, 6)?;
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut avail_chars: BTreeSet<_> = dict.global_char_index().keys().copied().collect();
    for t in 1usize.. {
        loop {
            input_buffer.clear();

            {
                let stdout = stdout.lock();
                let mut stdout = BufWriter::new(stdout);
                write!(stdout, "Available chars: ")?;
                for ch in &avail_chars {
                    write!(stdout, "{}", ch)?;
                }
                writeln!(stdout)?;

                write!(stdout, "Enter try {t} of {}: ", game.max_tries())?;
            }
            stdout.flush()?;

            stdin.read_line(&mut input_buffer)?;
            match game.try_input(input_buffer.trim()) {
                Ok(attempt) => {
                    let Attempt(attempt_chars) = attempt;
                    for ch in attempt_chars
                        .iter()
                        .filter(|attempt_char| attempt_char.state == CharResult::Unsuccessful)
                        .map(|attempt_char| attempt_char.ch)
                    {
                        avail_chars.remove(&ch);
                    }
                    println!("{}", attempt);
                    break;
                }
                Err(GameError::AttemptError(attempt_error)) => {
                    println!("{}", attempt_error);
                }
                other => {
                    other?;
                }
            }
        }
        match game.finish_status() {
            Some(GameFinishStatus::Win) => {
                println!("Win!");
                break;
            }
            Some(GameFinishStatus::Fail) => {
                println!("Fail!");
                println!("Word is: {word}",);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
