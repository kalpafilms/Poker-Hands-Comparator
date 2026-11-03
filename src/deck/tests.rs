use super::*;

#[test]
fn deck_when_instantiated_should_contains_53cards() {
    let deck = Deck::new();

    assert_eq!(53, deck.cards.len());
}

#[test]
fn deck_when_instantiated_should_all_53_cards_unique() {
    let deck = Deck::new();

    assert!(deck.cards.contains(&Card::new(Suit::Joker, Rank::Joker)));

    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Two)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Three)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Four)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Five)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Six)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Seven)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Eight)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Nine)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Ten)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Jack)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Queen)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::King)));
    assert!(deck.cards.contains(&Card::new(Suit::Spade, Rank::Ace)));

    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Two)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Three)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Four)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Five)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Six)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Seven)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Eight)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Nine)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Ten)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Jack)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Queen)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::King)));
    assert!(deck.cards.contains(&Card::new(Suit::Club, Rank::Ace)));

    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Two)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Three)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Four)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Five)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Six)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Seven)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Eight)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Nine)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Ten)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Jack)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Queen)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::King)));
    assert!(deck.cards.contains(&Card::new(Suit::Heart, Rank::Ace)));

    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Two)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Three)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Four)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Five)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Six)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Seven)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Eight)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Nine)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Ten)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Jack)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Queen)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::King)));
    assert!(deck.cards.contains(&Card::new(Suit::Diamond, Rank::Ace)));
}

#[test]
fn deck_when_instantiated_two_decks_should_both_must_have_distinct_order() {
    let deck_1 = Deck::new()
        .cards
        .iter()
        .clone()
        .map(|card| card.to_string())
        .collect::<Vec<String>>();
    let deck_2 = Deck::new()
        .cards
        .iter()
        .clone()
        .map(|card| card.to_string())
        .collect::<Vec<String>>();

    assert_ne!(deck_1, deck_2);
}

#[test]
fn draw_when_draw_a_card_should_not_be_none() {
    let mut deck = Deck::new();
    let card = deck.draw();

    assert!(card.is_some());
    assert!(!deck.cards.contains(&card.unwrap()));

    assert_eq!(52, deck.cards.len());
}

#[test]
fn draw_when_draw_54_cards_should_be_none_the_last_one() {
    let mut deck = Deck::new();

    for _ in 0..53 {
        let card = deck.draw();
        assert!(card.is_some());
        assert!(!deck.cards.contains(&card.unwrap()));
    }

    assert!(deck.draw().is_none());
    assert!(deck.cards.is_empty());
}

#[test]
fn draw_when_draw_all_from_two_decks_should_both_have_distint_order() {
    let mut deck_1 = Deck::new();
    let mut deck_2 = Deck::new();

    let mut set_1: Vec<String> = Vec::new();
    let mut set_2: Vec<String> = Vec::new();

    for _ in 0..53 {
        let card_1 = deck_1.draw();
        let card_2 = deck_2.draw();

        assert!(card_1.is_some());
        assert!(card_2.is_some());
        set_1.push(card_1.unwrap().to_string());
        set_2.push(card_2.unwrap().to_string());
    }

    assert!(deck_1.cards.is_empty());
    assert!(deck_2.cards.is_empty());

    assert_ne!(set_1, set_2);
}
