use crate::errors::GameError;

#[derive(Debug, Clone, Copy)]
pub struct Pos(pub char, pub u8);
impl Pos {
    pub fn rank(&self) -> u8 {
        self.1
    }
    pub fn file(&self) -> char {
        self.0
    }
}

pub trait Moving {
    fn possible_moves(&self) -> Vec<Pos>;

    fn can_move(&self, new_pos: Pos) -> bool;

    fn move_to(&mut self, new_pos: Pos) -> Result<(), GameError>;
}
