use strum::{Display, EnumCount};

/// Number enum to represent the number on a card.
#[derive(Display, EnumCount)]
enum Number {
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
fn number_enum_should_include_ace_to_king_and_joker() {
    assert_eq!(14, Number::COUNT);

    assert_eq!(1, Number::Joker as u8);
    assert_eq!(2, Number::Two as u8);
    assert_eq!(3, Number::Three as u8);
    assert_eq!(4, Number::Four as u8);
    assert_eq!(5, Number::Five as u8);
    assert_eq!(6, Number::Six as u8);
    assert_eq!(7, Number::Seven as u8);
    assert_eq!(8, Number::Eight as u8);
    assert_eq!(9, Number::Nine as u8);
    assert_eq!(10, Number::Ten as u8);
    assert_eq!(11, Number::Jack as u8);
    assert_eq!(12, Number::Queen as u8);
    assert_eq!(13, Number::King as u8);
    assert_eq!(14, Number::Ace as u8);
}

#[test]
fn number_enum_should_display_its_string() {
    assert_eq!("Joker", Number::Joker.to_string());
    assert_eq!("2", Number::Two.to_string());
    assert_eq!("3", Number::Three.to_string());
    assert_eq!("4", Number::Four.to_string());
    assert_eq!("5", Number::Five.to_string());
    assert_eq!("6", Number::Six.to_string());
    assert_eq!("7", Number::Seven.to_string());
    assert_eq!("8", Number::Eight.to_string());
    assert_eq!("9", Number::Nine.to_string());
    assert_eq!("10", Number::Ten.to_string());
    assert_eq!("J", Number::Jack.to_string());
    assert_eq!("Q", Number::Queen.to_string());
    assert_eq!("K", Number::King.to_string());
    assert_eq!("A", Number::Ace.to_string());
}
