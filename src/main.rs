#[macro_use]
extern crate gomoku;
use gomoku::board::{Board};
use gomoku::game;

fn main() {
/*    for play in board.get_plays(&Square::Black) {
        println!("({}, {})", play.0 + 1, play.1 + 1);
    }
    println!("-----------------------------------------------------------");
    for play in board.get_plays(&Square::White) {
        println!("<{}, {}>", play.0 + 1, play.1 + 1);
    }*/
    game::game_loop(Board::new());    
}
