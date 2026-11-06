#[cfg(test)]
mod tests;

mod hand;

use std::collections::HashSet;
use std::marker::PhantomData;

use crate::deck::card::Card;
use crate::deck::card::rank::Rank;
use crate::deck::card::suit::Suit;
use crate::player::hand::Hand;

/// Player struct represents a player in the game.
/// A player holds up to 5 cards, and sorts and groups them into 3 groups when full.
/// A player determines the hand rank from the groups, and report the hand on request.
pub struct Player {
    on_hand: Vec<Card>,
    _phantom: PhantomData<()>,
    card_group: [HashSet<Card>; 3],
    hand: Option<Hand>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            on_hand: Vec::new(),
            _phantom: PhantomData,
            card_group: [HashSet::new(), HashSet::new(), HashSet::new()],
            hand: None,
        }
    }

    /// Receive a card from the dealer.
    ///
    /// Note: Player only have up to 5 cards.
    pub fn receive(&mut self, card: Card) {
        if self.on_hand.len() < 5 {
            self.on_hand.push(card);
        }

        if self.on_hand.len() == 5 {
            self.group();
            self.evaluate_hand();
        };
    }

    /// Group cards into pairs and singletons.
    fn group(&mut self) {
        // Sort cards before grouping
        self.on_hand.sort();

        // Find pairs
        for cards in self.on_hand.windows(2) {
            if cards[0].rank == cards[1].rank {
                let group = match self.card_group[0].iter().next() {
                    None => 0,
                    Some(card) if card.rank == cards[0].rank => 0,
                    _ => 1,
                };

                self.card_group[group].insert(cards[0].clone());
                self.card_group[group].insert(cards[1].clone());
            }
        }

        // Find singletons
        for card in self.on_hand.iter() {
            let not_group_1 = !self.card_group[0].contains(card);
            let not_group_2 = !self.card_group[1].contains(card);

            if not_group_1 && not_group_2 {
                self.card_group[2].insert(card.clone());
            }
        }
    }

    /// Evaluate hands based on the grouped cards.
    fn evaluate_hand(&mut self) {
        let has_joker = self.card_group[2].contains(&Card::new(Suit::Joker, Rank::Joker));

        self.hand = Some(
            match (
                self.card_group[2].len(),
                has_joker,
                self.card_group[1].is_empty(),
            ) {
                (5, _, _) => {
                    let is_straight = Player::is_straight(&self.card_group[2]);
                    let is_flush = Player::is_flush(&self.card_group[2]);
                    match (is_straight, is_flush, has_joker) {
                        (true, true, _) => Hand::StraightFlush,
                        (true, false, _) => Hand::Straight,
                        (false, true, _) => Hand::Flush,
                        (false, false, true) => Hand::OnePair,
                        (false, false, false) => Hand::HighCard,
                    }
                }
                (3, true, _) => Hand::ThreeOfAKind,
                (3, false, _) => Hand::OnePair,
                (2, true, _) => Hand::FourOfAKind,
                (2, false, _) => Hand::ThreeOfAKind,
                (1, true, true) => Hand::FiveOfAKind,
                (1, true, false) => Hand::FullHouse,
                (1, false, true) => Hand::FourOfAKind,
                (1, false, false) => Hand::TwoPair,
                (0, _, _) => Hand::FullHouse,
                _ => return,
            },
        )
    }

    /// Determine if a hand is a straight.
    fn is_straight(cards: &HashSet<Card>) -> bool {
        if cards.len() < 2 {
            return false;
        }

        let mut has_joker = cards.contains(&Card::new(Suit::Joker, Rank::Joker));
        let mut sorted: Vec<Card> = cards.iter().cloned().collect();
        sorted.sort();

        for pair in sorted.windows(2) {
            if pair[0].rank == Rank::Joker {
                continue;
            }

            let rank_diff = pair[1].rank - pair[0].rank;
            match rank_diff {
                1 => {}
                2 if has_joker => has_joker = false,
                _ => return false,
            }
        }

        true
    }

    /// Determine if a hand is a flush.
    fn is_flush(cards: &HashSet<Card>) -> bool {
        let mut suits: HashSet<Suit> = HashSet::new();

        for card in cards {
            if card.rank == Rank::Joker {
                continue;
            }

            suits.insert(card.suit);
        }

        suits.len() == 1
    }
}
