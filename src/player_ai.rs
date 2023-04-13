use rand::prelude::*;

use crate::game::{
    Coordinate, Error, PlayerID, WinningCoordinates, NO_MOVE, NO_PLAYER, WINNING_COORDINATES,
};
use crate::player::Player;

pub type Score = i8;
pub type NextMoveStrategy = fn(possible_moves: Vec<Coordinate>) -> Coordinate;

const AI_SCORE_LOST: Score = -10;
const AI_SCORE_DRAW: Score = 0;
const AI_SCORE_WIN: Score = 10;
const AI_SCORE_DISTURB: Score = 10;

#[derive(Debug)]
pub struct AIPlayer {
    player_id: PlayerID,
    move_strategy: NextMoveStrategy,
}

impl AIPlayer {
    pub fn first_move_strategy() -> NextMoveStrategy {
        return |possible_moves: Vec<Coordinate>| -> Coordinate {
            return possible_moves.clone().swap_remove(0);
        };
    }

    pub fn random_move_strategy() -> NextMoveStrategy {
        return |possible_moves: Vec<Coordinate>| -> Coordinate {
            let next: usize = random();
            let idx: usize = next % possible_moves.len();
            return possible_moves.clone().swap_remove(idx);
        };
    }

    pub fn new(id: PlayerID) -> Self {
        return Self::new_with_strategy(id, Self::first_move_strategy());
    }

    pub fn new_with_strategy(player_id: PlayerID, move_strategy: NextMoveStrategy) -> Self {
        Self {
            player_id,
            move_strategy,
        }
    }

    fn evaluate_next_move(
        &self,
        b: crate::game::Board,
        coordinates: WinningCoordinates,
    ) -> (Coordinate, Score) {
        let mut free: i8 = 0;
        let mut other_player: i8 = 0;
        let mut score = AI_SCORE_DRAW;
        let mut next_coordinate = NO_MOVE;
        for coordinate in coordinates {
            if b[coordinate] == NO_PLAYER {
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

impl Player for AIPlayer {
    #[inline]
    fn id(&self) -> PlayerID {
        return self.player_id;
    }

    fn next_move(&self, b: crate::game::Board) -> Result<Coordinate, Error> {
        let mut best_score = AI_SCORE_LOST;
        let mut best_moves = Vec::new();

        for coordinates in WINNING_COORDINATES {
            let (next_move, score) = self.evaluate_next_move(b, coordinates);
            if score > best_score {
                best_score = score;
                best_moves.clear();
                best_moves.push(next_move);
            } else if score == best_score {
                best_moves.push(next_move);
            }
        }

        if best_moves.is_empty() {
            return Ok(NO_MOVE);
        }

        let next_move = (self.move_strategy)(best_moves);
        return Ok(next_move);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{PlayerID, NO_MOVE, NO_PLAYER};
    use crate::player::Player;
    use crate::player_ai::{AIPlayer, AI_SCORE_DISTURB, AI_SCORE_LOST, AI_SCORE_WIN};

    #[test]
    fn should_evaluate_first_winning_move() {
        let id: PlayerID = 1;
        let ai = AIPlayer::new(id);
        let (next_move, score) = ai.evaluate_next_move(
            [
                NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
                NO_PLAYER, NO_PLAYER,
            ],
            [0, 1, 2],
        );
        assert_eq!(next_move, 0, "wrong first move");
        assert_eq!(score, AI_SCORE_WIN / 3, "wrong score");
    }

    #[test]
    fn should_evaluate_second_winning_move() {
        let id: PlayerID = 1;
        let ai = AIPlayer::new(id);
        let (next_move, score) = ai.evaluate_next_move(
            [
                id, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
                NO_PLAYER,
            ],
            [0, 1, 2],
        );
        assert_eq!(next_move, 1, "wrong first move");
        assert_eq!(score, AI_SCORE_WIN / 2, "wrong score");
    }

    #[test]
    fn should_evaluate_third_winning_move() {
        let id: PlayerID = 1;
        let ai = AIPlayer::new(id);
        let (next_move, score) = ai.evaluate_next_move(
            [
                id, id, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
            ],
            [0, 1, 2],
        );
        assert_eq!(next_move, 2, "wrong first move");
        assert_eq!(score, AI_SCORE_WIN, "wrong score");
    }

    #[test]
    fn should_evaluate_no_move_when_game_is_lost() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let (next_move, score) =
            ai.evaluate_next_move([id2, id2, id2, id2, id2, id2, id2, id2, id2], [0, 1, 2]);
        assert_eq!(next_move, NO_MOVE, "wrong first move");
        assert_eq!(score, AI_SCORE_LOST, "wrong score");
    }

    #[test]
    fn should_evaluate_a_move_when_game_is_a_draw() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let (next_move, score) = ai.evaluate_next_move(
            [
                id2, id2, id2, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
            ],
            [0, 1, 2],
        );
        assert_eq!(next_move, NO_MOVE, "wrong first move");
        assert_eq!(score, AI_SCORE_LOST, "wrong score");
    }

    #[test]
    fn should_evaluate_a_disturb_move_when_game_will_be_lost() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let (next_move, score) = ai.evaluate_next_move(
            [
                NO_PLAYER, id2, id2, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
                NO_PLAYER,
            ],
            [0, 1, 2],
        );
        assert_eq!(next_move, 0, "wrong first move");
        assert_eq!(score, AI_SCORE_DISTURB, "wrong score");
    }

    #[test]
    fn should_make_first_move() {
        let id: PlayerID = 1;
        let ai = AIPlayer::new(id);
        let next_move = ai
            .next_move([
                NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
                NO_PLAYER, NO_PLAYER,
            ])
            .unwrap();
        assert_eq!(next_move, 0, "wrong first move");
    }

    #[test]
    fn should_make_a_winning_move() {
        let id: PlayerID = 1;
        let ai = AIPlayer::new(id);
        let next_move = ai
            .next_move([
                NO_PLAYER, id, id, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER, NO_PLAYER,
            ])
            .unwrap();
        assert_eq!(next_move, 0, "wrong winning move");
    }

    #[test]
    fn should_make_a_draw_move() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let next_move = ai
            .next_move([NO_PLAYER, id2, id2, id, id, id2, id2, id, id])
            .unwrap();
        assert_eq!(next_move, 0, "wrong winning move");
    }

    #[test]
    fn should_make_a_disturb_move() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let next_move = ai
            .next_move([NO_PLAYER, id2, id2, id, NO_PLAYER, id2, id2, id, id])
            .unwrap();
        assert_eq!(next_move, 0, "wrong winning move");
    }

    #[test]
    fn should_make_a_disturb_move_variant_2() {
        let id: PlayerID = 1;
        let id2: PlayerID = 2;
        let ai = AIPlayer::new(id);
        let next_move = ai
            .next_move([id, id2, id2, id2, NO_PLAYER, id2, id2, id, NO_PLAYER])
            .unwrap();
        assert_eq!(next_move, 4, "wrong winning move");
    }
}
