pub mod game {
    use std::fs::File;

    pub type Error = String;
    pub type PlayerID = usize;
    type Players = [PlayerID; PLAYERS_COUNT];
    type BoardField = usize;
    type Board = [PlayerID; BOARD_SIZE];
    type WinningCoordinate = [PlayerID; 3];

    pub const NO_PLAYER: PlayerID = 0;
    const PLAYERS_COUNT: usize = 2;
    const BOARD_SIZE: BoardField = 9;
    const WINNING_COORDINATES: [WinningCoordinate; 8] = [
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

    pub trait Player {
        fn id(&self) -> PlayerID;
        fn next_move(&self, b: Board) -> Result<BoardField, Error>;
    }

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
                    NO_PLAYER, NO_PLAYER, NO_PLAYER,
                    NO_PLAYER, NO_PLAYER, NO_PLAYER,
                    NO_PLAYER, NO_PLAYER, NO_PLAYER,
                ],
            }
        }

        pub fn make_move(&self, p: &dyn Player) -> Result<State, Error> {
            let (winner, finished) = self.is_finished();
            if finished {
                return if winner == NO_PLAYER {
                    Err(format!("game has finished with a draw"))
                } else {
                    Err(format!("player {} has already won the game", winner))
                };
            }

            return match p.next_move(self.board) {
                Err(err) => {
                    Err(format!("player {} couldn't make next move: {}", p.id(), err))
                }

                Ok(x) => {
                    if x >= BOARD_SIZE {
                        return Err(format!("player {} made a move outside of board: {}", p.id(), x));
                    } else if self.board[x] != NO_PLAYER {
                        return Err(format!(
                            "player {} cannot mark field {} since it's already taken by player {}",
                            p.id(), x, self.board[x],
                        ));
                    }
                    let mut c = self.clone();
                    c.board[x] = p.id();
                    Ok(c)
                }
            };
        }

        fn is_finished(&self) -> (PlayerID, bool) {
            let mut isFinished = true;
            for coordinates in WINNING_COORDINATES {
                let x = self.board[coordinates[0]];
                let y = self.board[coordinates[1]];
                let z = self.board[coordinates[2]];
                if x == NO_PLAYER || y == NO_PLAYER || z == NO_PLAYER {
                    isFinished = false;
                } else if x == y && x == z && y == z {
                    return (x, true);
                }
            }
            return (NO_PLAYER, isFinished);
        }
    }
}