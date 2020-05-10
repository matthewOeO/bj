use std::fmt;

use super::card::*;

struct Hand(Vec<Card>);

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", merge(&self.0))
    }
}
#[allow(dead_code)]
fn merge(input: &Vec<Card>) -> String {
    let (car, cdr) = input.split_first().unwrap();
    cdr.iter().fold(car.to_string(), |acc, v| {
        acc.lines()
            .zip(v.to_string().lines())
            .map(|(c1, c2)| format!("{} {}\n", c1, c2))
            .collect::<String>()
    })
}
#[test]
fn test_display() {
    use super::rank::*;
    use super::suit::*;
    let h: Hand = Hand(vec![
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
    ]);
    assert_eq!(
        "┌───┐ ┌───┐ ┌───┐
│♥  │ │♠  │ │♦  │
│  A│ │ 10│ │  2│
└───┘ └───┘ └───┘
",
        h.to_string()
    );
}
