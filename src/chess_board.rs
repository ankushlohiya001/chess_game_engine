use std::{fmt::Debug, ops::RangeInclusive};

use crate::{
    characters::positions,
    errors::GameError,
    moves::Moving,
    pieces::{Character, Piece, Side},
};

const FILE_RANGE: RangeInclusive<u8> = ('a' as u8)..=('h' as u8);
const RANK_RANGE: RangeInclusive<u8> = 1..=8;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl TryFrom<&str> for Pos {
    type Error = GameError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_lowercase();
        Pos::new(
            s.chars().next().unwrap(),
            s.chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
        )
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
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

    pub fn place_character_init(&mut self) {
        for pos in positions::Bishop {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::Bishop(side), pos).unwrap();
        }

        for pos in positions::Rook {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::Rook(side), pos).unwrap();
        }

        for pos in positions::Knight {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::Knight(side), pos).unwrap();
        }

        for pos in positions::King {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::King(side), pos).unwrap();
        }

        for pos in positions::Queen {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::Queen(side), pos).unwrap();
        }

        for pos in positions::Pawn {
            let side = if pos.rank() < 4 {
                Side::White
            } else {
                Side::Black
            };
            self.place_character(Character::Pawn(side), pos).unwrap();
        }
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
                .map(|i| self.matrix[8 * r as usize + i].map_or(' ', |x| x.symbol()))
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
    board.place_character_init();
    // board.place_character(Character::Pawn(Side::White), Pos('c', 2));

    let pos: Pos = Pos::try_from("f8").unwrap();

    println!("{:?}", pos.rank());
    println!("{:?}", pos.file());
}
