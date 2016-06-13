#[macro_use]
extern crate gomoku;

use gomoku::board;

fn main() {
    let board = board::Board::from(concat!(
    "_B_B_______________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n"));
//    println!("{}\n{}", board, board.check_patterns(&board::Square::Black));
//    println!("{}\n{}", board, board.check_patterns(&board::Square::White));
    for play in board.get_plays() {
        println!("({}, {})", play.0, play.1);
    }
    println!("[{}]", board.get_plays().len())
}
