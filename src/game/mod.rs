extern crate time;

use board::{Board, BoardState, Move, Square};
use minimax::TTEntry;
use minimax::minimax;

use std::io;
use std::i32;
use gomoku::graphics::Settings;
use self::time::PreciseTime;
use std::collections::HashMap;
use std::sync::mpsc;

pub struct Game {
    pub board: Board,
    pub board_state: BoardState,
    players: Vec<Player>,
    pub current_player: Player,
    pub last_move: Option<Move>,
    receiver: Option<Receiver<Option<(usize, usize)>>>
}

struct Player {
    color: Square,
    is_ai: bool,
}

impl Game {
    pub fn new(ai_player: bool) -> Self {
        let mut rng = rand::thread_rng();
        let toss: char = rng.gen_range(0, 2);
        let player_1 = Player { color: if toss = 0 {
            Square::Black
        } else {
            Square::White
        },
        is_ai: ai_player
        };
        let player_2 = Player { color: if toss = 0 {
            Square::White
        } else {
            Square::Black
        },
        is_ai: false,
        last_move: None,
        channel: None,
        };

        Game {
            board: Board::new(),
            players: vec![player_1, player_2],
            current_player: if toss = 0 {
                player_1
            } else {
                player_2
            },
        }
    }

    fn get_input_ai(board: &Board, player: &Square, now: PreciseTime, ttmap: &mut HashMap<u64, TTEntry>) -> Option<(usize, usize)> {

        let mut prev_value: Option<(usize, usize)> = None;
        for depth in 1..13 {
            let value = minimax(board, depth, i32::MIN, i32::MAX, true, None, player, now, ttmap).pos;
            if value == None { println!("Maximum depth in imparted time: {}", depth); break; }
            else { prev_value = value; }
        }
        prev_value
    }

    pub fn update(&mut self)
    {
        if let Some(pos) = self.channel.try_recv() {
            let player_move = board.play_at(pos.unwrap(), self.current_player);
            self.apply_move(player_move);
        }
    }

    pub fn apply_move(&mut self, player_move: Move)
    {
        match player_move {
            Move::Legal(board, _) => {
                self.board = board;
                self.board_state = board.game_state;
                self.last_move = player_move;
                self.current_player = if self.current_player == self.players[0]
                {
                    self.players[1]
                }
                else {
                    self.players[2]
                };
                if self.current_player.is_ai {
                    let (tx, rx) = channel();
                    self.receiver = rx;
                    thread::spawn(move || {
                        let pos = get_input_ai();
                        tx.send(pos).unwrap();
                    });
                }
            },
            _ => {
                self.last_move = player_move;
            }
        }
    }

    pub fn play(&mut self, pos: Option<(usize, usize)>)
    {
        if !self.current_player.is_ai && pos == None {
            let player_move = board.play_at(pos.unwrap(), &player);
            self.apply_move(player_move);
        }
    }

    pub fn game_loop(start: Board, tx: mspc)
    {
        let mut player = Square::Black;
        let mut board = start;
        Board::init_zobrist_array();
        let mut ttmap: HashMap<u64, TTEntry> = HashMap::new();
        println!("{}", board);
        loop {
            let now = PreciseTime::now();
            let input = if player == Square::Black {
                get_input_human()
                    //get_input_ia(&board, &Square::Black, now)
            }
            else {
                get_input_ai(&board, &Square::White, now, &mut ttmap)
            };
            println!("Time since last move: {}", now.to(PreciseTime::now()));
            board = match board.play_at(input, &player) {
                Move::Legal(a_board, (x, y)) => { println!("Last play at: X {}, Y {}", x + 1, y + 1); player = player.opposite(); a_board },
                Move::Illegal => { println!("illegal move, please try again"); board.clone() },
                Move::DoubleThrees => { println!("illegal move (double threes), please try again"); board.clone() },
                Move::OutOfBounds => { println!("your move position must be between 1 and 19"); board.clone() },
                Move::Other(message) => { println!("{}", message); board.clone() },
            };
            println!("{}", board);
            match board.game_state {
                BoardState::InProgress | BoardState::FiveAligned(_) => (),
                BoardState::Draw => {
                    println!("Draw");
                    break;
                }
                BoardState::Victory(ref color) => {
                    println!("Player {} wins", color);
                    break;
                }
            }
        }
    }
}
