#![allow(dead_code)]

use crate::errors::GameError;
use crate::moves::{Moving, Pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Character {
    King(Side),
    Queen(Side),
    Knight(Side),
    Rook(Side),
    Bishop(Side),
    Pawn(Side),
}

impl Character {
    pub fn side(self) -> Side {
        match self {
            Self::King(side) => side,
            Self::Queen(side) => side,
            Self::Knight(side) => side,
            Self::Rook(side) => side,
            Self::Bishop(side) => side,
            Self::Pawn(side) => side,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub character: Character,
    pub position: Pos,
    pub side: Side,
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
