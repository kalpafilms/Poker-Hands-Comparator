pub mod card;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::marker::PhantomData;

use itertools::iproduct;

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
        let suits = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        for (suit, rank) in iproduct!(suits, ranks) {
            cards.insert(Card::new(suit, rank));
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
