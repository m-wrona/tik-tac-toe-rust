use rand::prelude::*;

use crate::game::{Error, NO_MOVE, NO_PLAYER, Player, PlayerID, WinningCoordinate};

const AI_SCORE_LOST: i8 = -10;
const AI_SCORE_DRAW: i8 = 0;
const AI_SCORE_WIN: i8 = 10;
const AI_SCORE_DISTURB: i8 = 10;

#[derive(Debug)]
pub struct AiPlayer {
    player_id: PlayerID,
    random: ThreadRng,
    random_moves: bool,
}

impl AiPlayer {
    pub fn new(id: PlayerID, random_moves: bool) -> Self {
        Self {
            player_id: id,
            random: thread_rng(),
            random_moves,
        }
    }

    fn evaluate_next_move(&self, b: crate::game::Board, coordinates: WinningCoordinate) {
        let mut free: i8 = 0;
        let mut other_player: i8 = 0;
        let mut score = AI_SCORE_DRAW;
        let mut next_coordinate = NO_MOVE;
        for coordinate in coordinates {
            if b[coordinate] != NO_PLAYER {
                if next_coordinate == NO_MOVE {
                    next_coordinate = i8::try_from(coordinate).unwrap()
                }
                free += 1;
            } else if b[coordinate] != self.player_id {
                score = AI_SCORE_LOST;
                other_player += 1;
            }
        }
        if other_player == 0 && free == 1 {
            //certain win
            score = AI_SCORE_WIN
        } else if score == AI_SCORE_LOST && next_coordinate != NO_MOVE {
            //lost or a draw
            if other_player == 2 {
                //try to prevent other player to win
                score = AI_SCORE_DISTURB
            }
        } else if free > 0 {
            //still can win
            score = AI_SCORE_WIN / free
        }
    }
}

impl Player for AiPlayer {
    fn id(&self) -> PlayerID {
        return self.player_id;
    }

    fn next_move(&self, b: crate::game::Board) -> Result<crate::game::BoardField, Error> {
        todo!()
    }
}
