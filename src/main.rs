#[macro_use]
extern crate gomoku;

use gomoku::board::{Board};
use gomoku::game;

fn main() {
   game::game_loop(Board::new()) 
}
