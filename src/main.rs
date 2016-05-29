extern crate gomoku;

pub mod board;

use board::{Board, Square};

fn main() {
  let board = board::Board::new();
  print!("{}", board)
}
