use std::fmt;

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    pub fn all() -> impl Iterator<Item = Suit> {
        Suit::iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Spade => write!(f, "♠"),
            Suit::Heart => write!(f, "♥"),
            Suit::Club => write!(f, "♣"),
            Suit::Diamond => write!(f, "♦"),
        }
    }
}
