#[derive(Clone, Copy)]
pub enum Cell {
  X,
  O,
  Empty
}

impl Cell {
  pub fn to_string(&self) -> String {
    match self {
      Cell::X => String::from("x"),
      Cell::O => String::from("o"),
      Cell::Empty => String::from(" "),
    }
  }
}
