mod card;
mod player;

use card::{Card, Deck, Suit, CardValue};
use player::Player;

fn main() {
    // start with having 6 players and 36 cards, 5 cards in each hand and 6 card river
    let deck = Deck::new(12);
    print!("{:?}", &deck);
}

fn deal(players: Vec<Player>) {
    todo!()
}
