#[macro_use]
extern crate gomoku;

use gomoku::board;

fn main() {
    let board = board::Board::from(concat!(
    "___________________\n",
    "_BBB_W______BBWW___\n",
    "___________________\n",
    "____W______________\n",
    "_____B____B________\n",
    "______B____W_______\n",
    "_______B____W______\n",
    "________W____W_____\n",
    "______________B____\n",
    "_________B___W_____\n",
    "________B___W______\n",
    "_______W___W_______\n",
    "______W____________\n",
    "_____W___W_________\n",
    "_________W____B____\n",
    "_________W____B____\n",
    "__WWWW________B____\n",
    "______________W____\n",
    "______________W____\n"));
    println!("{}\n{}", board, board.check_patterns(&board::Square::Black));
    println!("{}\n{}", board, board.check_patterns(&board::Square::White));
}
