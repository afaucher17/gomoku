#[macro_use]
extern crate itertools;

extern crate gomoku;

use gomoku::board;

fn main() {
  let mut board = board::Board::new();
  board.state[15][13] = board::Square::Black;
  board.state[14][14] = board::Square::Black;
  board.state[13][15] = board::Square::White;
  board.state[16][11] = board::Square::Black;
  board.state[16][10] = board::Square::Black;
  board.state[16][9] = board::Square::White;
  board.state[16][13] = board::Square::Black;
  board.state[16][14] = board::Square::Black;
  board.state[16][15] =  board::Square::White;
  board.state[16][16] = board::Square::White;
  board.state[15][12] = board::Square::Black;
  board.state[14][12] = board::Square::Black;
  board.state[13][12] = board::Square::White;
  println!("{} {}", board, board.play_at(16, 12, &board::Square::White).unwrap());
}
