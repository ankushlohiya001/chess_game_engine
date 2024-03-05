#![allow(dead_code)]

use crate::chess_matrix::Character;
use crate::errors::GameError;
use crate::moves::{Moving, Pos};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    character: Character,
    position: Pos,
    side: Side,
}

impl Piece {
    pub fn new(character: Character, position: Pos, side: Side) -> Self {
        Piece {
            character,
            position,
            side,
        }
    }

    pub fn pos_at_matrix(&self) -> (usize, usize) {
        let pos = self.position;
        (
            (8 - pos.rank()) as usize,
            pos.file() as usize - 'a' as usize,
        )
    }
}

impl Moving for Piece {
    fn possible_moves(&self) -> Vec<Pos> {
        vec![]
    }

    fn can_move(&self, new_pos: Pos) -> bool {
        false
    }
}
