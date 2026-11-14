use super::*;

use crate::deck::card::rank::Rank;
use crate::deck::card::suit::Suit;

/////////////////////////////////////////////////
// Instantiation
/////////////////////////////////////////////////
#[test]
fn player_when_instantiated_should_hand_is_empty() {
    let player = Player::new();

    assert!(player.on_hand.is_empty());
}

/////////////////////////////////////////////////
// Player::receive(card: Card)
/////////////////////////////////////////////////
#[test]
fn player_when_received_a_card_should_player_have_it_on_hand() {
    let mut player = Player::new();
    let card = Card::new(Suit::Heart, Rank::Queen);

    assert_eq!(0, player.on_hand.len());
    player.receive(card);
    assert_eq!(1, player.on_hand.len());
}

#[test]
fn player_should_receive_up_to_5_cards() {
    let mut player = Player::new();
    let card_1 = Card::new(Suit::Heart, Rank::Queen);
    let card_2 = Card::new(Suit::Spade, Rank::Ace);
    let card_3 = Card::new(Suit::Club, Rank::Ten);
    let card_4 = Card::new(Suit::Diamond, Rank::Two);
    let card_5 = Card::new(Suit::Heart, Rank::Five);
    let card_6 = Card::new(Suit::Joker, Rank::Joker);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);
    player.receive(card_6);

    assert_eq!(5, player.on_hand.len());
    for card in vec![card_1, card_2, card_3, card_4, card_5] {
        assert!(player.on_hand.contains(&card));
    }
    assert!(!player.on_hand.contains(&card_6));
}

#[test]
fn player_when_received_5_cards_should_cards_are_grouped() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Ace);
    let card_3 = Card::new(Suit::Heart, Rank::Ace);
    let card_4 = Card::new(Suit::Club, Rank::Ace);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);


    let group_0_size = player.card_group[0].len();
    let group_1_size = player.card_group[1].len();
    let group_s_size = player.card_group[2].len();

    assert_eq!(5, group_0_size + group_1_size + group_s_size);
}

#[test]
fn player_when_received_5_cards_should_hand_is_evaluated() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Ace);
    let card_3 = Card::new(Suit::Heart, Rank::Ace);
    let card_4 = Card::new(Suit::Club, Rank::Ace);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    assert!(player.hand.is_some());
}

/////////////////////////////////////////////////
// Player::group()
/////////////////////////////////////////////////
#[test]
fn player_when_instantiated_should_have_3_card_groups() {
    let player = Player::new();

    assert_eq!(3, player.card_group.len());
}

#[test]
fn player_when_instantiated_should_all_card_group_empty() {
    let player = Player::new();

    assert!(player.card_group[0].is_empty());
    assert!(player.card_group[1].is_empty());
    assert!(player.card_group[2].is_empty());
}

#[test]
fn group_when_received_5_cards_should_sort_hand_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Ten);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Heart, Rank::Five);

    let sorted = vec![card_4, card_5, card_2, card_3, card_1];

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(sorted, player.on_hand);
}

#[test]
fn group_when_received_5_cards_should_group_into_3() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Ace);
    let card_3 = Card::new(Suit::Heart, Rank::Ace);
    let card_4 = Card::new(Suit::Club, Rank::Ace);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    let group_0_size = player.card_group[0].len();
    let group_1_size = player.card_group[1].len();
    let group_s_size = player.card_group[2].len();

    assert_eq!(5, group_0_size + group_1_size + group_s_size);
}

#[test]
fn group_when_received_high_card_should_group_0_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[0].is_empty());
}

#[test]
fn group_when_received_high_card_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_high_card_should_group_singleton_has_5_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(5, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_2, card_3, card_4, card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_one_pair_should_group_0_has_2_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(2, player.card_group[0].len());
    assert_eq!(HashSet::from([card_3, card_4]), player.card_group[0]);
}

#[test]
fn group_when_received_one_pair_should_group_0_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[0].iter() {
        assert_eq!(Rank::Five, card.rank);
    }
}

#[test]
fn group_when_received_one_pair_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_one_pair_should_group_singleton_has_3_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(3, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_2, card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_two_pair_should_group_0_has_2_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(2, player.card_group[0].len());
    assert_eq!(HashSet::from([card_1, card_2]), player.card_group[0]);
}

#[test]
fn group_when_received_two_pair_should_group_0_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[0].iter() {
        assert_eq!(Rank::Two, card.rank);
    }
}

