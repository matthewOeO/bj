use std::fmt;

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Rank {
    Ace,
    Pip(u8),
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Ace => write!(f, "A"),
            Rank::Pip(n @ 2..=10) => write!(f, "{}", n),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn all() -> impl Iterator<Item = Rank> + Clone {
        let mut ret = Vec::with_capacity(13);
        for rank in Rank::iter() {
            match rank {
                Rank::Pip(_) => (2..=10).for_each(|i| ret.push(Rank::Pip(i))),
                _ => ret.push(rank),
            }
        }
        ret.into_iter()
    }
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Rank::Ace), "A");
    assert_eq!(format!("{}", Rank::Pip(3)), "3");
}
