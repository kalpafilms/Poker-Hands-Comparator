use strum::{Display, EnumCount};

/// Suit enum to represent the suit on a card.
#[repr(u8)]
#[derive(Display, EnumCount)]
enum Suit {
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
