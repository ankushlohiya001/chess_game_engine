#![allow(dead_code)]

use std::mem;

use crate::chess_board::{ChessBoard, Pos};
use crate::errors::GameError;
use crate::moves::Moving;
use crate::pieces::{self, Character, Piece, Side};

pub struct Game {
    pub board: ChessBoard,
    side: Side,
    move_done: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: ChessBoard::new(),
            side: Side::White,
            move_done: false,
        }
    }

    fn place_pieces(&mut self) {}

    pub fn start(&mut self) {
        self.start_with(Side::White);
    }

    pub fn start_with(&mut self, side: Side) {
        self.side = side;
        self.place_pieces();
    }

    pub fn whose_turn(&self) -> Side {
        self.side
    }

    pub fn show_board(&self) {
        todo!("somehow show board to user")
    }

    pub fn pick(&mut self, file: char, rank: u8) -> Result<Piece, GameError> {
        // select a character / return an error
        if !self.move_done {
            let pos = Pos(file, rank);
            let maybe_character = self.board.pick_character(pos);
            if let Ok(character) = maybe_character {
                let piece = Piece::new(character, pos, Some(mem::take(&mut self.board)));
                if character.side() == self.side {
                    Ok(piece)
                } else {
                    piece.place_back(self);
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

    let maybe_piece = game.pick('a', 1);
    if let Err(error) = maybe_piece {
        assert_eq!(error, GameError::EmptyCell);
    }

    let piece = Piece::new_alone(Character::Pawn(Side::White), Pos('a', 1));

    let res = piece.place_at(&mut game, 'b', 2);

    println!("{:?}", res.err());

    // assert!(res.is_ok());

    let maybe_piece = game.pick('a', 1);
    if let Err(error) = maybe_piece {
        assert_eq!(error, GameError::EmptyCell);
    }

    let maybe_piece = game.pick('a', 2);
    assert!(maybe_piece.is_ok());
    let piece = maybe_piece.unwrap();
    assert_eq!(piece.position, Pos('a', 2));
    let moves = piece.possible_moves();

    println!("{:?}", moves);

    assert_eq!(0, 1);
}

// somehow moves most piece related stuff to piece module,
