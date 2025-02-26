#[derive(Clone, Copy)]
pub enum Player {
  X,
  O
}

impl Player {
  pub fn invert(self) -> Player {
    match self {
      Player::X => Player::O,
      Player::O => Player::X,
    }
  }
}
