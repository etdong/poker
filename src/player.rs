use std::{fmt, io};

use crate::card::Card;

pub struct Player<'a> {
    name: String,
    hand: Option<Vec<&'a Card<'a>>>
}

impl<'a> Player<'a> {

    pub fn new(pname: String) -> Player<'a>{
        return Player {
            name: pname,
            hand: None
        }
    }

    pub fn new_hand(pname: String, phand: Vec<&'a Card>) -> Player<'a> {
        return Player {
            name: pname,
            hand: Some(phand)
        }
    }

    pub fn add_card(&mut self, newcard: &'a Card) {
        if self.hand.is_none() {
            self.hand.replace(vec![newcard]);
        } else {
            self.hand.as_mut().unwrap().push(newcard);
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn bet(&self) -> u32 {
        let mut amount = String::new();
        loop {
            println!("How much would you like to bet?");
            io::stdin().read_line(&mut amount).expect("failed to readline");
            match amount.trim().parse::<u32>() {
                Ok(n) => {
                    println!("{} bet ${}", self.get_name(), amount);
                    return n
                },
                Err(e) => print!("Enter a valid amount! {}\n", e)
            }
        }
    }

    pub fn fold(&mut self) {
        let _ = self.hand.take();
        println!("{} folded.", self.get_name())
    }

    pub fn clear(&mut self) {
        let _ = self.hand.take();
    }
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_string: Vec<String> = Vec::new();
        if let Some(hand) = &self.hand {
            for card in hand {
                card_string.push(format!("{}{}", card.rank(), card.suit()));
            }
        }
        write!(f, "{}'s hand:\n{:?}", &self.name, card_string)
    }
}