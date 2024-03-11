use crate::errors::GameError;

pub use crate::chess_matrix::Pos;

// TODOs
// move manager can perform moves
// based on pattern specified in chess,
//  ie. e2, moving pawn to e2,
//      Kg4, moving King to g4
//
// also records so as to perform undos/redos

pub trait Moving {
    fn possible_moves(&self) -> Vec<Pos>;

    fn can_move(&self, new_pos: Pos) -> bool;

    fn move_to(&mut self, new_pos: Pos) -> Result<(), GameError> {
        Err(GameError::InvalidMove)
    }
}

// pub fn king_move(pos: Pos) -> Vec<Pos> {
//     let mut moves = Vec::with_capacity(9);
//
//     let file = pos.file();
//     let rank = pos.rank();
//
//     for i in -1..2 {
//         for j in -1..2 {
//             let d_file = (file as i32 + i) as u8 as char;
//             let d_rank = (rank as i32 + j) as u8;
//             if !(d_file == file && d_rank == rank) && Pos::is_valid(d_file, d_rank) {
//                 moves.push(Pos(d_file, d_rank));
//             }
//         }
//     }
//     moves
// }

pub fn cross_move(pos: Pos) -> Vec<Pos> {
    let mut moves = Vec::with_capacity(9);

    let file = pos.file();
    let rank = pos.rank();

    for i in 1..=8 {
        let d_file = (file as i32 + i) as u8 as char;
        let d_rank = (rank as i32 + i) as u8;
        if Pos::is_valid(d_file, d_rank) {
            moves.push(Pos(d_file, d_rank));
        }
    }

    moves
}

#[test]
fn test_moves() {
    let moves = cross_move(Pos('a', 1));

    assert_eq!(moves, vec![Pos('a', 2), Pos('b', 1), Pos('b', 2)]);
}
