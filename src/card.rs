use std::fmt;
use std::slice::Iter;
use self::Rank::*;
use self::Suit::*;

pub struct Card<'a> {
    rank: &'a Rank,
    suit: &'a Suit
}

pub enum Rank {
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
        EIGHT,
        NINE,
        TEN,
        JACK,
        QUEEN,
        KING,
        ACE];
        return RANKS.iter();
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::TWO => write!(f, "2 "),
            Rank::THREE => write!(f, "3 "),
            Rank::FOUR => write!(f, "4 "),
            Rank::FIVE => write!(f, "5 "),
            Rank::SIX => write!(f, "6 "),
            Rank::SEVEN => write!(f, "7 "),
            Rank::EIGHT => write!(f, "8 "),
            Rank::NINE => write!(f, "9 "),
            Rank::TEN => write!(f, "10 "),
            Rank::JACK => write!(f, "J "),
            Rank::QUEEN => write!(f, "Q "),
            Rank::KING => write!(f, "K "),
            Rank::ACE => write!(f, "A ")
        }
    }
}

pub enum Suit {
    SPADE = 4,
    HEART = 3,
    DIAMOND = 2,
    CLUB = 1
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [SPADE, HEART, DIAMOND, CLUB];
        return SUITS.iter();
    }
}


impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::SPADE => write!(f, "of Spades"),
            Suit::HEART => write!(f, "of Hearts"),
            Suit::DIAMOND => write!(f, "of Diamonds"),
            Suit::CLUB => write!(f, "of Clubs")
        }
    }
}
}