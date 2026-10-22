mod card;
mod game;
mod player;

use card::{Card, CardValue, Deck, Suit};
use game::Game;
use player::Player;

fn main() {
    // start with having 6 players and 36 cards, 5 cards in each hand and 6 card river
    let game = Game::new();
}
