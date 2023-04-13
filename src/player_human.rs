use std::io;
use std::io::Stdin;
use std::str::FromStr;

use crate::game::{Coordinate, Error, NO_MOVE, PlayerID};
use crate::player::Player;

#[derive(Debug)]
pub struct HumanPlayer {
    player_id: PlayerID,
    io: Stdin,
}

impl HumanPlayer {
    pub fn new(player_id: PlayerID) -> Self {
        return Self {
            player_id,
            io: io::stdin(),
        };
    }
}

impl Player for HumanPlayer {
    #[inline]
    fn id(&self) -> PlayerID {
        return self.player_id;
    }

    fn next_move(&self, _: crate::game::Board) -> Result<Coordinate, Error> {
        let mut buffer = String::new();
        println!("Waiting for player {} move....", self.player_id);
        return match self.io.read_line(&mut buffer) {
            Ok(_) => {
                let next_move = usize::from_str(buffer.trim_end()).unwrap_or(NO_MOVE);
                Ok(next_move)
            }

            Err(err) => {
                Err(format!("user input error: {}", err))
            }
        };
    }
}
