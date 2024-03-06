use crate::pieces::{Character, Side};

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

    fn index_from_rowcol((row, col): (usize, usize)) -> usize {
        row * 8 + col
    }

    pub fn piece_at(&self, pos: Pos) -> Option<Character> {
        let index = ChessMatrix::index_from_rowcol(pos.at_matrix());
        self.matrix[index]
    }
}

impl Default for ChessMatrix {
    fn default() -> Self {
        Self::new()
    }
}
