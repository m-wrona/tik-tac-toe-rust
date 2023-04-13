use crate::game::{Board, Coordinate, Error, PlayerID};

pub trait Player {
    fn id(&self) -> PlayerID;
    fn next_move(&self, b: Board) -> Result<Coordinate, Error>;
}