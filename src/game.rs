pub type Error = String;
pub type PlayerID = usize;
pub type Board = [PlayerID; BOARD_SIZE];
type Players = [PlayerID; PLAYERS_COUNT];
pub type Coordinate = usize;
pub type WinningCoordinates = [Coordinate; 3];

const BOARD_SIZE: usize = 9;
const PLAYERS_COUNT: usize = 2;
pub const NO_PLAYER: PlayerID = 0;
pub const NO_MOVE: Coordinate = BOARD_SIZE + 1;
pub const WINNING_COORDINATES: [WinningCoordinates; 8] = [
    //rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    //columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    //diagonals
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Clone)]
pub struct State {
    players: Players,
    board: Board,
}

impl State {
    pub fn new(p1: PlayerID, p2: PlayerID) -> Self {
        Self {
            players: [p1, p2],
            board: [
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            ],
        }
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn make_move(&self, player_id: PlayerID, x: Coordinate) -> Result<State, Error> {
        if player_id != self.players[0] && player_id != self.players[1] {
            return Err(format!("player {} doesn't play this game", player_id));
        }

        let (winner, finished) = self.is_finished();
        if finished {
            return if winner == NO_PLAYER {
                Err("game has finished with a draw".to_string())
            } else {
                Err(format!("player {} has already won the game", winner))
            };
        }

        if x >= BOARD_SIZE {
            return Err(format!(
                "player {} made a move outside of board: {}",
                player_id, x
            ));
        } else if self.board[x] != NO_PLAYER {
            return Err(format!(
                "player {} cannot mark field {} since it's already taken by player {}",
                player_id, x, self.board[x],
            ));
        }
        let mut c = self.clone();
        c.board[x] = player_id;
        Ok(c)
    }

    pub fn is_finished(&self) -> (PlayerID, bool) {
        let mut is_finished = true;
        for coordinates in WINNING_COORDINATES {
            let x = self.board[coordinates[0]];
            let y = self.board[coordinates[1]];
            let z = self.board[coordinates[2]];
            if x == NO_PLAYER || y == NO_PLAYER || z == NO_PLAYER {
                is_finished = false;
            } else if x == y && x == z && y == z {
                return (x, true);
            }
        }
        (NO_PLAYER, is_finished)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{PlayerID, State, NO_PLAYER, WINNING_COORDINATES};

    #[test]
    fn should_allow_player_to_make_a_move() {
        let s0 = State::new(1, 2);
        let id: PlayerID = 1;
        let s1 = s0.make_move(id, 0).unwrap();

        assert_eq!(
            s0.board,
            [
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            ],
            "invalid initial state"
        );
        assert_eq!(
            s0.is_finished(),
            (NO_PLAYER, false),
            "game shouldn't be finished"
        );

        assert_eq!(
            s1.board,
            [
                id, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            ],
            "invalid state after move"
        );
        assert_eq!(
            s1.is_finished(),
            (NO_PLAYER, false),
            "game shouldn't be finished"
        );
    }

    #[test]
    fn should_not_make_a_move_when_game_finished() {
        let id: PlayerID = 5;
        let mut s0 = State::new(id, 2);
        s0.board = [
            id, id, id, //no formatting
            NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
        ];

        let s1 = s0.make_move(id, 4);
        assert_eq!(s1.is_err(), true, "move should fail");
        assert_eq!(s1.unwrap_err(), "player 5 has already won the game");
    }

    #[test]
    fn should_not_make_a_move_when_field_taken() {
        let id: PlayerID = 5;
        let id2: PlayerID = 7;
        let mut s0 = State::new(id, id2);
        s0.board = [
            id2, id, id, //no formatting
            NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
        ];

        let s1 = s0.make_move(id, 0);
        assert_eq!(s1.is_err(), true, "move should fail");
        assert_eq!(
            s1.unwrap_err(),
            "player 5 cannot mark field 0 since it's already taken by player 7"
        );
    }

    #[test]
    fn should_not_allow_to_make_a_move_by_unknown_player() {
        let s0 = State::new(1, 2);
        let s1 = s0.make_move(7, 0);

        assert_eq!(s1.is_err(), true, "move should fail");
        assert_eq!(s1.unwrap_err(), "player 7 doesn't play this game");
    }

    #[test]
    fn should_finish_game() {
        let id: PlayerID = 5;
        let s0 = State::new(id, 2);
        let s1 = s0
            .make_move(id, 0)
            .and_then(|s| s.make_move(id, 1))
            .and_then(|s| s.make_move(id, 2))
            .unwrap();

        assert_eq!(
            s1.board,
            [
                id, id, id, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
                NO_PLAYER, NO_PLAYER, NO_PLAYER, //no formatting
            ],
            "invalid state after move"
        );
        assert_eq!(s1.is_finished(), (id, true), "game must be finished");
    }

    #[test]
    fn should_win_the_game_using_winning_coordinates() {
        let id: PlayerID = 5;
        let s0 = State::new(id, 2);
        for coordinates in WINNING_COORDINATES {
            let s1 = s0
                .make_move(id, coordinates[0])
                .and_then(|s| s.make_move(id, coordinates[1]))
                .and_then(|s| s.make_move(id, coordinates[2]))
                .unwrap();
            assert_eq!(s1.is_finished(), (id, true), "game must be finished");
        }
    }
}
