const MAX_VALUES: usize = 13;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(values: usize) -> Deck {
        let mut d = Deck::new_empty();
        for suit in Suit::ALL {
            for value in 1..=values {
                let c = Card {
                    suit,
                    value: Value(value),
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
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Diamond,
    Club,
    Spade,
}

impl Suit {
    const ALL: [Suit; 3] = [Suit::Diamond, Suit::Club, Suit::Spade];
}
