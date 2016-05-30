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

impl Board {
    pub fn new() -> Board {
        Board { state : vec![vec![Square::Empty; 19]; 19] }
    }

    pub fn play_at(&self, x: usize, y: usize, color: Square) -> Board {
        let mut clone = self.clone();
        clone.state[y][x] = color;
        clone
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
