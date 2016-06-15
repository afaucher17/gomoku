#[macro_use]
extern crate gomoku;

use gomoku::board::{Board, Square};
use gomoku::game;
use gomoku::minimax::minimax;
use std::i32;

fn main() {
    let board = Board::from(concat!(
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "________B__________\n",
    "_________B_________\n",
    "___________________\n",
    "___________________\n",
    "____________B______\n",
    "_____________W_____\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________"));
/*    for play in board.get_plays(&Square::Black) {
        println!("({}, {})", play.0 + 1, play.1 + 1);
    }
    println!("-----------------------------------------------------------");
    for play in board.get_plays(&Square::White) {
        println!("<{}, {}>", play.0 + 1, play.1 + 1);
    }*/
    game::game_loop(Board::new());
}
