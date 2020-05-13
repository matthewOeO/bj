extern crate cm;
use crate::cm::*;
use card::Card;
use rank::Rank;

#[allow(dead_code)]
fn allowable_card_value(c: Card) -> Vec<u8> {
    match c.rank {
        Rank::Ace => vec![1, 11],
        Rank::Pip(n @ 2..=10) => vec![n],
        Rank::Jack | Rank::Queen | Rank::King => vec![10],
        _ => unreachable!(),
    }
}
#[test]
fn test_allowable_card_value() {
    use suit::Suit;
    assert_eq!(
        allowable_card_value(Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        }),
        vec![1, 11]
    );
    assert_eq!(
        allowable_card_value(Card {
            rank: Rank::Pip(9),
            suit: Suit::Heart,
        }),
        vec![9]
    );
}
