use crate::CharPos;
use std::collections::HashSet;

pub enum Constraints {
    CharsNotInDict(HashSet<char>),
    CharAtPos(char, CharPos),
}
