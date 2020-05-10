extern crate cm;

use crate::cm::*;
use card::Card;
use rank::Rank;
use suit::Suit;

fn main() {
    let c = Card {
        rank: Rank::Ace,
        suit: Suit::Heart,
    };
    println!("{}", c);
}