#[test]
fn group_when_received_two_pair_should_group_1_has_2_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(2, player.card_group[1].len());
    assert_eq!(HashSet::from([card_4, card_5]), player.card_group[1]);
}

#[test]
fn group_when_received_two_pair_should_group_1_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[1].iter() {
        assert_eq!(Rank::Nine, card.rank);
    }
}

#[test]
fn group_when_received_two_pair_should_group_singleton_has_1_card() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Ace);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(1, player.card_group[2].len());
    assert_eq!(HashSet::from([card_3]), player.card_group[2]);
}

#[test]
fn group_when_received_three_of_a_kind_should_group_0_has_3_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(3, player.card_group[0].len());
    assert_eq!(HashSet::from([card_2, card_3, card_4]), player.card_group[0]);
}

#[test]
fn group_when_received_three_of_a_kind_should_group_0_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[0].iter() {
        assert_eq!(Rank::Five, card.rank);
    }
}

#[test]
fn group_when_received_three_of_a_kind_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_three_of_a_kind_should_group_singleton_has_2_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(2, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_straight_should_group_0_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[0].is_empty());
}

#[test]
fn group_when_received_straight_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_straight_should_group_singleton_has_5_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(5, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_2, card_3, card_4, card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_flush_should_group_0_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[0].is_empty())
}

#[test]
fn group_when_received_flush_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty())
}

#[test]
fn group_when_received_flush_should_group_singleton_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(5, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_2, card_3, card_4, card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_full_house_should_group_0_has_2_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(2, player.card_group[0].len());
    assert_eq!(HashSet::from([card_1, card_2]), player.card_group[0]);
}

#[test]
fn group_when_received_full_house_should_group_0_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[0].iter() {
        assert_eq!(Rank::Two, card.rank);
    }
}

#[test]
fn group_when_received_full_house_should_group_1_has_3_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(3, player.card_group[1].len());
    assert_eq!(HashSet::from([card_3, card_4, card_5]), player.card_group[1]);
}

#[test]
fn group_when_received_full_house_should_group_1_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[1].iter() {
        assert_eq!(Rank::Seven, card.rank);
    }
}

#[test]
fn group_when_received_full_house_should_group_singleton_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[2].is_empty());
}

#[test]
fn group_when_received_four_of_a_kind_should_group_0_has_4_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(4, player.card_group[0].len());
    assert_eq!(HashSet::from([card_1, card_2, card_3, card_4]), player.card_group[0]);
}

#[test]
fn group_when_received_four_of_a_kind_should_group_0_has_all_same_rank() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    for card in player.card_group[0].iter() {
        assert_eq!(Rank::Two, card.rank);
    }
}

#[test]
fn group_when_received_four_of_a_kind_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_four_of_a_kind_should_group_singleton_has_1_card() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(1, player.card_group[2].len());
    assert_eq!(HashSet::from([card_5]), player.card_group[2]);
}

#[test]
fn group_when_received_straight_flush_should_group_0_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[0].is_empty());
}

#[test]
fn group_when_received_straight_flush_should_group_1_is_empty() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert!(player.card_group[1].is_empty());
}

#[test]
fn group_when_received_straight_flush_should_group_singleton_has_5_cards() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5]);
    player.group();

    assert_eq!(5, player.card_group[2].len());
    assert_eq!(HashSet::from([card_1, card_2, card_3, card_4, card_5]), player.card_group[2]);
}

/////////////////////////////////////////////////
// Player::evaluate_hand()
/////////////////////////////////////////////////
#[test]
fn player_when_instantiated_should_hand_is_none() {
    let player = Player::new();

    assert!(player.hand.is_none());
}

