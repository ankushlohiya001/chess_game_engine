#![allow(dead_code)]

use crate::character::{self, *};

pub enum GameError {
    InvalidMove,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

pub struct Game {
    side: Side,
    sel_character: Option<Character>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            side: Side::White,
            sel_character: None,
        }
    }

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

    pub fn select(&mut self, character: Character) -> Result<Character, GameError> {
        // select a character / return an error
        Err(GameError::InvalidMove)
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
