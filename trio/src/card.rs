use std::fmt;

const MAX_VALUES: usize = 13;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(values: usize) -> Deck {
        let mut d = Deck::new_empty();
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

    pub fn new_empty() -> Deck {
        Deck {
            cards: Vec::with_capacity(MAX_VALUES * Suit::ALL.len()),
        }
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
            0 => "A",
            _ => stringify!(_ + 1),
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
