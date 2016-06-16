#[macro_use]
extern crate gomoku;

use gomoku::board::{Board, Square};
use gomoku::game;
use gomoku::minimax::minimax;
use std::i32;

fn main() {
/*    for play in board.get_plays(&Square::Black) {
        println!("({}, {})", play.0 + 1, play.1 + 1);
    }
    println!("-----------------------------------------------------------");
    for play in board.get_plays(&Square::White) {
        println!("<{}, {}>", play.0 + 1, play.1 + 1);
    }*/
    game::game_loop(Board::new());    
}
