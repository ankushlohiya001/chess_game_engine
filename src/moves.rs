use crate::{
    characters::{
        Bishop_Move, King_Move, Knight_Move, Pawn_First_Move, Pawn_Move, Queen_Move, Rook_Move,
    },
    chess_board::ChessBoard,
    errors::GameError,
    pieces::Character,
};

pub use crate::chess_board::Pos;

pub mod dirs {
    pub type Dir = (i32, i32);

    pub const TOP_LEFT: Dir = (-1, 1);
    pub const TOP: Dir = (0, 1);
    pub const TOP_RIGHT: Dir = (1, 1);

    pub const MID_LEFT: Dir = (-1, 0);
    pub const MID: Dir = (0, 0);
    pub const MID_RIGHT: Dir = (1, 0);

    pub const BOT_LEFT: Dir = (-1, -1);
    pub const BOT: Dir = (0, -1);
    pub const BOT_RIGHT: Dir = (1, -1);

    pub const TWO_TOP_LEFT: Dir = (-1, 2);
    pub const TWO_TOP_RIGHT: Dir = (1, 2);

    pub const TWO_LEFT_TOP: Dir = (-2, 1);
    pub const TWO_LEFT_BOT: Dir = (-2, -1);

    pub const TWO_RIGHT_TOP: Dir = (2, 1);
    pub const TWO_RIGHT_BOT: Dir = (2, -1);

    pub const TWO_BOT_LEFT: Dir = (-1, -2);
    pub const TWO_BOT_RIGHT: Dir = (1, -2);

    pub const TWO_TOP: Dir = (0, 2);

    pub const ONE_TOP_LEFT: Dir = (-1, 1);
    pub const ONE_TOP_RIGHT: Dir = (1, 1);
}

// TODOs
// move manager can perform moves
// based on pattern specified in chess,
//  ie. e2, moving pawn to e2,
//      Kg4, moving King to g4
//
// also records so as to perform undos/redos
pub trait Moving {
    fn character(&self) -> Character;

    fn current_position(&self) -> Pos;

    fn surrounding(&self) -> &mut ChessBoard;

    fn possible_moves(&self) -> Vec<Pos> {
        match self.character() {
            Character::Bishop(_) => {
                let dirs = Bishop_Move.clone();
                self.move_maker(dirs, true)
            }
            Character::Queen(_) => {
                let dirs = Queen_Move.clone();
                self.move_maker(dirs, true)
            }
            Character::Rook(_) => {
                let dirs = Rook_Move.clone();
                self.move_maker(dirs, true)
            }
            Character::Knight(_) => {
                let dirs = Knight_Move.clone();
                self.move_maker(dirs, false)
            }
            Character::King(_) => {
                let dirs = King_Move.clone();
                self.move_maker(dirs, false)
            }
            Character::Pawn(_) => {
                let rank = self.current_position().rank();
                let dirs = if rank == 2 || rank == 7 {
                    Pawn_First_Move.clone()
                } else {
                    Pawn_Move.clone()
                };
                self.move_maker(dirs, false)
            }
        }
    }

    fn can_move(&self, new_pos: Pos) -> bool;

    fn move_to(&mut self, new_pos: Pos) -> Result<(), GameError> {
        Err(GameError::InvalidMove)
    }

    fn move_maker(&self, mut dirs: Vec<dirs::Dir>, infinite: bool) -> Vec<Pos> {
        let mut moves = Vec::with_capacity(9);
        let pos = self.current_position();
        let surounding = self.surrounding();

        let max = if infinite { 8 } else { 1 };

        for i in 1..=max {
            for (index, (d_file, d_rank)) in dirs.iter().enumerate() {
                if let Ok(pos) = pos.d_pos(d_file * i, d_rank * i) {
                    if let Some(nei) = surounding.character_at(pos) {
                        if !Character::same_side(&self.character(), &nei) {
                            moves.push(pos);
                        }
                        dirs.remove(index);
                        break;
                    }
                    moves.push(pos);
                }
            }
        }

        moves
    }
}

#[test]
fn test_moves() {}
