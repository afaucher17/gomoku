extern crate rand;
extern crate time;

use board::{Board, BoardState, Move, Square};
use minimax::TTEntry;
use minimax::minimax;

use std::i32;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};
use std::thread;
use self::rand::{Rng};
use self::time::PreciseTime;

pub struct Game {
    pub board: Board,
    players: Vec<Player>,
    pub current_player: Player,
    pub last_move: Option<Move>,
    receiver: Option<Receiver<AIDecision>>,
    map: HashMap<u64, TTEntry>,
}

struct AIDecision {
    pos: Option<(usize, usize)>,
    map: HashMap<u64, TTEntry>,
    start: PreciseTime,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Player {
    pub color: Square,
    pub is_ai: bool,
}

impl Game {
    pub fn new(ai_player: bool) -> Self {
        let mut rng = rand::thread_rng();
        Board::init_zobrist_array();
        let toss: u8 = rng.gen_range(0, 2);
        let player_1 = Player {
            color: if toss == 0 { Square::Black } else { Square::White },
            is_ai: ai_player
        };
        let player_2 = Player {
            color: if toss == 0 { Square::White } else { Square::Black },
            is_ai: false,
        };
        let mut game = Game {
            board: Board::new(),
            players: vec![player_1.clone(), player_2.clone()],
            current_player: if toss == 0 { player_1 } else { player_2 },
            last_move: None,
            receiver: None,
            map: HashMap::new(),
        };
        if game.current_player.is_ai {
            game.ai_move()
        };
        game
    }

    fn get_input_ai(board: &Board, player: &Square, ttmap: &mut HashMap<u64, TTEntry>, now: PreciseTime) -> Option<(usize, usize)> {
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
        let mut received = false;
        let mut player_move = Move::Illegal;
        if let Some(ref receiver) = self.receiver {
            if let Ok(decision) = receiver.try_recv() {
                received = true;
                player_move = self.board.play_at(decision.pos, &self.current_player.color, decision.start);
                self.map = decision.map.clone();
            }
        }
        if received {
            self.apply_move(player_move);
        }
    }

    fn ai_move(&mut self)
    {
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        let tx = tx.clone();
        let board = self.board.clone();
        let mut map = self.map.clone();
        let color = self.current_player.color.clone();
        thread::spawn(move || {
            let now = PreciseTime::now();
            let pos = Game::get_input_ai(&board, &color, &mut map, now);
            tx.send(AIDecision { pos: pos, map: map, start: now }).unwrap();
        });
    }

    pub fn apply_move(&mut self, player_move: Move)
    {
        match player_move.clone() {
            Move::Legal(board, _, _, _) => {
                self.board = board.clone();
                self.last_move = Some(player_move);
                self.current_player = if self.current_player == self.players[0]
                {
                    self.players[1].clone()
                }
                else
                {
                    self.players[0].clone()
                };
                if self.current_player.is_ai {
                    self.ai_move();
                }
            },
            _ => {
                self.last_move = Some(player_move);
            }
        }
    }

    pub fn play(&mut self, pos: Option<(usize, usize)>)
    {
        if !self.current_player.is_ai && pos != None {
            let player_move = self.board.play_at(pos, &self.current_player.color, PreciseTime::now());
            self.apply_move(player_move);
        }
    }
}
