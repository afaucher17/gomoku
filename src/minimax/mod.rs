use board::{Board, Square};

use std::cmp;
use std::cmp::Ordering;
use std::i32;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, PartialOrd, Debug)]
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
               mut alpha: i32,
               mut beta: i32,
               maximizing_player: bool,
               prev_play: Option<(usize, usize)>,
               player: &Square,
               mut killer_moves: Vec<Vec<(usize, usize)>>)
    -> Decision
{
    let current_color = match maximizing_player { true => player.clone(), false => player.opposite() };
    if depth == 0 || board.check_full_board()
        || (prev_play.is_some() &&
            board.check_aligned(prev_play.unwrap(), &current_color))
        || board.b_capture >= 10
        || board.w_capture >= 10 {
        let score = board.evaluation(&player);
        return Decision {
            score: score,
            pos: prev_play
        };
    }
    let mut plays: Vec<(usize, usize)> = Vec::new();
    if !killer_moves[depth].is_empty() { plays.append(&mut killer_moves[depth].clone()); }
    plays.append(&mut board.get_plays(&current_color));
    if maximizing_player {
        let mut v = Decision { score: i32::MIN, pos: None };
        //println!(" (DEPTH = {}, POS = {:?}, (MAXIMAZING):", depth, prev_play);
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                let score = v.score;
                let decision = minimax(&child.unwrap(), depth - 1, alpha, beta, false, Some(pos), player, killer_moves.clone());
                //print!(" {},", decision.score);
                v = cmp::max(v, decision);
                alpha = cmp::max(alpha, v.score);
                if alpha >= beta {
                    if killer_moves[depth].len() == 2 { killer_moves[depth].remove(0); killer_moves[depth].push(v.pos.unwrap()); }
                    //print!(" beta cutoff (beta {} <= {})", beta, v.score);
                    break ; // beta cut-off
                }
            }
        }
        let decision = Decision { 
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
        //println!(") => ({}, {:?})", v.score, decision.pos.unwrap());
        return decision;
    }
    else {
        let mut v = Decision { score: i32::MAX, pos: None };
        //println!(" (DEPTH = {}, POS = {:?}, (MINIMIZING): ", depth, prev_play);
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                let score = v.score;
                let decision = minimax(&child.unwrap(), depth - 1, alpha, beta, true, Some(pos), player, killer_moves.clone());
                //print!("{},", decision.score);
                v = cmp::min(v, decision);
                beta = cmp::min(beta, v.score);
                if beta <= alpha {
                    if killer_moves[depth].len() == 2 { killer_moves[depth].remove(0); killer_moves[depth].push(v.pos.unwrap()); }
                    //print!(" alpha cutoff ({} <= alpha {})", v.score, alpha);
                    break ; // alpha cut-off
                }
            }
        }
        let decision = Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
        //println!(") => ({}, {:?})", v.score, decision.pos.unwrap());
        return decision;
    }
}
