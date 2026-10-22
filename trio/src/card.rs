use std::fmt;

const MAX_VALUES: usize = 13;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(values: usize) -> Deck {
        let mut d = Deck::default();
        for suit in Suit::ALL {
            for value in 0..values {
                let c = Card {
                    suit,
                    value: CardValue(value),
                };
                d.cards.push(c);
            }
        }
        d
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_symbol = match self.suit {
            Suit::Diamond => "◆",
            Suit::Club => "♣",
            Suit::Spade => "♠",
        };

        let value_symbol = match self.value.0 {
            0 => String::from("A"),
            10 => String::from("J"),
            11 => String::from("Q"),
            12 => String::from("K"),
            _ => format!("{}", self.value.0 + 1),
        };

        write!(f, "{} {}", value_symbol, suit_symbol)
    }
}

// Values are 0 indexed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardValue(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Diamond,
    Club,
    Spade,
}

impl Suit {
    const ALL: [Suit; 3] = [Suit::Diamond, Suit::Club, Suit::Spade];
}
