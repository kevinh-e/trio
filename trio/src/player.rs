use crate::card::{Card, CardValue};

#[derive(Debug, Clone)]
pub struct Player {
    cards: Vec<Card>,
    trios: [CardValue; 3],
}
