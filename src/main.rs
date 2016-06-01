#[macro_use]
extern crate itertools;

extern crate gomoku;

use gomoku::board;

fn main() {
  let mut board = board::Board::new();
  board.state[0][0] = board::Square::Black;
  board.state[1][1] = board::Square::Black;
  board.state[3][4] = board::Square::Black;
  board.state[3][5] = board::Square::Black;
  println!("{}{}", board, board.check_free_threes(3, 3, board::Square::Black));
}
