use crate::card::{Card, CardValue};

#[derive(Debug, Clone, Default)]
pub struct Player {
    cards: Vec<Card>,
    trios: Vec<CardValue>,
}

impl Player {
    pub fn new() {
        !todo!()
    }
}
