use crate::cell::Cell;
use crate::player::Player;

#[derive(Clone)]
pub struct Board {
  cells: Vec<Vec<Cell>>
}

impl Board {
  // Create a new empty board.
  pub fn new() -> Board {
    let board_cells = vec![
      vec![Cell::Empty, Cell::Empty, Cell::Empty],
      vec![Cell::Empty, Cell::Empty, Cell::Empty],
      vec![Cell::Empty, Cell::Empty, Cell::Empty]
    ];
    Board { cells: board_cells }
  }

  // Return all available moves
  // a move is a pair of type `(usize, usize)`, it is made out of two numbers
  // the index of the row and the index of the column of the target cell of the
  // move.
  pub fn moves(&self) -> Vec<(usize, usize)> {
    let mut moves = vec![];
    for i in 0..3 {
      for j in 0..3 {
        if let Cell::Empty = self.cells[i][j] {
          moves.push((i, j));
        }
      }
    }
    return moves;
  }

  // Checks if the game is over, either because a player won or because it's
  // a draw.
  pub fn game_over(&self) -> bool {
    // || means OR
    return self.score() != 0 || self.moves().len() == 0;
  }

  // Finds the score of the game: returns 1 if x won, returns -1 if o won,
  // and returns 0 if it is a draw.
  pub fn score(&self) -> i32 {
    for i in 0..3 {
      // ith row
      let x = &self.cells[i][0];
      let y = &self.cells[i][1];
      let z = &self.cells[i][2];
      if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
        return 1;
      } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
        return -1;
      }
    }
    for i in 0..3 {
      // ith column
      let x = &self.cells[0][i];
      let y = &self.cells[1][i];
      let z = &self.cells[2][i];
      if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
        return 1;
      } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
        return -1;
      }
    }
    
    let x = &self.cells[0][0];
    let y = &self.cells[1][1];
    let z = &self.cells[2][2];    
    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
      return 1;
    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
      return -1;
    }

    let x = &self.cells[2][0];
    let y = &self.cells[1][1];
    let z = &self.cells[0][2];    
    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
      return 1;
    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
      return -1;
    }
    
    return 0;
  }
  
  // Applies the given move by putting either X or O in that cell.
  // Whether X or O depends on the player parameter, that specifies which player
  // is making this move.
  // This function does not modify the board itself, instead it creates a copy of
  // it and then applies the move to it.
  pub fn apply_move(&self, m: (usize, usize), player: Player) -> Board {
    let mut board = self.clone();
    match player {
      Player::X => board.cells[m.0][m.1] = Cell::X,
      Player::O => board.cells[m.0][m.1] = Cell::O,
    };
    return board;
  }
  
  // Prints the board to the screen in a nice format
  pub fn print(&self) {
    println!("-----------");
    for row in &self.cells {
      println!("  {} | {} | {}", row[0].to_string(), row[1].to_string(), row[2].to_string());
    }
    println!("-----------");
  }
}
