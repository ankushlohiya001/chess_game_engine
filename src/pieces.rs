#![allow(dead_code)]

use std::marker::PhantomData;

use crate::errors::GameError;
use crate::moves::{Moving, Pos};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

pub struct King;
pub struct Queen;
pub struct Knight;
pub struct Rook;
pub struct Bishop;
pub struct Pawn;

#[derive(Debug, Clone, Copy)]
pub struct Piece<R> {
    character: PhantomData<R>,
    position: Pos,
    side: Side,
}

impl<T> Piece<T> {
    pub fn new(position: Pos, side: Side) -> Self {
        Piece {
            character: PhantomData,
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

impl Moving for Piece<King> {
    fn possible_moves(&self) -> Vec<Pos> {
        vec![]
    }

    fn can_move(&self, new_pos: Pos) -> bool {
        false
    }
}

#[test]
fn testing() {
    // let king = Piece<> {
    //     position: Pos('e', 1),
    //     side: Side::White,
    // };
    //
    // assert_eq!(king.pos_at_matrix(), (7, 4));
}
