use std::{fmt, io};

use crate::card::Card;

pub struct Player<'a> {
    name: String,
    hand: Option<Vec<&'a Card<'a>>>
}
}