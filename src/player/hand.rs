use strum::{Display, EnumCount};

/// Hand enum to represent the strength of a hand.
/// Hand enum must have exactly 10 types and must be exhaustive.
/// Hands support equality and ordering comparisons.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Display, EnumCount)]
pub enum Hand {
    #[strum(to_string = "High Card")]
    HighCard = 1,
    #[strum(to_string = "One Pair")]
    OnePair = 2,
    #[strum(to_string = "Two Pair")]
    TwoPair = 3,
    #[strum(to_string = "Three Of A Kind")]
    ThreeOfAKind = 4,
    #[strum(to_string = "Straight")]
    Straight = 5,
    #[strum(to_string = "Flush")]
    Flush = 6,
    #[strum(to_string = "Full House")]
    FullHouse = 7,
    #[strum(to_string = "Four Of A Kind")]
    FourOfAKind = 8,
    #[strum(to_string = "Straight Flush")]
    StraightFlush = 9,
    #[strum(to_string = "Five Of A Kind")]
    FiveOfAKind = 10,
}

/////////////////////////////////////////////////
// Test cases
/////////////////////////////////////////////////
#[test]
fn hand_should_have_high_card_to_five_of_a_kind() {
    assert_eq!(Hand::COUNT, 10);

    assert_eq!(1, Hand::HighCard as u8);
    assert_eq!(2, Hand::OnePair as u8);
    assert_eq!(3, Hand::TwoPair as u8);
    assert_eq!(4, Hand::ThreeOfAKind as u8);
    assert_eq!(5, Hand::Straight as u8);
    assert_eq!(6, Hand::Flush as u8);
    assert_eq!(7, Hand::FullHouse as u8);
    assert_eq!(8, Hand::FourOfAKind as u8);
    assert_eq!(9, Hand::StraightFlush as u8);
    assert_eq!(10, Hand::FiveOfAKind as u8);
}

#[test]
fn hand_enum_should_display_its_string() {
    assert_eq!("High Card", Hand::HighCard.to_string());
    assert_eq!("One Pair", Hand::OnePair.to_string());
    assert_eq!("Two Pair", Hand::TwoPair.to_string());
    assert_eq!("Three Of A Kind", Hand::ThreeOfAKind.to_string());
    assert_eq!("Straight", Hand::Straight.to_string());
    assert_eq!("Flush", Hand::Flush.to_string());
    assert_eq!("Full House", Hand::FullHouse.to_string());
    assert_eq!("Four Of A Kind", Hand::FourOfAKind.to_string());
    assert_eq!("Straight Flush", Hand::StraightFlush.to_string());
    assert_eq!("Five Of A Kind", Hand::FiveOfAKind.to_string());
}

#[test]
fn hand_enum_should_be_able_to_check_equality() {
    assert_eq!(Hand::HighCard, Hand::HighCard);
    assert_ne!(Hand::Flush, Hand::StraightFlush);
}

#[test]
fn hand_enum_should_be_comparable() {
    assert!(Hand::HighCard < Hand::OnePair);
    assert!(Hand::OnePair < Hand::TwoPair);
    assert!(Hand::TwoPair < Hand::ThreeOfAKind);
    assert!(Hand::ThreeOfAKind < Hand::Straight);
    assert!(Hand::Straight < Hand::Flush);
    assert!(Hand::Flush < Hand::FullHouse);
    assert!(Hand::FullHouse < Hand::FourOfAKind);
    assert!(Hand::FourOfAKind < Hand::StraightFlush);
    assert!(Hand::StraightFlush < Hand::FiveOfAKind);
}
