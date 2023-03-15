use mordle::{Dict, Game, GameFinishStatus, StaticDict};
use rand::seq::SliceRandom;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    io,
    io::{BufReader, BufWriter, Write},
};

#[derive(Debug)]
enum MainErrors {
    EmptyDict,
    ReadLineError(io::Error),
}

impl Display for MainErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MainErrors::EmptyDict => write!(f, "Empty dictionary"),
            MainErrors::ReadLineError(err) => write!(f, "Can not read line: {err}"),
        }
    }
}

impl From<io::Error> for MainErrors {
    #[inline]
    fn from(value: io::Error) -> Self {
        Self::ReadLineError(value)
    }
}

impl Error for MainErrors {}

fn main() -> anyhow::Result<()> {
    let dict = StaticDict::default();
    let word = dict
        .words()
        .choose(&mut rand::thread_rng())
        .ok_or(MainErrors::EmptyDict)?;
    let mut game = Game::new(&dict, word, 6)?;
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut stdin = BufReader::new(io::stdin().lock());
    match game.main_loop(&mut stdin, &mut stdout)? {
        GameFinishStatus::Win => {
            writeln!(stdout, "Win!")?;
        }
        GameFinishStatus::Fail => {
            writeln!(stdout, "Fail!")?;
            writeln!(stdout, "Word is: {word}")?;
        }
    }
    Ok(())
}
