#![allow(dead_code)]

use crate::chess_matrix::{ChessMatrix, Pos};
use crate::errors::GameError;
use crate::moves::Moving;
use crate::pieces::{King, Piece, Side};

pub struct Game {
    matrix: ChessMatrix,
    side: Side,
}

impl Game {
    pub fn new() -> Game {
        Game {
            matrix: ChessMatrix::new(),
            side: Side::White,
        }
    }

    fn place_characters(&mut self) {}

    pub fn start(&self) {
        todo!("need to do something about which i'm unaware")
    }

    pub fn start_with(&mut self, side: Side) {
        self.side = side;
    }

    pub fn whose_turn(&self) -> Side {
        self.side
    }

    pub fn show_board(&self) {
        todo!("somehow show board to user")
    }

    pub fn select(&mut self, pos: Pos) -> Result<impl Moving, GameError> {
        // select a character / return an error
        let maybe_character = self.matrix.piece_at(pos);
        if let Some(character) = maybe_character {
            let piece = Piece::new(pos, Side::White);
            Ok(piece)
        } else {
            Err(GameError::EmptyCell)
        }
    }

    pub fn change_side(&mut self) {
        self.side = match self.side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    pub fn request_draw(&mut self) {
        todo!("to request draw")
    }

    pub fn resign(&mut self) {
        todo!("accepts defeat!")
    }

    pub fn is_game_over(&self) -> bool {
        todo!("provide way to tell this")
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
