use strum::{Display, EnumCount};

/// Suit enum to represent the suit on a card.
/// Suit enum must have exactly five types and must be exhaustive.
/// Suits support only equality comparison.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Display, EnumCount)]
pub enum Suit {
    Joker = 1,
    #[strum(to_string = "\u{2660}")]
    Spade = 2,
    #[strum(to_string = "\u{2663}")]
    Club = 3,
    #[strum(to_string = "\u{2665}")]
    Heart = 4,
    #[strum(to_string = "\u{2666}")]
    Diamond = 5,
}

/////////////////////////////////////////////////
// Test cases
/////////////////////////////////////////////////
#[test]
fn suit_enum_should_include_four_suits_and_a_joker() {
    // Playing card suits are fixed
    assert_eq!(5, Suit::COUNT);

    assert_eq!(1, Suit::Joker as u8);
    assert_eq!(2, Suit::Spade as u8);
    assert_eq!(3, Suit::Club as u8);
    assert_eq!(4, Suit::Heart as u8);
    assert_eq!(5, Suit::Diamond as u8);
}

#[test]
fn suit_enum_should_display_its_string() {
    assert_eq!("Joker", Suit::Joker.to_string());
    assert_eq!("\u{2660}", Suit::Spade.to_string());
    assert_eq!("\u{2666}", Suit::Diamond.to_string());
    assert_eq!("\u{2665}", Suit::Heart.to_string());
    assert_eq!("\u{2663}", Suit::Club.to_string());
}

#[test]
fn suit_enum_should_be_able_to_check_equality() {
    assert_eq!(Suit::Spade, Suit::Spade);
    assert_ne!(Suit::Spade, Suit::Club);
}
