use std::fmt;

use super::rank::*;
use super::suit::*;

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "┌───┐\n│{: <3}│\n│{: >3}│\n└───┘",
            format!("{}", self.suit),
            format!("{}", self.rank),
        )
    }
}

#[test]
fn test_display() {
    let c = Card {
        rank: Rank::Ace,
        suit: Suit::Heart,
    };
    println!("{}", c);
    assert_eq!(
        format!("{}", c),
        "┌───┐
│♥  │
│  A│
└───┘"
    );
}
