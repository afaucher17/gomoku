extern crate itertools;

use std::fmt;
use board::itertools::Itertools;


#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Square
{
    Black,
    White,
    Empty,
}

#[derive(Clone)]
pub struct Board
{
    pub state: Vec<Vec<Square>>,
    pub b_capture: usize,
    pub w_capture: usize
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.state.iter()
               .map(|line| line.iter()
                    .map(|square| match *square {
                        Square::Empty => "_",
                        Square::Black => "B",
                        Square::White => "W"
                    }).collect::<String>() + "\n"
                   ).collect::<String>())
    }
}

impl Square {
    pub fn opposite(&self) -> Square
    {
        match *self {
            Square::Black => Square::White,
            Square::White => Square::Black,
            Square::Empty => Square::Empty
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board { state : vec![vec![Square::Empty; 19]; 19], b_capture : 0, w_capture : 0 }
    }

    pub fn play_at(&self, x: usize, y: usize, color: Square) -> Board {
        let mut clone = self.clone();
        clone.state[y][x] = color;
        clone
    }

    fn check_capture(&self, color: &Square, pos: (usize, usize)) -> Board {
        let (x, y) = pos;
        let mut board = self.clone();
        {
            let mut capture = |v: &[(Square, (usize, usize))]| match v {
                [(ref a, _), (ref b, (xb, yb)), (ref c, (xc, yc)), (ref d, _)] if a == d && b == c && *b == color.opposite() && a == color => {
                    board.state[xb][yb] = color.clone();
                    board.state[xc][yc] = color.clone();
                    board.b_capture += if *color == Square::Black { 2 } else { 0 };
                    board.w_capture = if *color == Square::White { 2 } else { 0 };
                }
                _ => ()
            };

            // East
            if x + 4 < 19 {
                capture(&(x..x + 5).map(|i| (self.state[i][y].clone(), (i, y))).collect::<Vec<_>>());
            }
            // West
            if x >= 4 {
                capture(&(0..x + 1).map(|i| (self.state[x - i][y].clone(), (x - i, y))).collect::<Vec<_>>());
            }
            // South
            if y + 4 < 19 {
                capture(&(y..y + 5).map(|i| (self.state[y][i].clone(), (y, i))).collect::<Vec<_>>());
            }
            // North
            if y >= 4 {
                capture(&(0..y + 1).map(|i| (self.state[y][x - i].clone(), (y, x - i))).collect::<Vec<_>>());
            }
            // North-East
            if x + 4 < 19 && y + 4 < 19 {
                capture(&(0..5).map(|i| (self.state[x + i][y + i].clone(), (x + i, y + i))).collect::<Vec<_>>());
            }
            // North-West
            if x + 4 < 19 && y >= 4 {
                capture(&(0..5).map(|i| (self.state[x + i][y - i].clone(), (x + i, y - i))).collect::<Vec<_>>());
            }
            // South-East
            if x >= 4 && y + 4 < 19 {
                capture(&(0..5).map(|i| (self.state[x - i][y + i].clone(), (x - i, y + i))).collect::<Vec<_>>());
            }
            // South-West
            if x >= 4 && y >= 4 {
                capture(&(0..5).map(|i| (self.state[x - i][y + i].clone(), (x - i, y + i))).collect::<Vec<_>>());
            }
        }
        board.clone()
    }

    pub fn check_victory(&self, color: Square) -> bool {
        self.state.iter().any(|v| v.iter().group_by(|elt| **elt == color)
                              .any(|(key, value)| key && value.iter().count() >= 5))
            || (0..19).any(|i| (0..19)
                           .map(|j| self.state[i][j].clone())
                           .collect::<Vec<Square>>().iter()
                           .group_by(|elt| **elt == color)
                           .any(|(key, value)| key && value.iter().count() >= 5))
    }
}
