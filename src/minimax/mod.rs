use board::{Board, Square};

use std::cmp;
use std::cmp::Ordering;
use std::i32;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Decision
{
    score: i32,
    pub pos: Option<(usize, usize)>
}

impl Ord for Decision
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        let lhs = self.score;
        let rhs = other.score;
        lhs.cmp(&rhs)
    }
}

pub fn minimax(board: &Board,
               depth: usize,
               alpha: i32,
               beta: i32,
               maximizingPlayer: bool,
               prev_play: Option<(usize, usize)>,
               player: &Square)
    -> Decision
{
    let current_color = match maximizingPlayer { true => player.clone(), false => player.opposite() };
    if depth == 0 || board.check_full_board()
        || (prev_play.is_some() &&
            board.check_aligned(prev_play.unwrap(), &current_color))
        || board.b_capture >= 10
        || board.w_capture >= 10 {
        return Decision {
            score: board.evaluation(&player),
            pos: prev_play
        };
    }
    let plays = board.get_plays(&current_color);
    if maximizingPlayer {
        let mut v = Decision { score: i32::MIN, pos: None };
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                v = cmp::max(v, minimax(&child.unwrap(), depth - 1, alpha, beta, false, Some(pos), player));
                let alpha = cmp::max(alpha, v.score);
                if beta <= alpha {
                    break ; // beta cut-off
                }
            }
        }
        return Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
    }
    else {
        let mut v = Decision { score: i32::MAX, pos: None };
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                v = cmp::min(v, minimax(&child.unwrap(), depth - 1, alpha, beta, true, Some(pos), player));
                let beta = cmp::min(beta, v.score);
                if beta <= alpha {
                    break ; // alpha cut-off
                }
            }
        }
        return Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
    }
}
