use crate::dealer::Dealer;
use crate::player::Player;
use std::io;

pub struct Game<'a> {
    dealer: Dealer<'a>,
    pot: u32
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a>{
        let dealer = Dealer::new();
        let pot: u32 = 0;
        return Game{dealer, pot}
    }
    
    pub fn start(&'a mut self, players: &mut Vec<Box<Player<'a>>>) {
        loop {
            self.dealer.clean(players);
            self.pot = 0;
            {
                let current = &mut *players;
                self.dealer.deal(current, 2);
                for i in current {
                    println!("pot: {}", self.pot);
                    println!("Currently {}'s turn. What would you like to do?", i.get_name());
                    println!("{}", i);
                    println!("1: Bet\n2: Call\n3: Check\n4: Fold");
                    loop {
                        let mut action = String::new();
                        io::stdin().read_line(&mut action).expect("failed to readline");
                        match action.as_str().trim() {
                            "1" => {
                                self.pot += i.bet();
                                break;
                            },
                            "2" => println!("Not yet implemented!"),
                            "3" => println!("Not yet implemented!"),
                            "4" => {i.fold(); break;},
                            "5" => break,
                            _ => println!("Invalid input!")
                        }
                    }
                }
            }
            {
                println!("\n\nRound end!");
                let current = &mut *players;
                for i in current {
                    println!("{}", i);
                }
                println!("\n\n")
            }
        }
    }
}