#![allow(dead_code)]

use std::{borrow::BorrowMut, cell::RefCell, mem, ops::DerefMut};

use crate::{
    characters,
    chess_board::ChessBoard,
    errors::GameError,
    game::{self, Game},
    moves::{Moving, Pos},
    pieces,
};

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

    pub fn is_white(&self) -> bool {
        self.side() == Side::White
    }

    pub fn is_black(&self) -> bool {
        !self.is_white()
    }

    pub fn symbol(&self) -> char {
        use characters::*;
        let symbol = match self {
            Self::King(_) => King,
            Self::Queen(_) => Queen,
            Self::Knight(_) => Knight,
            Self::Rook(_) => Rook,
            Self::Bishop(_) => Bishop,
            Self::Pawn(_) => Pawn,
        };
        if self.is_white() {
            symbol.0
        } else {
            symbol.1
        }
    }

    pub fn same_side(character_a: &Character, character_b: &Character) -> bool {
        character_a.side() == character_b.side()
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub character: Character,
    pub position: Pos,
    pub side: Side,
    pub surrounding: Option<RefCell<ChessBoard>>,
}

impl Piece {
    pub fn new(character: Character, position: Pos, surrounding: Option<ChessBoard>) -> Self {
        let side = character.side();
        Piece {
            character,
            position,
            side,
            surrounding: surrounding.map(RefCell::new),
        }
    }

    pub fn new_alone(character: Character, position: Pos) -> Self {
        Self::new(character, position, None)
    }

    pub fn same_side(piece_a: &Piece, piece_b: &Piece) -> bool {
        piece_a.side == piece_b.side
    }

    pub fn place_at(&self, game: &mut Game, file: char, rank: u8) -> Result<(), GameError> {
        let pos = Pos(file, rank);
        if self.can_move(pos) {
            if let Some(ref surrounding_ref) = self.surrounding {
                let mut surrounding = surrounding_ref.borrow_mut();
                let res = surrounding.place_character(self.character, pos);
                game.board = mem::take(surrounding.deref_mut());
                res
            } else {
                Err(GameError::AlonePiece)
            }
        } else {
            Err(GameError::InvalidMove)
        }
    }

    pub fn place_back(&self, game: &mut Game) {
        let pos = self.position;
        self.place_at(game, pos.file(), pos.rank()).unwrap();
    }
}

impl Moving for Piece {
    fn character(&self) -> Character {
        self.character
    }

    fn current_position(&self) -> Pos {
        self.position
    }

    fn surrounding(&self) -> &mut ChessBoard {
        self.surrounding.as_ref().unwrap().get_mut()
    }

    fn can_move(&self, new_pos: Pos) -> bool {
        self.possible_moves().contains(&new_pos)
    }
}

#[test]
fn piece_test() {
    let piece = Piece::new_alone(Character::King(Side::White), Pos('a', 1));

    assert_eq!(piece.side, Side::White);
    assert_eq!(piece.position, Pos('a', 1));

    assert!(piece.can_move(Pos('b', 2)));
}
