#[macro_use]
extern crate gomoku;

use gomoku::board;

fn main() {
  let mut board = board::Board::new();
  let mut playables: Vec<(usize, usize)> = Vec::new();
  board = board.play_at(4, 4, &board::Square::Black).unwrap();
  playables = board.update_playables(4, 4, playables);
  println!("{}", board);
  for tuple in playables.clone() {
    print!("({}, {})", tuple.0, tuple.1)
  }
  println!("");
  board = board.play_at(5, 5, &board::Square::Black).unwrap();
  playables = board.update_playables(5, 5, playables);
  println!("{}", board);
  for tuple in playables.clone() {
    print!("({}, {})", tuple.0, tuple.1)
  }
  println!("");
}