#[test]
fn evaluate_hand_when_received_high_card_should_hand_is_high_card() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::HighCard, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_high_card_with_joker_should_hand_is_one_pair() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Joker, Rank::Joker);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::OnePair, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_one_pair_should_hand_is_one_pair() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::OnePair, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_one_pair_with_joker_should_hand_is_three_of_a_kind() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Joker, Rank::Joker);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::ThreeOfAKind, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_two_pair_should_hand_is_two_pair() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::TwoPair, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_two_pair_with_joker_should_hand_is_full_house() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Joker, Rank::Joker);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::FullHouse, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_three_of_a_kind_should_hand_is_three_of_a_kind() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Spade, Rank::Three);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::ThreeOfAKind, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_three_of_a_kind_with_joker_should_hand_is_four_of_a_kind() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Joker, Rank::Joker);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Three);
    let card_5 = Card::new(Suit::Spade, Rank::Three);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::FourOfAKind, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_straight_should_hand_is_straight() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::Straight, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_flush_should_hand_is_flush() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::Flush, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_full_house_should_hand_is_full_house() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Seven);
    let card_4 = Card::new(Suit::Club, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::FullHouse, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_four_of_a_kind_should_hand_is_four_of_a_kind() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Spade, Rank::Seven);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::FourOfAKind, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_straight_flush_should_hand_is_straight_flush() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::StraightFlush, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_received_five_of_a_kind_should_hand_is_five_of_a_kind() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Diamond, Rank::Two);
    let card_3 = Card::new(Suit::Heart, Rank::Two);
    let card_4 = Card::new(Suit::Club, Rank::Two);
    let card_5 = Card::new(Suit::Joker, Rank::Joker);

    player.receive(card_1);
    player.receive(card_2);
    player.receive(card_3);
    player.receive(card_4);
    player.receive(card_5);

    player.evaluate_hand();

    assert_eq!(Hand::FiveOfAKind, player.hand.unwrap());
}

#[test]
fn evaluate_hand_when_have_more_than_5_singleton_should_hand_is_none() {
    let mut player = Player::new();

    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Ace);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);
    let card_6 = Card::new(Suit::Heart, Rank::Seven);

    player.on_hand.extend([card_1, card_2, card_3, card_4, card_5, card_6]);
    player.group();
    player.evaluate_hand();

    assert_eq!(None, player.hand);
}

/////////////////////////////////////////////////
// Player::is_straight(cards: &HashSet<Card>)
/////////////////////////////////////////////////
#[test]
fn is_straight_when_received_one_card_should_return_false() {
    let cards = HashSet::from([Card::new(Suit::Spade, Rank::Six)]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_high_card_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);
    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_one_pair_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Ace);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Heart, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Five);
    let card_5 = Card::new(Suit::Spade, Rank::Nine);
    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_straight_should_return_true() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_flush_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_straight_flush_should_return_true() {
    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_joker_high_card_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Joker, Rank::Joker);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);
    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_joker_straight_should_return_true() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Joker, Rank::Joker);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_joker_flush_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Joker, Rank::Joker);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_straight(&cards));
}

#[test]
fn is_straight_when_received_joker_straight_flush_should_return_true() {
    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Joker, Rank::Joker);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_straight(&cards));
}

/////////////////////////////////////////////////
// Player::is_flush(cards: &HashSet<Card>)
/////////////////////////////////////////////////
#[test]
fn is_flush_when_received_high_card_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Three);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Club, Rank::Ten);
    let card_5 = Card::new(Suit::Spade, Rank::Two);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_flush(&cards));
}

#[test]
fn is_flush_when_received_straight_should_return_false() {
    let card_1 = Card::new(Suit::Spade, Rank::Six);
    let card_2 = Card::new(Suit::Diamond, Rank::Seven);
    let card_3 = Card::new(Suit::Heart, Rank::Eight);
    let card_4 = Card::new(Suit::Club, Rank::Nine);
    let card_5 = Card::new(Suit::Spade, Rank::Ten);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(false, Player::is_flush(&cards));
}

#[test]
fn is_flush_when_received_flush_should_return_true() {
    let card_1 = Card::new(Suit::Spade, Rank::Two);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_flush(&cards));
}

#[test]
fn is_flush_when_received_straight_flush_should_return_true() {
    let card_1 = Card::new(Suit::Diamond, Rank::Four);
    let card_2 = Card::new(Suit::Diamond, Rank::Five);
    let card_3 = Card::new(Suit::Diamond, Rank::Six);
    let card_4 = Card::new(Suit::Diamond, Rank::Seven);
    let card_5 = Card::new(Suit::Diamond, Rank::Eight);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_flush(&cards));
}

#[test]
fn is_flush_when_received_joker_flush_should_return_true() {
    let card_1 = Card::new(Suit::Joker, Rank::Joker);
    let card_2 = Card::new(Suit::Spade, Rank::Four);
    let card_3 = Card::new(Suit::Spade, Rank::Five);
    let card_4 = Card::new(Suit::Spade, Rank::Seven);
    let card_5 = Card::new(Suit::Spade, Rank::Jack);

    let cards = HashSet::from([card_1, card_2, card_3, card_4, card_5]);

    assert_eq!(true, Player::is_flush(&cards));
}
