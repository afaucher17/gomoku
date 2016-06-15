use board::{Board, Square};
use minimax::minimax;
use std::io;
use std::i32;

pub fn get_input() -> Option<(usize, usize)> {
    let mut parsed: Vec<Result<usize, _>>;
    loop {
        println!("Please state your play: X Y");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        parsed = input.split_whitespace().map(|e| e.parse::<usize>()).collect();
        if parsed.iter().any(|e| e.is_err()) || parsed.len() < 2 || parsed.len() > 2 { println!("format must be: [1-19] [1-19]") } else { break }
    }
    Some((parsed[0].clone().unwrap() - 1, parsed[1].clone().unwrap() - 1))
}

pub fn game_loop(start: Board)
{
    let mut player = Square::Black;
    let mut board = start;
    loop {
        println!("{}", board);
        let input = if player == Square::Black {
            get_input()
        }
        else {
            minimax(&board, 4, i32::MIN, i32::MAX, true,
            None, &Square::White).pos
        };
        board = match board.play_at(input, &player) {
            Some(a_board) => { player = player.opposite(); a_board },
            None => { println!("illegal move, please try again"); board.clone() },
        };
        println!("{}", board);
    }
}
