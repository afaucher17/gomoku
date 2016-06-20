extern crate time;

use board::{Board, BoardState, Move, Square};
use minimax::TTEntry;
use minimax::minimax;

use std::io;
use std::i32;
use self::time::PreciseTime;
use std::collections::HashMap;

pub fn get_input_human() -> Option<(usize, usize)> {
    let mut parsed: Vec<Result<usize, _>>;
    loop {
        println!("Please state your play: X Y");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        parsed = input.split_whitespace().map(|e| e.parse::<usize>()).collect();
        if parsed.iter().any(|e| e.is_err()) || parsed.len() != 2 || parsed[0].clone().unwrap() == 0 || parsed[1].clone().unwrap() == 0 { println!("format must be: [1-19] [1-19]") } else { break }
    }
    Some((parsed[0].clone().unwrap() - 1, parsed[1].clone().unwrap() - 1))
}

fn get_input_ia(board: &Board, player: &Square, now: PreciseTime, ttmap: &mut HashMap<u64, TTEntry>) -> Option<(usize, usize)> {

    let mut prev_value: Option<(usize, usize)> = None;
    for depth in 1..13 {
        let value = minimax(board, depth, i32::MIN, i32::MAX, true, None, player, now, ttmap).pos;
        if value == None { println!("Maximum depth in imparted time: {}", depth); break; }
        else { prev_value = value; }
    }
    prev_value
}

pub fn game_loop(start: Board)
{
    let mut player = Square::Black;
    let mut board = start;
    let mut ttmap: HashMap<u64, TTEntry> = HashMap::new();
    println!("{}", board);
    loop {
        let now = PreciseTime::now();
        let input = if player == Square::Black {
            get_input_human()
            //get_input_ia(&board, &Square::Black, now)
        }
        else {
            get_input_ia(&board, &Square::White, now, &mut ttmap)
        };
        println!("Time since last move: {}", now.to(PreciseTime::now()));
        board = match board.play_at(input, &player) {
            Move::Legal(a_board, (x, y)) => { println!("Last play at: X {}, Y {}", x + 1, y + 1); player = player.opposite(); a_board },
            Move::Illegal => { println!("illegal move, please try again"); board.clone() },
            Move::DoubleThrees => { println!("illegal move (double threes), please try again"); board.clone() },
            Move::OutOfBounds => { println!("your move position must be between 1 and 19"); board.clone() },
            Move::Other(message) => { println!("{}", message); board.clone() },
        };
        println!("{}", board);
        match board.game_state {
            BoardState::InProgress | BoardState::FiveAligned(_) => (),
            BoardState::Draw => {
                println!("Draw");
                break;
            }
            BoardState::Victory(ref color) => {
                println!("Player {} wins", color);
                break;
            }
        }
    }
}
