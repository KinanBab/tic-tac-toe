use crate::board::Board;
use crate::player::Player;

// Dumb agent, makes the first available move.
pub fn first_move(board: Board, player: Player) -> (i32, usize, usize) {
  let moves = board.moves();
  return (0, moves[0].0, moves[0].1);
}

// The best agent. Tries all possiblities and finds the best winning move.
pub fn min_max(board: Board, player: Player) -> (i32, usize, usize) {
  if board.game_over() {
    // We use 10, 10 because there are no more moves available when the game
    // is over.
    return (board.score(), 10, 10);
  }

  // Get all the available legal moves.
  let moves = board.moves();
  
  // Try the first move, store it as the best move and score for now.
  let m = moves[0];
  let board_after_move = board.apply_move(m, player);  // `board` is unaffected, it is copied and the move is applied to the copy.
  let (mut best_score, _, _) = min_max(board_after_move, player.invert());
  let mut best_move = m;

  // Try remaining moves, see if any of them are better.
  for i in 1..moves.len() {
    let m = moves[i];
    let board_after_move = board.apply_move(m, player);
    let (score, _, _) = min_max(board_after_move, player.invert());
    match player {
      Player::X => {
        // We need the move with the max score for player X
        if score > best_score {
          best_score = score;
          best_move = m;
        }
      },
      Player::O => {
        // We need the move with the min score for player O
        if score < best_score {
          best_score = score;
          best_move = m;
        }
      },
    }
  }

  return (best_score, best_move.0, best_move.1);
}
