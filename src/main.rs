#[macro_use]
extern crate itertools;

extern crate gomoku;

use gomoku::board;

fn main() {
  let mut board = board::Board::new();
  /*board.state[0][0] = board::Square::Black;
  board.state[0][1] = board::Square::Black;
  board.state[0][2] = board::Square::Black;
  board.state[0][3] = board::Square::Black;
  board.state[0][4] = board::Square::Black;
  board.state[2][5] = board::Square::Black;
  board.state[3][5] = board::Square::Black;
  board.state[4][5] = board::Square::Black;
  board.state[5][5] = board::Square::Black;
  board.state[6][5] = board::Square::Black;*/
  board.state[0][4] = board::Square::Black;
  board.state[1][3] = board::Square::Black;
  board.state[2][2] = board::Square::Black;
  board.state[3][1] = board::Square::Black;
  board.state[4][0] = board::Square::Black;
  println!("{}{}", board, board.check_aligned(board::Square::Black));
  board = board::Board::new();
  board.state[14][18] = board::Square::Black;
  board.state[15][17] = board::Square::Black;
  board.state[16][16] = board::Square::Black;
  board.state[17][15] = board::Square::Black;
  board.state[18][14] = board::Square::Black;
  println!("{}{}", board, board.check_aligned(board::Square::Black));
  board = board::Board::new();
  board.state[0][14] = board::Square::Black;
  board.state[1][15] = board::Square::Black;
  board.state[2][16] = board::Square::Black;
  board.state[3][17] = board::Square::Black;
  board.state[4][18] = board::Square::Black;
  println!("{}{}", board, board.check_aligned(board::Square::Black));
  board = board::Board::new();
  board.state[14][0] = board::Square::Black;
  board.state[15][1] = board::Square::Black;
  board.state[16][2] = board::Square::Black;
  board.state[17][3] = board::Square::Black;
  board.state[18][4] = board::Square::Black;
  println!("{}{}", board, board.check_aligned(board::Square::Black));
}
