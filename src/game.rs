use crate::dealer::Dealer;
use crate::player::Player;
use std::io;

pub struct Game<'a> {
    dealer: Dealer<'a>,
    pot: u32
}
