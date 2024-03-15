use std::borrow::Borrow;

use crate::{
    characters::King,
    errors::GameError,
    pieces::{self, Character, Piece},
};

pub use crate::chess_board::Pos;

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

pub fn cross_move(piece: &Piece, infinite: bool) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);
    let pos = piece.position;
    let surounding = piece.surrounding.as_ref().unwrap().borrow();

    let max = if infinite { 8 } else { 1 };

    let mut dirs = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for i in 1..=max {
        for (index, (d_file, d_rank)) in dirs.iter().enumerate() {
            if let Ok(pos) = pos.d_pos(d_file * i, d_rank * i) {
                if let Some(nei) = surounding.character_at(pos) {
                    if Character::same_side(&piece.character, &nei) {
                        dirs.remove(index);
                        break;
                    }
                }
                moves.push(pos);
            }
        }
    }

    moves
}

const PLUS_MAP: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn plus_move(piece: &Piece, infinite: bool) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);
    let pos = piece.position;

    let max = if infinite { 8 } else { 1 };

    for i in 1..=max {
        // if let Ok(pos) = pos.d_pos(i) {
        //     moves.push(pos);
        // }
    }

    moves
}

pub fn two_n_half_move(piece: &Piece) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);
    let pos = piece.position;

    let file = pos.file();
    let rank = pos.rank();
    moves
}

pub fn one_two_move(piece: &Piece) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);
    let pos = piece.position;

    let file = pos.file();
    let rank = pos.rank();
    moves
}

#[test]
fn test_moves() {
    // let piece = Piece::new(Character::King, position, surrounding)
    // let moves = cross_move(pieces, true);
    //
    // assert_eq!(
    //     moves,
    //     vec![
    //         Pos('c', 3),
    //         Pos('c', 5),
    //         Pos('e', 3),
    //         Pos('e', 5),
    //         Pos('b', 2),
    //         Pos('b', 6),
    //         Pos('f', 2),
    //         Pos('f', 6),
    //         Pos('a', 1),
    //         Pos('a', 7),
    //         Pos('g', 1),
    //         Pos('g', 7),
    //         Pos('h', 8)
    //     ]
    // );
}
