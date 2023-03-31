pub mod board {
    use std::fmt::Error;

    type PlayerID = u32;
    type Players = [PlayerID; PlayersCount];
    type BoardField = usize;
    type Board = [PlayerID; BoardSize];

    const PlayersCount: usize = 2;
    const BoardSize: BoardField = 9;

    pub trait Player {
        fn id(&self) -> PlayerID;
        fn nextMove(&self, b: Board) -> Result<BoardField, Error>;
    }

    #[derive(Debug, Clone)]
    pub struct State {
        players: Players,
        board: Board,
    }

    impl State {
        pub fn new(p1: PlayerID, p2: PlayerID) -> Self {
            Self {
                players: [p1,p2],
                board: [0,0,0,0,0,0,0,0,0],
            }
        }

        pub fn Move(&self, _p: &dyn Player) {
            todo!()
        }
    }
}