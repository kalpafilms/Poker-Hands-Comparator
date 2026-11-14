# Design analysis

## Application purpose

Implement a 5-card poker hands comparison application.

## Playing cards basics

* 52 standard cards + 1 joker card
* 4 types of suit: Spade, Heart, Diamond, Club
* 13 numbers per suit: A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2

## Poker hands ranking

https://en.wikipedia.org/wiki/List_of_poker_hands

| Rank | Name            | Description                                                                                      |
|------|-----------------|--------------------------------------------------------------------------------------------------|
| 1    | Five of a kind  | 5 cards of the same number                                                                       |
| 2    | Straight flush  | 5 cards of sequential number, all of the same suit                                               |
| 3    | Four of a kind  | 4 cards of the same number, 1 card of another number                                             |
| 4    | Full house      | 3 cards of one number and 2 cards of another number                                              |
| 5    | Flush           | 5 cards all of the same suit, not all of sequential number                                       |
| 6    | Straight        | 5 cards of sequential number, not all of the same suit                                           |
| 7    | Three of a kind | 3 cards of the same number and 2 cards of two other numbers                                      |
| 8    | Two pair        | 2 cards of the same number, 2 cards of another number and 1 card of a third number               |
| 9    | One pair        | 2 cards of the same number and 3 cards of three other numbers                                    |
| 10   | High card       | 5 cards not all of sequential number or of the same suit, and none of which are of the same rank |

## Process flow

1. Deal 5 random cards per player.
2. Evaluate hands.
3. Compare hands.
4. Show winner and winning hand.

### Deal cards

Deal cards to players until all players have 5 cards.

### Evaluate hand

Find a hand from the highest probability to the lowest one (i.e High card to Five of a kind) for efficiency

* One pair, two pair, and three of a kind take over 49% of total probability
* Straight and higher hands take only 0.76%
* High card takes 50.11%

1. Sort dealt cards by number
2. Group cards by number
    * Sorted cards can be grouped into 3 groups by number: Pair1, Pair2, and Residue group
3. Find hand
    * Condition: Check the number of cards in each group (residue group first)
    * Note: GroupName (Number) means the number of cards in the group
    1. Residue (0)
        * Full house: Pair1 (2) + Pair2 (3) or Pair1 (3) + Pair2 (2)
    2. Residue (1)
        * Two pair: Pair1 (2) + Pair2 (2)
        * Four of a kind: Pair1 (4) + Pair2 (0)
    3. Residue (2)
        * Three of a kind: Pair1 (3) + Pair2 (0)
    4. Residue (3)
        * One pair: Pair1 (2) + Pair2 (0)
    5. Residue (4)
        * None
    6. Residue (5)
        * High card: Neither sequential nor the same suit
        * Straight: All sequential and all different suits
        * Flush: All the same suit and non-sequential
        * Straight Flush: Sequential and the same suit
4. Handle Joker
    1. Find the joker card in the residue group which is not empty
    2. Check 4 cards of sequential number for a Straight (e.g. 9, 10, Q, K, Joker)
    3. Check 4 cards of the same suit for a Flush (e.g. ♥, ♥, ♥, ♥, Joker)
    4. Update the hand by the effect of the joker
        * Residue (1)
            * Two pair → Full house
            * Four of a kind → Five of a kind
        * Residue (2)
            * Three of a kind → Four of a kind
        * Residue (3)
            * One pair → Three of a kind
        * Residue (5)
            * High card → One pair

### Compare hands and display the result

1. Compare by the determined ranks
2. Compare by numbers when both hands' ranks are the same
3. Display the winner and their hand
    * If both players have the same hand and rank, the result becomes a draw (Suit types do not have orders)
