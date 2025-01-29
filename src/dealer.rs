use crate::{card::{Card, Rank, Suit}, player::Player};
use rand::{prelude::*, rng};

pub struct Dealer<'a> {
    deck: Vec<Card<'a>>,
}
