use board::{Board, Square};
use std::cmp;

struct Decision
{
    chosen_score: usize,
    chose_move: (usize, usize)
}

pub fn minimax(board: board::Board, depth: usize, max_depth: usize, alpha: bool, beta: bool, maximizingPlayer: bool) -> usize 
{
    if depth == 0 /* or node is a terminal node */ {
        return board.evaluation(Square::White);
    }
    if maximizingPlayer {
        let v = usize.min_value();
        /* for child of board {
         * let v = cmp::max(v, minimax(child, depth - 1, alpha, beta, false));
         * let alpha = cmp::max(alpha, v);
         * if beta <= alpha {
         *      break; // beta cut-off
         * }
         * }
         * return v; */
    }
    else {
        let v = usize.max_value();
        /* for child of board {
         *  let v = cmp::min(v, minimax(child, depth - 1, alpha, beta, true));
         *  let beta = cmp::min(beta, v);
         *  if beta <= alpha {
         *      break; // alpha cut-off
         *  }
         *  return v; */
    }
}
