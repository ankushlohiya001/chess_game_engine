#![allow(dead_code)]

use crate::chess_matrix::{ChessMatrix, Pos};
use crate::errors::GameError;
use crate::pieces::{Piece, Side};

pub struct Game {
    matrix: ChessMatrix,
    side: Side,
    move_done: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            matrix: ChessMatrix::new(),
            side: Side::White,
            move_done: false,
        }
    }

    fn place_characters(&mut self) {}

    pub fn start(&mut self) {
        self.start_with(Side::White);
    }

    pub fn start_with(&mut self, side: Side) {
        self.side = side;
        self.place_characters();
    }

    pub fn whose_turn(&self) -> Side {
        self.side
    }

    pub fn show_board(&self) {
        todo!("somehow show board to user")
    }

    pub fn select(&mut self, file: char, rank: u8) -> Result<Piece, GameError> {
        // select a character / return an error
        if !self.move_done {
            let pos = Pos(file, rank);
            let maybe_character = self.matrix.character_at(pos);
            if let Some(character) = maybe_character {
                if character.side() == self.side {
                    Ok(Piece::new(character, pos, character.side()))
                } else {
                    Err(GameError::OpponentPiece)
                }
            } else {
                Err(GameError::EmptyCell)
            }
        } else {
            Err(GameError::SideNotChanged)
        }
    }

    pub fn change_side(&mut self) -> Result<(), GameError> {
        if self.move_done {
            self.side = match self.side {
                Side::White => Side::Black,
                Side::Black => Side::White,
            };
            self.move_done = false;
            Ok(())
        } else {
            Err(GameError::SideAlreadyChanged)
        }
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

#[test]
fn game_test() {
    let mut game = Game::new();
    game.start();
    let res = game.change_side();
    assert_eq!(res, Err(GameError::SideAlreadyChanged));

    let whose_turn = game.whose_turn();
    assert_eq!(whose_turn, Side::White);
}
