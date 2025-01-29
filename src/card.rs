use std::fmt;
use std::slice::Iter;
use self::Rank::*;
use self::Suit::*;

pub struct Card<'a> {
    rank: &'a Rank,
    suit: &'a Suit
}
}