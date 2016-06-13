use board::{Board, Square};
use std::cmp;

struct Decision
{
    chosen_score: usize,
    chose_move: (usize, usize)
}

pub fn minimax(board: board::Board, depth: usize, alpha: i32, beta: i32, maximizingPlayer: bool, prev_play: (usize, usize), evaluated: board::Square) -> usize 
{
    let current_player = Square::Empty; //TODO
    if depth == 0 || check_full_board(board)
        || board.check_aligned(prev_play.0, prev_play.1, &current_player)
        || board.b_capture >= 10
        || board.w_capture >= 10 {
        return board.evaluation(Square::White);
    }
    let plays = board.get_plays();
    if maximizingPlayer {
        let mut v = usize.min_value();
        for play in plays {
            let child = board.play_at(play.0, play.1, &current_player);
            if child.is_some() {
                v = cmp::max(v, minimax(child.unwrap(), depth - 1, alpha, beta, false, play));
                let alpha = cmp::max(alpha, v);
                if beta <= alpha {
                    break ; // beta cut-off
                }
            }
        }
        return v;
    }
    else {
        let mut v = usize.max_value();
        for play in plays {
            let child = board.play_at(play.0, play.1, &current_player);
            if child.is_some() {
                v = cmp::min(v, minimax(child.unwrap(), depth - 1, alpha, beta, true, play));
                let beta = cmp::min(beta, v);
                if beta <= alpha {
                    break ; // alpha cut-off
                }
            }
        }
        return v;
    }
}
