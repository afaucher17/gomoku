extern crate itertools;
extern crate rand;

use board::square::Square;

use std::fmt;
use self::itertools::Itertools;
use std::hash::{Hash, Hasher};
use self::rand::Rng;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Board
{
    pub state: Vec<Vec<Square>>,
    pub b_capture: usize,
    pub w_capture: usize,
    pub game_state: BoardState,
    pub hash: u64,
}

#[derive(Clone, PartialEq)]
pub enum BoardState
{
    InProgress,
    Draw,
    Victory(Square),
    FiveAligned(Square),
}

static mut ZOBRIST_ARRAY: [[u64; 361]; 2] = [[0; 361]; 2];

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} Black Captures: {} White Captures: {}", self.state.iter()
               .map(|line| line.iter()
                    .map(|square| match *square {
                        Square::Empty => "_",
                        Square::Black => "B",
                        Square::White => "W"
                    }).collect::<String>() + "\n"
                   ).collect::<String>(), self.b_capture, self.w_capture)
    }
}

impl fmt::Debug for Board
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} Black Captures: {} White Captures: {}", self.state.iter()
               .map(|line| line.iter()
                    .map(|square| match *square {
                        Square::Empty => "_",
                        Square::Black => "B",
                        Square::White => "W"
                    }).collect::<String>() + "\n"
                   ).collect::<String>(), self.b_capture, self.w_capture)
    }
}

