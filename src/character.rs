#![allow(dead_code)]

use crate::errors::GameError;

#[derive(Debug, Clone, Copy)]
pub struct Pos(u8, char);

#[derive(Debug)]
pub enum Character {
    King(Pos),
    Queen(Pos),
    Knight(Pos),
    Pawn(Pos),
    Bishop(Pos),
    Rook(Pos),
}

impl Character {
    pub fn possible_moves(&self) -> Vec<Pos> {
        Vec::new()
    }

    pub fn can_move(&self, new_pos: Pos) -> bool {
        false
    }

    pub fn move_to(&mut self, new_pos: Pos) -> Result<(), GameError> {
        Err(GameError::InvalidMove)
    }
}
