extern crate cm;
use crate::cm::*;
use card::Card;
use rank::Rank;

use itertools::Itertools;

#[allow(dead_code)]
fn allowable_card_value(c: Card) -> Vec<u8> {
    match c.rank {
        Rank::Ace => vec![1, 11],
        Rank::Pip(n @ 2..=10) => vec![n],
        Rank::Jack | Rank::Queen | Rank::King => vec![10],
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn hand_value(cards: Vec<Card>) -> u8 {
    let patterns = cards.into_iter().map(|c| allowable_card_value(c));
    let patterns: Vec<u8> = patterns
        .multi_cartesian_product()
        .map(|v| v.into_iter().sum())
        .filter(|&v| v < 22)
        .collect::<Vec<_>>();
    println!("{:?}", patterns);
    patterns[0]
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

#[test]
fn test_hand_value() {
    use suit::Suit;
    let cards = vec![
        Card {
            suit: Suit::Heart,
            rank: Rank::Ace,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Pip(10),
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Pip(2),
        },
    ];
    assert_eq!(hand_value(cards), 13);
}
