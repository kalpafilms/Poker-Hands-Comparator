mod rank;
mod suit;

use std::cmp::Ordering;
use std::fmt::Display;

use rank::Rank;
use suit::Suit;

/// Card struct to represent a playing card with a suit and a rank.
/// Card suits can be compared for equality only.
/// Card ranks can be compared for equality and ordering.
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.suit.to_string(), self.rank.to_string())
    }
}

/////////////////////////////////////////////////
// Test cases
/////////////////////////////////////////////////
#[test]
fn card_struct_should_have_rank_and_suit_fields() {
    let card = Card::new(Suit::Spade, Rank::Ace);

    assert_eq!(card.suit, Suit::Spade);
    assert_eq!(card.rank, Rank::Ace);
}

#[test]
fn card_instance_should_be_comparable_by_rank() {
    let card1 = Card::new(Suit::Spade, Rank::Two);
    let card2 = Card::new(Suit::Heart, Rank::Three);
    let card3 = Card::new(Suit::Heart, Rank::Two);
    let card4 = Card::new(Suit::Diamond, Rank::Ace);

    assert!(card1 < card2);
    assert!(!(card1 > card4));

    assert_eq!(card1, card3);
    assert_ne!(card1, card2);
}

#[test]
fn card_instance_should_make_string_of_suit_rank_order() {
    let card = Card::new(Suit::Spade, Rank::Ace);
    assert_eq!("♠ A", card.to_string());
}
