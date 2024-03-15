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

#[derive(Debug, Clone)]
pub struct ChessBoard {
    matrix: [Option<Character>; 64],
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard { matrix: [None; 64] }
    }

    fn index_from_pos(pos: Pos) -> usize {
        let (row, col) = pos.at_matrix();
        row * 8 + col
    }

    pub fn character_at(&self, pos: Pos) -> Option<Character> {
        let index = ChessBoard::index_from_pos(pos);
        self.matrix[index]
    }

    pub fn pick_character(&mut self, pos: Pos) -> Result<Character, GameError> {
        if let Some(chracter) = self.character_at(pos) {
            // somehow replace these lines as are redundent
            let index = ChessBoard::index_from_pos(pos);
            self.matrix[index] = None;

            Ok(chracter)
        } else {
            Err(GameError::EmptyCell)
        }
    }

    pub fn place_character(&mut self, character: Character, pos: Pos) -> Result<(), GameError> {
        let index = ChessBoard::index_from_pos(pos);
        if self.matrix[index].is_none() {
            self.matrix[index] = Some(character);
            Ok(())
        } else {
            Err(GameError::OccupiedCell)
        }
    }

    pub fn show(&self) {
        // don't read this code :DD
        let mut board_str = (0..8).fold(String::new(), |mut st, r| {
            st.push(('0' as u8 + 8 - r) as char);
            st.push('|');
            let mut file = (0..8)
                .map(|i| self.matrix[i].map_or(' ', |x| x.symbol()))
                .fold(st, |mut st, c| {
                    st.push(c);
                    st.push('|');
                    st
                });

            file.push('\n');
            file
        });

        board_str.push_str("  ");
        let files = (0..8)
            .map(|x| ('a' as u8 + x) as char)
            .fold(board_str, |mut st, c| {
                st.push(c);
                st.push(' ');
                st
            });
        println!("{}", files);
    }
}

impl Default for ChessBoard {
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

    let mut board = ChessBoard::new();

    for i in 1..=8 {
        board.place_character(Character::King(Side::White), Pos('a', i));
        board.place_character(Character::King(Side::Black), Pos('b', i));
        board.place_character(Character::Queen(Side::White), Pos('c', i));
        board.place_character(Character::Queen(Side::Black), Pos('d', i));
    }

    board.show();

    assert!(false);
}
