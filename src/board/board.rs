use std::fmt;

use board::square::Square;

#[derive(Clone)]
#[derive(PartialEq)]
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
        Board { state: s.split('\n').collect::<Vec<&'a str>>()
            .iter()
            .map(|s| s.chars().map(|c| match c {
                                'B' => Square::Black,
                                'W' => Square::White,
                                _ => Square::Empty
                            }).collect::<Vec<Square>>())
            .collect::<Vec<Vec<Square>>>(),
            b_capture: 0, w_capture: 0 }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            state : vec![vec![Square::Empty; 19]; 19],
            b_capture : 0,
            w_capture : 0,
        }
    }

    pub fn play_at(&self, x: usize, y: usize, color: &Square) -> Option<Board> {
        let mut clone = self.clone();
        if !(0..19).contains(x) || !(0..19).contains(y)
            || clone.state[x][y] != Square::Empty {
            None
        }
        else {
            clone.state[x][y] = color.clone();
            if !clone.check_free_threes(x as i32, y as i32, color) {
                Some(clone.check_capture(color, (x, y)))
            }
            else { None }
        }
    }

    fn get_surroundings(&self, x: i32, y: i32) -> Vec<(usize, usize)> {
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

    pub fn get_plays(&self) -> Vec<(usize, usize)> {
        let mut plays: Vec<(usize, usize)> = (0..19)
            .fold(vec![], |mut acc, i| {
                acc.extend((0..19)
                           .filter(|j: &usize| self.state[i][*j] != Square::Empty)
                           .fold(vec![], |mut acc2, j| { acc2.extend(self.get_surroundings(i as i32, j as i32).iter().cloned()); acc2 })
                           .iter().cloned()); acc });
        plays.sort_by(|a, b| a.cmp(&b));
        plays.dedup();
        plays
    }
}