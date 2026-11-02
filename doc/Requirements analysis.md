# Requirements analysis

## What should be done

Compare two 5-card poker hands

## Flow

1. Two players come to the table
2. The dealer deals cards onto the table in front of each player
3. Each player sorts and groups the received cards
4. Each player determines their hands
5. The dealer declares the winner by comparing the rank of players' hands

## Actors

* Card: represents a card that has suit and rank attributes
* Deck: represents a card deck that has 53 unique cards in it
* Dealer: deals cards to players
* Dealer (as result declarer): declares the game result
* Player: holds up to 5 dealt cards
* Player (as card grouper): sorts cards by rank and groups them into 3 by rank
* Player (as suit counter): counts rank per suit
* Player (as sequential finder): finds the longest rank sequence
* Player (as hand determiner): determines its hand

## Flow per actor

1. Dealer: places 53 cards in a deck
2. Dealer: shuffles the deck
3. Dealer: Deal cards to players
4. Player: Sort and group dealt cards
5. Player: Count suits
6. Player: Count sequential ranks
7. Player: Determine their hand
8. Dealer: Compare all hands
9. Dealer: Declare winners and losers

## Responsibilities and characteristics

### Card

#### Responsibility

* Dealt to players from a deck by a dealer
* Held by a player
* Forms part of a hand

#### Characteristics

* A card has 'suit' and 'rank' attributes
    * 'Suit' consists of 4 types; Spade, Diamond, Heart, and Club
    * 'Rank' consists of 13 sequential ranks; 2 to 10, J (11), Q (12), K (13), and A (14)
* A card is one of 52 combinations (4 suits × 13 ranks)
* The joker card has its own unique suit and rank
    * Only the Joker card has the Joker attribute
    * No other card can have the Joker attribute in its suit, rank, or both attributes
* A single card is comparable with another single card by their ranks

### Deck

#### Responsibility

* Holds up to 53 cards
* Gives a card as per the dealer's request

#### Characteristics

* Cards in the deck are shufflable
* A deck consists of 53 different cards

### Player

#### Responsibility

Player has multiple responsibilities

* Receives and holds cards
* Sorts and groups cards by rank
* Counts rank of the same suit
* Finds longest consecutive sequence of ranks
* Determines hand according to hand ranking table

#### Characteristics

* Holds up to 5 cards
* Manages cards

### Dealer

#### Responsibility

Dealer has multiple responsibilities

* Draws a card from a deck
* Deals the drawn card to a player
* Declares the result from players' hands

#### Characteristics

* Declares the result as 'Win', 'Lose', or 'Draw'

## Logics

### Card grouper

* Grouper clusters cards by rank
* The group pertains to One/Two pairs, Three/Four/Five of a kind, and Full house combinations
* Cards form three sets: first group, second group, residue
* The second group’s rank is higher due to sorting
* Example: [♠2, ♦2, ♠3, ♦3, ♠4] → [♠2, ♦2], [♠3, ♦3], [5]

### Hand determiner

* The hand evaluator determines the player’s hand based on the cards held
* Test hands from the highest probability (High card to Four of a kind) for efficiency
* Hands can be simplified by residue card count.
    * Note: _R (0)_ means 0 card in the residue group, _G1 (5)_ means 5 cards in the first group, and _G2 (3)_ means 3
      cards in the second group
    * R (0)
        * Full house: G1 (2) + G2 (3) or G1 (3) + G (2)
    * R (1)
        * Two pair: G1 (2) + G2 (2)
        * Four of a kind: G1 (4) + G2 (0)
    * R (2)
        * Three of a kind: G1 (3) + G2 (0)
    * R (3)
        * One pair: G1 (2) + G (0)
    * R (4)
        * No possible hand
    * R (5)
        * High card: Neither sequential nor the same suit
        * Straight: All sequential and all different suits
        * Flush: All the same suit and non-sequential
        * Straight Flush: Sequential and the same suit
* The joker upgrades the hand
    * Hand is upgraded to the highest possible hand (e.g., One pair becomes Three of a kind, which is higher than Two
      pair)
    * Full house, Straight, Flush, and Straight flush, which require five cards, do not need to undergo this process
    * One pair → Three of a kind
    * Two pair → Full house
    * Three of a kind → Four of a kind
    * Four of a kind → Five of a kind
    * High card needs to evaluate rank and suit for One pair, Straight, Flush, and Straight flush

### Suit counter

* Counter counts same-suit cards
* The count is related to Flush and Straight flush
* Counter tracks only the maximum count
* If two or more suit counts are equal, only the first suit is used
    * Possible combinations are [5, 0, 0, 0], [4, 1, 0, 0], [3, 2, 0, 0], [3, 1, 1, 0], [2, 2, 1, 0], [2, 1, 1, 1]
    * i.e., two or more suit counts mean they cannot form a Flush or Straight flush

### Sequential finder

* Finder finds the longest sequential rank
* The sequential rank is related to Straight and Straight flush

### Hand comparer

* Comparer compares two hands using the hand ranking table
* Higher rank wins
* If hands are identical, the higher rank wins
    * High card, Straight, Flush, Straight flush: last rank of the residue group
    * One pair, Three of a kind, Four of a kind, Five of a kind: last rank of the first group
    * Two pair: last rank of the second group
    * Full house: last rank of a group that has 3 cards
