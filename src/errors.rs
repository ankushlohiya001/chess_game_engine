#[derive(Debug)]
pub enum GameError {
    EmptyCell,
    OccupiedCell,
    SideNotChanged,
    SideAlreadyChanged,
    OpponentPiece,
    InvalidMove,
    GameOver,
}
