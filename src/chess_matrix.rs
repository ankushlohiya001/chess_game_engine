use std::char;

use crate::{
    errors::GameError,
    pieces::{Character, Side},
};

#[derive(Debug, Clone, Copy)]
pub struct Pos(pub char, pub u8);
impl Pos {
    pub fn rank(&self) -> u8 {
        self.1
    }

    pub fn file(&self) -> char {
        self.0
    }

    pub fn at_matrix(&self) -> (usize, usize) {
        (
            (8 - self.rank()) as usize,
            self.file() as usize - 'a' as usize,
        )
    }
}

pub struct ChessMatrix {
    matrix: [Option<Character>; 64],
}

impl ChessMatrix {
    pub fn new() -> Self {
        ChessMatrix { matrix: [None; 64] }
    }

    fn index_from_rowcol(pos: Pos) -> usize {
        let (row, col) = pos.at_matrix();
        row * 8 + col
    }

    pub fn character_at(&self, pos: Pos) -> Option<Character> {
        let index = ChessMatrix::index_from_rowcol(pos);
        self.matrix[index]
    }

    pub fn pick_character(&mut self, pos: Pos) -> Result<Character, GameError> {
        if let Some(chracter) = self.character_at(pos) {
            // somehow replace these lines as are redundent
            let index = ChessMatrix::index_from_rowcol(pos);
            self.matrix[index] = None;

            Ok(chracter)
        } else {
            Err(GameError::EmptyCell)
        }
    }

    pub fn place_character(&mut self, character: Character, pos: Pos) -> Result<(), GameError> {
        let index = ChessMatrix::index_from_rowcol(pos);
        if self.matrix[index].is_none() {
            self.matrix[index] = Some(character);
            Ok(())
        } else {
            Err(GameError::OccupiedCell)
        }
    }
}

impl Default for ChessMatrix {
    fn default() -> Self {
        Self::new()
    }
}
