use board::{Board, Square, Move};

use std::cmp;
use std::cmp::Ordering;
use std::i32;

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

static mut KILLER_MOVES: [[Option<(usize, usize)>; 2]; 4] = [
           [None, None], [None, None], [None, None], [None, None],
        ];

fn get_plays(board: &Board, color: &Square, depth: usize) -> Vec<(usize, usize)>
{
    let mut v: Vec<(usize, usize)> = Vec::new();
    unsafe {
        match (KILLER_MOVES[depth][0], KILLER_MOVES[depth][1])
        {
            (None, None) => (),
            (Some(pos), None) | (None, Some(pos))=> v.push(pos),
            (Some(pos), Some(pos2)) => v.extend(vec![pos, pos2]),
        }
    }
    v.extend(board.get_plays(color));
    v
}

fn add_killer_move(pos: Option<(usize, usize)>, depth: usize)
{
    unsafe {
        match (KILLER_MOVES[depth][0], KILLER_MOVES[depth][1])
        {
            (None, None) => KILLER_MOVES[depth][0] = pos,
            (Some(_), None) => KILLER_MOVES[depth][1] = pos,
            (old, Some(_)) => {
                KILLER_MOVES[depth][1] = old;
                KILLER_MOVES[depth][0] = pos;
            }
        }
    }
}

pub fn minimax(board: &Board,
               depth: usize,
               mut alpha: i32,
               mut beta: i32,
               maximizing_player: bool,
               prev_play: Option<(usize, usize)>,
               player: &Square)
    -> Decision
{
    let current_color = match maximizing_player { true => player.clone(), false => player.opposite() };
    if depth == 0 || board.is_terminal() {
        return Decision {
            score: board.evaluation(&player),
            pos: prev_play
        };
    }
    let plays: Vec<(usize, usize)> = get_plays(board, &current_color, depth - 1);
    if maximizing_player {
        let mut v = Decision { score: i32::MIN, pos: None };
        //println!(" (DEPTH = {}, POS = {:?}, (MAXIMAZING):", depth, prev_play);
        for pos in plays {
            if let Move::Legal(child, _) = board.play_at(Some(pos), &current_color)
            {
                {
                    let decision = minimax(&child, depth - 1, alpha, beta, false, Some(pos), player);
                    //print!(" {},", decision.score);
                    v = cmp::max(v, decision);
                    alpha = cmp::max(alpha, v.score);
                }
                if alpha >= beta {
                    add_killer_move(v.pos, depth - 1);
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
            if let Move::Legal(child, _) = board.play_at(Some(pos), &current_color) {
                {
                    let decision = minimax(&child, depth - 1, alpha, beta, true, Some(pos), player);
                    //print!("{},", decision.score);
                    v = cmp::min(v, decision);
                    beta = cmp::min(beta, v.score);
                }
                if beta <= alpha {
                    add_killer_move(v.pos, depth);
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
