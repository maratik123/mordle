use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub enum CharResult {
    Exact,
    NotInPosition,
    Unsuccessful,
}

impl Display for CharResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CharResult::Exact => '+',
                CharResult::NotInPosition => '?',
                CharResult::Unsuccessful => ' ',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_exact() {
        assert_eq!("+", CharResult::Exact.to_string());
    }

    #[test]
    fn display_not_in_position() {
        assert_eq!("?", CharResult::NotInPosition.to_string());
    }

    #[test]
    fn display_unsuccessful() {
        assert_eq!(" ", CharResult::Unsuccessful.to_string());
    }
}