impl<'a> From<&'a str> for Board
{
    fn from(s: &'a str) -> Self {
        let mut board = Board { state: s.split('\n').collect::<Vec<&'a str>>()
            .iter()
                .map(|s| s.chars().map(|c| match c {
                    'B' => Square::Black,
                    'W' => Square::White,
                    _ => Square::Empty
                }).collect::<Vec<Square>>())
            .collect::<Vec<Vec<Square>>>(),
            b_capture: 0,
            w_capture: 0,
            hash: 0,
            game_state: BoardState::InProgress,
        };
        board.generate_hash();
        board
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            state : vec![vec![Square::Empty; 19]; 19],
            b_capture : 0,
            w_capture : 0,
            game_state: BoardState::InProgress,
            hash : 0,
        }
    }

    pub fn init_zobrist_array() {
        unsafe {
            let mut rng = rand::thread_rng();
            for i in 0..2 {
                for j in 0..361 {
                    ZOBRIST_ARRAY[i][j] = rng.gen::<u64>();
                }
            }
        }
    }

    pub fn add_move(&mut self, pos: (usize, usize), color: &Square) 
    {
        let (x, y) = pos;
        unsafe {
            self.hash ^= ZOBRIST_ARRAY[match *color {
                Square::Black => 0,
                Square::White => 1,
                _ => 0,
            }][x * 19 + y];
        };
    }

    pub fn generate_hash(&mut self)
    {
        for i in 0..19
        {
            for j in 0..19
            {
                match self.state[i][j] {
                    Square::Black => self.add_move((i, j), &Square::Black),
                    Square::White => self.add_move((i, j), &Square::White),
                    Square::Empty => ()
                }
            }
        }
    }

    pub fn get_score(&self, color: &Square) -> i32
    {
        match *color {
            Square::White => self.w_capture as i32,
            Square::Black => self.b_capture as i32,
            Square::Empty => 0,
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self.game_state {
            BoardState::Victory(_) | BoardState::Draw => true,
            _ => false,
        }
    }

    pub fn play_at(&self, pos: Option<(usize, usize)>, color: &Square) -> Option<Board> {
        match pos {
            Some((x, y)) => {
                let mut clone = self.clone();
                if !(0..19).contains(x) || !(0..19).contains(y)
                    || clone.state[x][y] != Square::Empty {
                        None
                    }
                else {
                    clone.state[x][y] = color.clone();
                    if !clone.check_free_threes(x as i32, y as i32, color) {
                        clone = clone.check_capture(color, (x, y));
                        clone.game_state = clone.get_game_state(pos.unwrap(), color);
                        clone.add_move(pos.unwrap(), color);
                        Some(clone)
                    }
                    else { None }
                }
            },
            None => None,
        }
    }

    fn get_game_state(&self, pos: (usize, usize), color: &Square) -> BoardState
    {
        if self.b_capture >= 10 {
            BoardState::Victory(Square::Black)
        }
        else if *color == Square::Black && self.five_aligned(pos, color) {
            if self.check_aligned(pos, color) {
                BoardState::Victory(Square::Black)
            }
            else {
                BoardState::FiveAligned(Square::Black)
            }
        }
        else if self.w_capture >= 10 {
            BoardState::Victory(Square::White)
        }
        else if *color == Square::White && self.five_aligned(pos, color) {
            if self.check_aligned(pos, color) {
                BoardState::Victory(Square::White)
            } else {
                BoardState::FiveAligned(Square::White)
            }
        }
        else if self.check_full_board() {
            BoardState::Draw
        }
        else {
            BoardState::InProgress
        }
    }

    fn get_square_surroundings(&self, x: i32, y: i32) -> Vec<(usize, usize)> {
        let mut surr: Vec<(usize, usize)> = Vec::new();
        if (0..19).contains(x + 1)
            && self.state[(x + 1) as usize][y as usize] == Square::Empty {
                surr.push(((x + 1) as usize, y as usize))
            }
        if (0..19).contains(x + 1) && (0..19).contains(y + 1)
            && self.state[(x + 1) as usize][(y + 1) as usize] == Square::Empty {
                surr.push(((x + 1) as usize, (y + 1) as usize))
            }
        if (0..19).contains(y + 1)
            && self.state[x as usize][(y + 1) as usize] == Square::Empty {
                surr.push((x as usize, (y + 1) as usize))
            }
        if (0..19).contains(x - 1) && (0..19).contains(y + 1)
            && self.state[(x - 1) as usize][(y + 1) as usize] == Square::Empty {
                surr.push(((x - 1) as usize, (y + 1) as usize))
            }
        if (0..19).contains(x - 1)
            && self.state[(x - 1) as usize][y as usize] == Square::Empty {
                surr.push(((x - 1) as usize, y as usize))
            }
        if (0..19).contains(x - 1) && (0..19).contains(y - 1)
            && self.state[(x - 1) as usize][(y - 1) as usize] == Square::Empty {
                surr.push(((x - 1) as usize, (y - 1) as usize))
            }
        if (0..19).contains(y - 1)
            && self.state[x as usize][(y - 1) as usize] == Square::Empty {
                surr.push((x as usize, (y - 1) as usize))
            }
        if (0..19).contains(x + 1) && (0..19).contains(y - 1)
            && self.state[(x + 1) as usize][(y - 1) as usize] == Square::Empty {
                surr.push(((x + 1) as usize, (y - 1) as usize))
            }
        surr
    }

    fn get_surroundings(&self, color: &Square) -> Vec<(usize, usize)>
    {
        (0..19).fold(vec![], |mut acc, i| {
                acc.extend((0..19)
                           .filter(|j: &usize| self.state[i][*j] != Square::Empty && self.state[i][*j] == *color)
                           .fold(vec![], |mut acc2, j| { acc2.extend(self.get_square_surroundings(i as i32, j as i32).iter().cloned()); acc2 })
                           .iter().cloned()); acc })
    }

    pub fn get_plays(&self, color: &Square) -> Vec<(usize, usize)> {
        let mut plays = self.check_threats();
        let mut check_capture = self.check_capture_pos(color);
        plays.append(&mut check_capture);
        if plays.is_empty() {
            let mut player_surroundings = self.get_surroundings(color);
            plays.append(&mut player_surroundings);
        }
        if plays.is_empty() {
            plays.push((9, 9))
        }
        plays.into_iter().unique().collect()
    }

    pub fn evaluation(&self, player: &Square, current_player: &Square) -> i32 {
        self.check_patterns(player, current_player)
    }
}
