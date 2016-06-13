use board::{Board, Square};
use std::io;

pub fn get_input() -> (usize, usize) {
    let mut parsed: Vec<Result<usize, _>>;
    loop {
        println!("Please state your play: X Y");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        parsed = input.split_whitespace().map(|e| e.parse::<usize>()).collect();
        if parsed.iter().any(|e| e.is_err()) { println!("format must be: [1-18] [1-18]") } else { break }
    }

    (parsed[0].clone().unwrap() - 1, parsed[1].clone().unwrap() - 1)
}

pub fn game_loop(board: Board)
{
    let mut player = Square::White;
    let mut new_board = board;
    loop {
        player = player.opposite();
        let input: (usize, usize) = get_input();
        new_board = match new_board.play_at(input.0, input.1, &player) {
            Some(board) => board,
            None => Board::new(),
        };
        println!("{}", new_board);
    }
}
