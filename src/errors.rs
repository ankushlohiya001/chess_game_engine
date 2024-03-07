#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    EmptyCell,
    OccupiedCell,
    SideNotChanged,
    SideAlreadyChanged,
    OpponentPiece,
    InvalidMove,
    GameOver,
}
