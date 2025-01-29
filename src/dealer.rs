use crate::{card::{Card, Rank, Suit}, player::Player};
use rand::{prelude::*, rng};

pub struct Dealer<'a> {
    deck: Vec<Card<'a>>,
}

impl<'a> Dealer<'a> {
    pub fn new() -> Dealer<'a> {
        let mut d: Vec<Card<'a>> = Vec::new();
        for r in Rank::iterator() {
            for s in Suit::iterator() {
                let temp_card = Card::new(r, s);
                d.push(temp_card);
            }
        }
        return Dealer{deck: d}
    }

    pub fn deal(&'a self, players: &mut Vec<Box<Player<'a>>>, num_cards: u32) {
        for _ in 0..num_cards {
            for i in 0..players.len() {
                let current = players.get_mut(i).unwrap();
                if let Some(card) = &self.draw() {
                    current.add_card(card);
                }
            }
        }
    }

    pub fn draw(&self) -> Option<&Card<'a>> {
        let mut rng = rng();
        let index: usize = rng.random_range(0..51);
        self.deck.get(index)
    }

    pub fn clean(&'a self, players: &mut Vec<Box<Player<'a>>>) {
        for i in 0..players.len() {
            let current = players.get_mut(i).unwrap();
            current.clear();
        }
    }
}