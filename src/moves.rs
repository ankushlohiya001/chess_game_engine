use crate::errors::GameError;

pub use crate::chess_matrix::Pos;

// TODOs
// move manager can perform moves
// based on pattern specified in chess,
//  ie. e2, moving pawn to e2,
//      Kg4, moving King to g4
//
// also records so as to perform undos/redos

pub trait Moving {
    fn possible_moves(&self) -> Vec<Pos>;

    fn can_move(&self, new_pos: Pos) -> bool;

    fn move_to(&mut self, new_pos: Pos) -> Result<(), GameError> {
        Err(GameError::InvalidMove)
    }
}

pub fn king_move(pos: Pos) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);
    moves
}
