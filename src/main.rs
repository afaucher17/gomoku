#[macro_use]
extern crate gomoku;

use gomoku::board;
use gomoku::game;

fn main() {
    let board = board::Board::from(concat!(
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
    "___________________\n",
    "___________________\n"));
//    println!("{}\n{}", board, board.check_patterns(&board::Square::Black));
//    println!("{}\n{}", board, board.check_patterns(&board::Square::White));
//    for play in board.get_plays() {
//        println!("({}, {})", play.0, play.1);
//    }
//    println!("[{}]", board.get_plays().len())
    game::game_loop(board);
}
