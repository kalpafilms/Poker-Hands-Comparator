# Design Specification: Actors

## Enumerations

Note: Requires `.name()` function to retrieve the representation name (The function name may vary)

### Suit

| Variant   | Value | Representation |
|-----------|-------|----------------|
| `JOKER`   | 1     | Joker          |
| `SPADE`   | 2     | ♠ `\u2660`    |
| `CLUB`    | 3     | ♣ `\u2663`    |
| `HEART`   | 4     | ♥ `\u2665`    |
| `DIAMOND` | 5     | ♦ `\u2666`    |

### Rank

| Variant | Value | Representation |
|---------|-------|----------------|
| `JOKER` | 1     | Joker          |
| `TWO`   | 2     | 2              |
| `THREE` | 3     | 3              |
| `FOUR`  | 4     | 4              |
| `FIVE`  | 5     | 5              |
| `SIX`   | 6     | 6              |
| `SEVEN` | 7     | 7              |
| `EIGHT` | 8     | 8              |
| `NINE`  | 9     | 9              |
| `TEN`   | 10    | 10             |
| `JACK`  | 11    | J              |
| `QUEEN` | 12    | Q              |
| `KING`  | 13    | K              |
| `ACE`   | 14    | A              |

### Hand

| Variant          | Value | Representation  |
|------------------|-------|-----------------|
| `HIGH_CARD`      | 1     | High card       |
| `ONE_PAIR`       | 2     | One pair        |
| `TWO_PAIR`       | 3     | Two pair        |
| `THREE_KIND`     | 4     | Three of a kind |
| `STRAIGHT`       | 5     | Straight        |
| `FLUSH`          | 6     | Flush           |
| `FULL_HOUSE`     | 7     | Full house      |
| `FOUR_KIND`      | 8     | Four of a kind  |
| `STRAIGHT_FLUSH` | 9     | Straight flush  |
| `FIVE_KIND`      | 10    | Five of a kind  |

### Result enum

| Variant | Value | Representation |
|---------|-------|----------------|
| `LOSE`  | -1    | Lose           |
| `Draw`  | 0     | Draw           |
| `Win`   | 1     | Win            |

## Structs

### Card

* Responsibility
    * Represents a unique suit-rank combination
    * Dealt and delivered by a dealer to a player

* Attributes

| Field  | Type   | Description         |
|--------|--------|---------------------|
| `suit` | `Suit` | Represents its suit |
| `rank` | `Rank` | Represents its rank |

* Function
    * None

### Dealer

* Responsibility
    * Has 1 deck of cards
    * Shuffles the deck
    * Draws cards from the deck
    * Deals the drawn cards to players
    * Receives all players' hands
    * Compare the hands
    * Declares the game result

* Attribute

| Field  | Type          | Description             |
|--------|---------------|-------------------------|
| `deck` | `Card` vector | Contains up to 53 cards |

* Function

| Name                      | Return | Description                                   |
|---------------------------|--------|-----------------------------------------------|
| `init_deck()`             | `Unit` | Instantiates 53 unique cards and shuffle them |
| `draw_card()`             | `Card` | Draws the top card from the deck              |
| `deal_card(player)`       | `Unit` | Deals cards to the players                    |
| `declare_result(players)` | `Unit` | Decleres the game result                      |

### Player

* Responsibility
    * Receives and holds up to 5 cards from the dealer
    * Sorts the received cards
    * Groups the sorted cards into 3 groups
    * Finds the longest rank sequence
    * Determines their hand
    * reports the hand to the dealer

* Attribute

| Name            | Type          | Description         |
|-----------------|---------------|---------------------|
| `cards_on_hand` | `Card` vector | Dealt cards         |
| `grouped_cards` | `HashMap`     | Grouped information |
| `hand`          | `Hand`        | Determined hand     |

* Function

| Name                            | Return    | Description                    |
|---------------------------------|-----------|--------------------------------|
| `sort(cards)`                   | `HashMap` | Sorts the received cards       |
| `group(cards)`                  | `HashMap` | Groups the cards into 3 groups |
| `is_straight(grouped_cards[2])` | `bool`    | Checks for a straight          |
| `is_flush(r_group[2])`          | `bool`    | Checks for a flush             |
| `determine_hand(grouped_cards)` | `Hand`    | Determines their hand          |
