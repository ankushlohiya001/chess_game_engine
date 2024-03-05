use crate::errors::GameError;

pub use crate::chess_matrix::Pos;

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
