extern crate time;

use board::{Board, Square, Move};

use std::cmp;
use std::cmp::Ordering;
use std::i32;
use std::collections::HashMap;
use self::time::PreciseTime;

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Decision
{
    score: i32,
    pub pos: Option<(usize, usize)>
}

#[derive(Clone)]
pub struct TTEntry
{
    score: i32,
    tttype: TTType,
    depth: usize,
}

#[derive(Clone)]
enum TTType
{
    ExactValue,
    Lowerbound,
    Upperbound,
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

static mut KILLER_MOVES: [[Option<(usize, usize)>; 2]; 12] = [
    [None, None], [None, None], [None, None], [None, None],
    [None, None], [None, None], [None, None], [None, None],
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
               player: &Square,
               start: PreciseTime,
               ttmap: &mut HashMap<u64, TTEntry>
              )
    -> Decision
{
    let current_color = match maximizing_player { true => player.clone(), false => player.opposite() };
    // Time-out
    if start.to(PreciseTime::now()).num_milliseconds() >= 500 {
        return Decision {
            score: 0,
            pos: None
        };
    }

    // Transition Table
    {
        let tte = ttmap.get(&board.hash);
        if tte.is_some() && tte.unwrap().depth >= depth
        {
            let tte = tte.unwrap();
            match tte.tttype {
                TTType::ExactValue => return Decision { score: tte.score, pos: prev_play },
                TTType::Lowerbound if tte.score > alpha => alpha = tte.score,
                TTType::Upperbound if tte.score < beta => beta = tte.score,
                _ => ()
            }
            if alpha >= beta {
                return Decision { score: tte.score, pos: prev_play };
            }
        }
    }
    // Terminal Node
    if depth == 0 || board.is_terminal() {
        let value = board.evaluation(&player, &current_color);
        if value <= alpha {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Lowerbound, depth: depth });
        }
        else if value >= beta {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Upperbound, depth: depth });
        }
        else {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::ExactValue, depth: depth });
        }
        return Decision {
            score: value,
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
                    let decision = minimax(&child, depth - 1, alpha, beta, false, Some(pos), player, start, ttmap);
                    if decision.pos == None { return decision; }
                    v = cmp::max(v, decision);
                    alpha = cmp::max(alpha, v.score);
                }
                if alpha >= beta {
                    add_killer_move(v.pos, depth - 1);
                    break ; // beta cut-off
                }
            }
        }
        let value = v.score;
        if value <= alpha {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Lowerbound, depth: depth });
        }
        else if value >= beta {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Upperbound, depth: depth });
        }
        else {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::ExactValue, depth: depth });
        }
        let decision = Decision { 
            score: value,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
        return decision;
    }
    else {
        let mut v = Decision { score: i32::MAX, pos: None };
        for pos in plays {
            if let Move::Legal(child, _) = board.play_at(Some(pos), &current_color) {
                {
                    let decision = minimax(&child, depth - 1, alpha, beta, true, Some(pos), player, start, ttmap);
                    if decision.pos == None { return decision; }
                    v = cmp::min(v, decision);
                    beta = cmp::min(beta, v.score);
                }
                if beta <= alpha {
                    add_killer_move(v.pos, depth);
                    break ; // alpha cut-off
                }
            }
        }
        let value = v.score;
        if value <= alpha {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Lowerbound, depth: depth });
        }
        else if value >= beta {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::Upperbound, depth: depth });
        }
        else {
            ttmap.insert(board.hash, TTEntry { score: value, tttype: TTType::ExactValue, depth: depth });
        }
        let decision = Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
        return decision;
    }
}
