mod card;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::marker::PhantomData;

use itertools::iproduct;
use num_enum::TryFromPrimitive;

use card::Card;
use card::rank::Rank;
use card::suit::Suit;

/// Deck struct to represent a collection of cards.
/// Upon instantiation, a deck must contain all 53 cards and be shuffled.
/// A card can be drawn from the deck.
///
/// Note: Due to the nature of Rust's HashSet, which uses a random seed for hashing, additional card shuffling is not required.
pub struct Deck {
    cards: HashSet<Card>,
    _phantom: PhantomData<()>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: HashSet<Card> = HashSet::new();
        // Insert Joker card
        cards.insert(Card::new(Suit::Joker, Rank::Joker));
        // Insert regular cards
        let suit_range = 2_u8..=5_u8;
        let rank_range = 2_u8..=14_u8;

        for (suit, rank) in iproduct!(suit_range, rank_range) {
            if let (Ok(suit), Ok(rank)) = (
                Suit::try_from_primitive(suit),
                Rank::try_from_primitive(rank),
            ) {
                cards.insert(Card::new(suit, rank));
            }
        }

        Self {
            cards,
            _phantom: PhantomData,
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        if let Some(card) = self.cards.iter().next().cloned() {
            self.cards.remove(&card);
            Some(card)
        } else {
            None
        }
    }
}
