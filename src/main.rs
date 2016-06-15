#[macro_use]
extern crate gomoku;

use gomoku::board;
use gomoku::game;

fn main() {
    let board = board::Board::from(concat!(
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "________B__________\n",
    "_________B_________\n",
    "___________________\n",
    "___________________\n",
    "____________B______\n",
    "_____________W_____\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________\n",
    "___________________"));
    for play in board.get_plays(&board::Square::Black) {
        println!("({}, {})", play.0, play.1);
    }
    for play in board.get_plays(&board::Square::White) {
        println!("<{}, {}>", play.0, play.1);
    }
//    game::game_loop(board);
}
