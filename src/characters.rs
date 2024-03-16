#![allow(non_upper_case_globals)]
use crate::moves::dirs::*;

pub const King: (char, char) = ('♔', '♚');
pub const Queen: (char, char) = ('♕', '♛');
pub const Rook: (char, char) = ('♖', '♜');
pub const Bishop: (char, char) = ('♗', '♝');
pub const Knight: (char, char) = ('♘', '♞');
pub const Pawn: (char, char) = ('♙', '♟');

pub const King_Move: Vec<Dir> = vec![
    TOP_LEFT, TOP, TOP_RIGHT, MID_LEFT, MID, MID_RIGHT, BOT_LEFT, BOT, BOT_RIGHT,
];

pub const Queen_Move: Vec<Dir> = vec![
    TOP_LEFT, TOP, TOP_RIGHT, MID_LEFT, MID, MID_RIGHT, BOT_LEFT, BOT, BOT_RIGHT,
];

pub const Rook_Move: Vec<Dir> = vec![TOP, MID_RIGHT, BOT, MID_LEFT];

pub const Bishop_Move: Vec<Dir> = vec![TOP_LEFT, TOP_RIGHT, BOT_RIGHT, BOT_LEFT];

pub const Knight_Move: Vec<Dir> = vec![
    TWO_TOP_LEFT,
    TWO_TOP_RIGHT,
    TWO_RIGHT_TOP,
    TWO_RIGHT_BOT,
    TWO_BOT_RIGHT,
    TWO_BOT_LEFT,
    TWO_LEFT_BOT,
    TWO_LEFT_TOP,
];

pub const Pawn_First_Move: Vec<Dir> = vec![TOP, TWO_TOP, ONE_TOP_LEFT, ONE_TOP_RIGHT];

pub const Pawn_Move: Vec<Dir> = vec![TOP, ONE_TOP_LEFT, ONE_TOP_RIGHT];
