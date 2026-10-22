use crate::{Deck, Player};

const DEFAULT_DECKSIZE: usize = 12;
const DEFAULT_PLAYER_N: usize = 6;

#[derive(Clone, Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new(DEFAULT_DECKSIZE),
            players: vec![Player::default(); DEFAULT_PLAYER_N],
        }
    }

    pub fn deal() -> Result<(), String> {
        todo!()
    }
}
