use std::vec;

use player::Player;
use game::Game;

pub mod player;
pub mod card;
pub mod dealer;
pub mod game;

fn main() {
    let name1 = "t1".to_string();
    let p1 = Player::new(name1);
    let name2 = "t2".to_string();
    let p2 = Player::new(name2);
    let name3 = "t3".to_string();
    let p3 = Player::new(name3);

    let mut players: Vec<Box<Player>> = vec![Box::new(p1), Box::new(p2), Box::new(p3)];
    let mut g = Game::new();
    let current = &mut players;
    g.start(current);
}
