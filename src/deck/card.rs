pub mod rank;
pub mod suit;

use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use rank::Rank;
use suit::Suit;

/// Card struct to represent a playing card with a suit and a rank.
/// Card suits can be compared for equality only.
/// Card ranks can be compared for equality and ordering.
#[derive(Eq, Hash, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,

    _phantom: PhantomData<()>,
}

impl Card {
    pub fn new(mut suit: Suit, mut rank: Rank) -> Self {
        if suit == Suit::Joker {
            rank = Rank::Joker;
        }
        if rank == Rank::Joker {
            suit = Suit::Joker;
        }

        Self {
            suit,
            rank,
            _phantom: PhantomData,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        (self.rank == other.rank) && (self.suit == other.suit)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.suit == Suit::Joker {
            write!(f, "{}", self.suit.to_string())
        } else {
            write!(f, "{} {}", self.suit.to_string(), self.rank.to_string())
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/////////////////////////////////////////////////
// Test cases
/////////////////////////////////////////////////
#[test]
fn card_struct_should_have_rank_and_suit_fields() {
    let card = Card::new(Suit::Spade, Rank::Ace);

    assert_eq!(Suit::Spade, card.suit);
    assert_eq!(Rank::Ace, card.rank);
}

#[test]
fn card_instance_should_be_comparable_by_rank() {
    let card1 = Card::new(Suit::Spade, Rank::Two);
    let card2 = Card::new(Suit::Heart, Rank::Three);
    let card3 = Card::new(Suit::Heart, Rank::Two);
    let card4 = Card::new(Suit::Diamond, Rank::Ace);
    let card5 = Card::new(Suit::Spade, Rank::Two);

    assert_eq!(Ordering::Less, card1.cmp(&card2));
    assert_eq!(Ordering::Greater, card4.cmp(&card5));
    // Note: Ordering does not consider suit.
    assert_eq!(Ordering::Equal, card1.cmp(&card3));

    assert_eq!(card1, card5);
    assert_ne!(card1, card3);
    assert_ne!(card2, card3);
}

#[test]
fn card_instance_should_make_string_of_suit_rank_order() {
    let card = Card::new(Suit::Spade, Rank::Ace);
    let joker = Card::new(Suit::Joker, Rank::Joker);

    assert_eq!("♠ A", card.to_string());
    assert_eq!("Joker", joker.to_string());
}

#[test]
fn card_instance_should_joker_card_have_joker_suit_joker_rank() {
    let joker = Card::new(Suit::Joker, Rank::Joker);
    let joker_1 = Card::new(Suit::Joker, Rank::Ace);
    let joker_2 = Card::new(Suit::Diamond, Rank::Joker);

    assert_eq!(Suit::Joker, joker_1.suit);
    assert_eq!(Rank::Joker, joker_1.rank);
    assert_eq!(joker, joker_1);
    assert_eq!(Suit::Joker, joker_2.suit);
    assert_eq!(Rank::Joker, joker_2.rank);
    assert_eq!(joker, joker_2);
}

#[test]
fn card_instance_should_have_simplified_debug_string() {
    let card = Card::new(Suit::Spade, Rank::Five);
    let joker = Card::new(Suit::Joker, Rank::Joker);

    assert_eq!("♠ 5", format!("{:?}", card));
    assert_eq!("Joker", format!("{:?}", joker));
}
