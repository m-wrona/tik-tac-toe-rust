use rand::prelude::*;

use crate::game::{Coordinate, Error, NO_MOVE, NO_PLAYER, Player, PlayerID, WINNING_COORDINATES, WinningCoordinates};

pub type Score = i8;

const AI_SCORE_LOST: Score = -10;
const AI_SCORE_DRAW: Score = 0;
const AI_SCORE_WIN: Score = 10;
const AI_SCORE_DISTURB: Score = 10;

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

    fn evaluate_next_move(&self, b: crate::game::Board, coordinates: WinningCoordinates) -> (Coordinate, Score) {
        let mut free: i8 = 0;
        let mut other_player: i8 = 0;
        let mut score = AI_SCORE_DRAW;
        let mut next_coordinate = NO_MOVE;
        for coordinate in coordinates {
            if b[coordinate] != NO_PLAYER {
                if next_coordinate == NO_MOVE {
                    next_coordinate = coordinate
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
        return (next_coordinate, score);
    }
}

impl Player for AiPlayer {
    #[inline]
    fn id(&self) -> PlayerID {
        return self.player_id;
    }

    fn next_move(&self, b: crate::game::Board) -> Result<Coordinate, Error> {
        let mut best_score = AI_SCORE_LOST;
        let mut best_next_moves = Vec::new();

        for coordinates in WINNING_COORDINATES {
            let (next_move, score) = self.evaluate_next_move(b, coordinates);
            if score > best_score {
                best_score = score;
                best_next_moves.clear();
                best_next_moves.push(next_move);
            } else if score == best_score {
                best_next_moves.push(next_move);
            }
        }

        //TODO add random moves
        return Ok(best_next_moves.pop().unwrap_or(NO_MOVE));
    }
}
