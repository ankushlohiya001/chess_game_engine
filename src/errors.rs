#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    EmptyCell,
    GameOver,
    SideNotChanged,
}
