use num_enum::TryFromPrimitive;
use strum::{Display, EnumCount};

/// Rank enum to represent the rank on a card.
/// Rank enum must have exactly 14 types and must be exhaustive.
/// Ranks support equality and ordering comparisons.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Copy, Clone, Display, EnumCount, TryFromPrimitive)]
pub enum Rank {
    Joker = 1,
    #[strum(to_string = "2")]
    Two = 2,
    #[strum(to_string = "3")]
    Three = 3,
    #[strum(to_string = "4")]
    Four = 4,
    #[strum(to_string = "5")]
    Five = 5,
    #[strum(to_string = "6")]
    Six = 6,
    #[strum(to_string = "7")]
    Seven = 7,
    #[strum(to_string = "8")]
    Eight = 8,
    #[strum(to_string = "9")]
    Nine = 9,
    #[strum(to_string = "10")]
    Ten = 10,
    #[strum(to_string = "J")]
    Jack = 11,
    #[strum(to_string = "Q")]
    Queen = 12,
    #[strum(to_string = "K")]
    King = 13,
    #[strum(to_string = "A")]
    Ace = 14,
}

/////////////////////////////////////////////////
// Test cases
/////////////////////////////////////////////////
#[test]
fn rank_enum_should_include_ace_to_king_and_joker() {
    // Playing card ranks are fixed
    assert_eq!(14, Rank::COUNT);

    assert_eq!(1, Rank::Joker as u8);
    assert_eq!(2, Rank::Two as u8);
    assert_eq!(3, Rank::Three as u8);
    assert_eq!(4, Rank::Four as u8);
    assert_eq!(5, Rank::Five as u8);
    assert_eq!(6, Rank::Six as u8);
    assert_eq!(7, Rank::Seven as u8);
    assert_eq!(8, Rank::Eight as u8);
    assert_eq!(9, Rank::Nine as u8);
    assert_eq!(10, Rank::Ten as u8);
    assert_eq!(11, Rank::Jack as u8);
    assert_eq!(12, Rank::Queen as u8);
    assert_eq!(13, Rank::King as u8);
    assert_eq!(14, Rank::Ace as u8);
}

#[test]
fn rank_enum_should_display_its_string() {
    assert_eq!("Joker", Rank::Joker.to_string());
    assert_eq!("2", Rank::Two.to_string());
    assert_eq!("3", Rank::Three.to_string());
    assert_eq!("4", Rank::Four.to_string());
    assert_eq!("5", Rank::Five.to_string());
    assert_eq!("6", Rank::Six.to_string());
    assert_eq!("7", Rank::Seven.to_string());
    assert_eq!("8", Rank::Eight.to_string());
    assert_eq!("9", Rank::Nine.to_string());
    assert_eq!("10", Rank::Ten.to_string());
    assert_eq!("J", Rank::Jack.to_string());
    assert_eq!("Q", Rank::Queen.to_string());
    assert_eq!("K", Rank::King.to_string());
    assert_eq!("A", Rank::Ace.to_string());
}

#[test]
fn rank_enum_should_be_able_to_check_equality() {
    assert_eq!(Rank::Ace, Rank::Ace);
    assert_ne!(Rank::Ace, Rank::Two);
}

#[test]
fn rank_enum_should_be_comparable() {
    assert!(Rank::Two < Rank::Three);
    assert!(Rank::Ace > Rank::Jack);
}
