mod agents;
mod board;
mod cell;
mod player;

use crate::agents::{min_max, first_move};
use crate::board::Board;
use crate::cell::Cell;
use crate::player::Player;

fn main() {
  let mut board = Board::new();
  println!("Game begins");
  board.print();
  
  
  let mut player = Player::X;
  while !board.game_over() {
    let (_, row, col) = match player {
      Player::X => min_max(board.clone(), player),
      Player::O => first_move(board.clone(), player),
    };
  
    board = board.apply_move((row, col), player);
    player = player.invert();
    board.print();
  }
  
  if board.score() == 1 {
    println!("X wins");
  } else if board.score() == -1 {
    println!("O wins");
  } else {
    println!("Draw");
  }
}
