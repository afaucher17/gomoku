#[macro_use]
extern crate itertools;

extern crate gomoku;

use gomoku::board;

fn main() {
  let board = board::Board::new();
  print!("{}", board)
}
