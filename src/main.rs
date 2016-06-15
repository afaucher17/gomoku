#[macro_use]
extern crate gomoku;

use gomoku::board::{Board, Square};
use gomoku::game;
use gomoku::minimax::minimax;
use std::i32;

fn main() {
    game::game_loop(Board::new());
}
