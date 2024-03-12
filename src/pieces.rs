#![allow(dead_code)]

use crate::moves::{cross_move, plus_move, Moving, Pos};

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
    pub fn new(character: Character, position: Pos) -> Self {
        let side = character.side();
        Piece {
            character,
            position,
            side,
        }
    }
}

impl Moving for Piece {
    fn possible_moves(&self) -> Vec<Pos> {
        match self.character {
            Character::Bishop(_) => cross_move(self.position),
            Character::Rook(_) => plus_move(self.position),
            _ => vec![],
        }
    }

    fn can_move(&self, new_pos: Pos) -> bool {
        true
    }
}

#[test]
fn piece_test() {
    let piece = Piece::new(Character::King(Side::White), Pos('a', 1));

    assert_eq!(piece.side, Side::White);
    assert_eq!(piece.position, Pos('a', 1));
}
