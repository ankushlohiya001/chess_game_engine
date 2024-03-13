use std::ops::RangeInclusive;

use crate::{
    errors::GameError,
    pieces::{Character, Side},
};

const FILE_RANGE: RangeInclusive<u8> = ('a' as u8)..=('h' as u8);
const RANK_RANGE: RangeInclusive<u8> = 1..=8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos(pub char, pub u8);
impl Pos {
    pub fn new(file: char, rank: u8) -> Result<Pos, GameError> {
        if Pos::is_valid(file, rank) {
            Ok(Pos(file, rank))
        } else {
            Err(GameError::InvalidPosition)
        }
    }

    pub fn is_valid(file: char, rank: u8) -> bool {
        FILE_RANGE.contains(&(file as u8)) && RANK_RANGE.contains(&rank)
    }

    pub fn d_pos(&self, d_file: i32, d_rank: i32) -> Result<Pos, ()> {
        // need to refactor almost all stuff about this function
        let new_file = (self.file() as i32 + d_file) as u8 as char;
        let new_rank = (self.rank() as i32 + d_rank) as u8;
        if Pos::is_valid(new_file, new_rank) {
            Ok(Pos(new_file, new_rank))
        } else {
            Err(())
        }
    }

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

#[test]
fn pos_test() {
    let pos = Pos('a', 1);
    assert_eq!(pos.file(), 'a');
    assert_eq!(pos.rank(), 1);

    assert_eq!(pos.at_matrix(), (7, 0));

    // unsafe position
    let _pos = Pos('z', 100);

    // safe position
    let maybe_pos = Pos::new('z', 100);
    assert_eq!(maybe_pos, Err(GameError::InvalidPosition));
}
