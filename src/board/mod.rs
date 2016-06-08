extern crate itertools;

use std::fmt;

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
        if !(0..19).contains(x) || !(0..19).contains(y) || clone.state[x][y] != Square::Empty {
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

    pub fn check_free_threes(&self, x: i32, y: i32, color: &Square) -> bool {
        let sq_to_char = |sq: &Square| match *sq { Square::Black => 'B', Square::White => 'W', Square::Empty => ' ' };
        let p = vec![" x xx ", " xx x ", "  xxx ", " xxx  "].iter()
            .map(|s| s.replace("x", match *color { Square::Black => "B", Square::White => "W", _ => " " })).collect::<Vec<String>>();
        let mut t = vec![String::new(); 4];
        t[0] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| x + i < 19 && x + i >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][y as usize])).collect::<String>();
        t[1] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| y + i < 19 && y + i >= 0)
            .map(|i| sq_to_char(&self.state[x as usize][(y + i) as usize])).collect::<String>();
        t[2] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| x + i < 19 && x + i >= 0 && y + i < 19 && y - *i >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][(y + i) as usize])).collect::<String>();
        t[3] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| x + i < 19 && x + i >= 0 && y - *i < 19 && y - *i >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][(y - i) as usize])).collect::<String>();
        t.iter().filter(|s| s.find(&p[0]).is_some()
                        || s.find(&p[1]).is_some()
                        || s.find(&p[2]).is_some()
                        || s.find(&p[3]).is_some()).count() > 1
    }

    //Playable board
    pub fn update_playables(&self, x: i32, y: i32, plays: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut updated = plays.clone();
        updated.iter().position(|&e| x as usize == e.0 && y as usize == e.1).map(|e| updated.remove(e));
        if self.state[(x + 1) as usize][y as usize] == Square::Empty
            && (0..19).contains(x + 1) {
            updated.push(((x + 1) as usize, y as usize))
        }
        if self.state[(x + 1) as usize][(y + 1) as usize] == Square::Empty
            && (0..19).contains(x + 1) && (0..19).contains(y + 1) {
            updated.push(((x + 1) as usize, (y + 1) as usize))
        }
        if self.state[x as usize][(y + 1) as usize] == Square::Empty
            && (0..19).contains(y + 1) {
            updated.push((x as usize, (y + 1) as usize))
        }
        if self.state[(x - 1) as usize][(y + 1) as usize] == Square::Empty
            && (0..19).contains(x - 1) && (0..19).contains(y + 1) {
            updated.push(((x - 1) as usize, (y + 1) as usize))
        }
        if self.state[(x - 1) as usize][y as usize] == Square::Empty
            && (0..19).contains(x - 1) {
            updated.push(((x - 1) as usize, y as usize))
        }
        if self.state[(x - 1) as usize][(y - 1) as usize] == Square::Empty
            && (0..19).contains(x - 1) && (0..19).contains(y - 1) {
            updated.push(((x - 1) as usize, (y - 1) as usize))
        }
        if self.state[x as usize][(y - 1) as usize] == Square::Empty
            && (0..19).contains(y - 1) {
            updated.push((x as usize, (y - 1) as usize))
        }
        if self.state[(x + 1) as usize][(y - 1) as usize] == Square::Empty
            && (0..19).contains(x + 1) && (0..19).contains(y - 1) {
            updated.push(((x + 1) as usize, (y - 1) as usize))
        }
        updated.sort_by(|a, b| a.cmp(&b));
        updated.dedup();
        updated
    }


}
