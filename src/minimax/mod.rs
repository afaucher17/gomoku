use board::{Board, Square};
use std::cmp;

struct Decision
{
    chosen_score: usize,
    chose_move: (usize, usize)
}

pub fn minimax(board: board::Board, depth: usize, alpha: i32, beta: i32, maximizingPlayer: bool, prev_play: (usize, usize)) -> usize 
{
    if depth == 0 || check_full_board(board) || board.check_aligned(prev_play.0, prev_play.1<F15><F15>{
        return board.evaluation(Square::White);
    }
    if maximizingPlayer {
        let mut v = usize.min_value();
        for child in  {
            v = cmp::max(v, minimax(child, depth - 1, alpha, beta, false));
            let alpha = cmp::max(alpha, v);
            if beta <= alpha {
                break ; // beta cut-off
            }
        }
        return v;
    }
    else {
        let mut v = usize.max_value();
        for child in board {
            v = cmp::min(v, minimax(child, depth - 1, alpha, beta, true));
            let beta = cmp::min(beta, v);
            if beta <= alpha {
                break ; // alpha cut-off
            }
        }
        return v;
    }
}
