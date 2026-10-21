use crate::card::{Card, CardValue};

#[derive(Debug, Clone)]
pub struct Player {
    cards: Vec<Card>,
    trios: Vec<CardValue>,
}

impl Player {
    pub fn new_empty() -> Player {
        Player {
            cards: Vec::new(),
            trios: Vec::new(),
        }
    }
    pub fn 
}
