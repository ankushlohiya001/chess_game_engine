#![allow(dead_code)]

use crate::errors::GameError;
use crate::moves::{Moving, Pos};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Character {
    position: Pos,
    side: Side,
}

impl Character {
    pub fn pos_at_matrix(&self) -> (usize, usize) {
        let pos = self.position;
        (
            (8 - pos.rank()) as usize,
            pos.file() as usize - 'a' as usize,
        )
    }
}

#[test]
fn testing() {
    let king = Character {
        position: Pos('e', 1),
        side: Side::White,
    };

    assert_eq!(king.pos_at_matrix(), (7, 4));
}
