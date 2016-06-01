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

    pub fn play_at(&self, x: usize, y: usize, color: &Square) -> Option<Board> {
        let mut clone = self.clone();
        if clone.state[x][y] != Square::Empty {
            None
        }
        else {
            clone.state[x][y] = color.clone();
            Some(clone.check_capture(color, (x, y)))
        }
    }

    fn check_capture(&self, color: &Square, pos: (usize, usize)) -> Board {
        let (x, y) = pos;
        let mut board = self.clone();
        {
            let mut capture = |v: &[(Square, (usize, usize))]| match v {
                [(ref a, _), (ref b, (xb, yb)), (ref c, (xc, yc)), (ref d, _)] if a == d && b == c && *b == color.opposite() && a == color => {
                    board.state[xb][yb] = Square::Empty;
                    board.state[xc][yc] = Square::Empty;
                    board.b_capture += if *color == Square::Black { 2 } else { 0 };
                    board.w_capture += if *color == Square::White { 2 } else { 0 };
                }
                _ => ()
            };

            // East
            if x + 4 < 19 {
                capture(&(0..4).map(|i| (self.state[x + i][y].clone(), (x + i, y))).collect::<Vec<_>>());
            }
            // West
            if x >= 4 {
                capture(&(0..4).map(|i| (self.state[x - i][y].clone(), (x - i, y))).collect::<Vec<_>>());
            }
            // South
            if y + 4 < 19 {
                capture(&(0..4).map(|i| (self.state[x][y + i].clone(), (x, y + i))).collect::<Vec<_>>());
            }
            // North
            if y >= 4 {
                capture(&(0..4).map(|i| (self.state[x][y - i].clone(), (x, y - i))).collect::<Vec<_>>());
            }
            // North-East
            if x + 4 < 19 && y + 4 < 19 {
                capture(&(0..4).map(|i| (self.state[x + i][y + i].clone(), (x + i, y + i))).collect::<Vec<_>>());
            }
            // North-West
            if x + 4 < 19 && y >= 4 {
                capture(&(0..4).map(|i| (self.state[x + i][y - i].clone(), (x + i, y - i))).collect::<Vec<_>>());
            }
            // South-East
            if x >= 4 && y + 4 < 19 {
                capture(&(0..4).map(|i| (self.state[x - i][y + i].clone(), (x - i, y + i))).collect::<Vec<_>>());
            }
            // South-West
            if x >= 4 && y >= 4 {
                capture(&(0..4).map(|i| (self.state[x - i][y - i].clone(), (x - i, y - i))).collect::<Vec<_>>());
            }
        }
        board.clone()
    }

    /*pub fn check_aligned(&self, color: Square) -> bool {
        //  Board line exploration:
        self.state.iter().any(|v| v.iter()
                              .group_by(|elt| **elt == color)
                              //.inspect(|x| println!("{:?}", x))
                              .any(|(key, value)| key && value.iter().count() >= 5))
        //  Board column exploration:
            || (0..19).any(|i| (0..19)
                           .map(|j| self.state[j][i].clone())
                           .collect::<Vec<Square>>()
                           .iter()
                           .group_by(|elt| **elt == color)
                           .any(|(key, value)| key && value.iter().count() >= 5))
        //  Board down-right diagonal exploration:
            || (0..38).any(|i: i32| (0..19)
                           .filter(|j: &i32| (i - 19) + j < 19 && (i - 19) + j >= 0)
                           //.inspect(|j: &i32| println!("[{}][{}]", (i - 19) + j, j))
                           .map(|j: i32| self.state[((i - 19) + j) as usize][j as usize].clone())
                           .collect::<Vec<Square>>()
                           .iter()
                           .group_by(|elt| **elt == color)
                           //.inspect(|x| println!("{:?}", x))
                           .any(|(key, value)| key && value.iter().count() >= 5))
        //  Board up-right diagonal exploration
            || (0..38).any(|i: i32| (0..19).rev()
                           .filter(|j: &i32| i - j < 19 && i - j >= 0)
                           //.inspect(|j: &i32| println!("[{}][{}]", i - j, j))
                           .map(|j: i32| self.state[(i - j) as usize][j as usize].clone())
                           .collect::<Vec<Square>>()
                           .iter()
                           .group_by(|elt| **elt == color)
                           //.inspect(|x| println!("{:?}", x))
                           .any(|(key, value)| key && value.iter().count() >= 5))

    }*/

    fn rec_explo(&self, color: &Square, x: i32, y: i32, add_x: i32, add_y: i32, acc: i32) -> i32 {
        if acc > 4 || x + add_x > 18 || y + add_y > 18 || x + add_x < 0 || y + add_y < 0
            || self.state[(x + add_x) as usize][(y + add_y) as usize] != *color { acc }
        else { self.rec_explo(color, x + add_x, y + add_y, add_x, add_y, acc + 1) }
    }

    pub fn check_aligned(&self, x: i32, y: i32, color: &Square) -> bool {
        (self.rec_explo(color, x, y, 1, 1, 1) + self.rec_explo(color, x, y, -1, -1, 0)) > 4
            || (self.rec_explo(color, x, y, 1, 0, 1) + self.rec_explo(color, x, y, -1, 0, 0)) > 4
            || (self.rec_explo(color, x, y, 0, 1, 1) + self.rec_explo(color, x, y, 0, -1, 0)) > 4
            || (self.rec_explo(color, x, y, 1, -1, 1) + self.rec_explo(color, x, y, -1, 1, 0)) > 4
    }

    pub fn check_free_threes(&self, x: usize, y: usize, color: Square) -> bool {
        false
    }
}
