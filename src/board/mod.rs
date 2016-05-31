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

    pub fn check_aligned(&self, color: Square) -> bool {
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

    }
}
