use crate::card::{Card, Value};

#[derive(Debug, Clone)]
pub struct Player {
    cards: Vec<Card>,
    trios: [Value; 3],
}
