use board::board::{Board, Right};
use board::square::Square;

impl Board
{
    pub fn check_moveintocapture(&self, color: &Square, pos: (usize, usize)) -> bool {
        let (x, y) = pos;

        let capture = |s|
        {
            s == "xyyx".replace("x", color.opposite().to_str()).replace("y", color.to_str())
        };
        
        // East
        if x + 2 < 19 && x >= 1 {
            if capture((0..4).map(|i| self.state[x + i - 1][y].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // West
        if x + 1 < 19 && x >= 2 {
            if capture((0..4).map(|i| self.state[x + i - 2][y].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // South
        if y + 2 < 19 && y >= 1 {
            if capture((0..4).map(|i| self.state[x][y + i - 1].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // North
        if y + 1 < 19 && y >= 2 {
            if capture((0..4).map(|i| self.state[x][y + i - 2].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // North-East
        if y + 1 < 19 && y >= 2 && x + 2 < 19 && x >= 1 {
            if capture((0..4).map(|i| self.state[x + i - 1][y + i - 2].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // North-West
         if y + 1 < 19 && y >= 2 && x + 1 < 19 && x >= 2 {
            if capture((0..4).map(|i| self.state[x + i - 2][y + i - 2].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // South-East
        if y + 2 < 19 && y >= 1 && x + 2 < 19 && x >= 1 {
            if capture((0..4).map(|i| self.state[x + i - 1][y + i - 1].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        // South-West
        if y + 2 < 19 && y >= 1 && x + 1 < 19 && x >= 2 {
            if capture((0..4).map(|i| self.state[x + i - 2][y + i - 1].to_char())
                .collect::<String>()) {
                return true;
            };
        }
        false
    }

    pub fn check_capture(&self, color: &Square, pos: (usize, usize)) -> Board {
        let (x, y) = pos;
        let mut board = self.clone();
        {
            let mut capture = |right: Right|
                if right.data == "xyyx".replace("x", color.to_str()).
                    replace("y", color.opposite().to_str()) {
                        let (xb, yb) = (right.fun)(1);
                        let (xc, yc) = (right.fun)(2);
                        board.state[xb][yb] = Square::Empty;
                        board.state[xc][yc] = Square::Empty;
                        board.add_move((xb, yb), &color.opposite());
                        board.add_move((xc, yc), &color.opposite());
                        board.b_capture +=
                            if *color == Square::Black { 2 } else { 0 };
                        board.w_capture +=
                            if *color == Square::White { 2 } else { 0 };
                    };

            // East
            if x + 3 < 19 {
                capture(Right { data: (0..4).map(|i| self.state[x + i][y].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x + i, y))})
            }
            // West
            if x >= 3 { 
                capture(Right { data: (0..4).map(|i| self.state[x - i][y].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x - i, y))})
            }
            // South
            if y + 3 < 19 { 
                capture(Right { data: (0..4).map(|i| self.state[x][y + i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x, y + i))})
            }
            // North
            if y >= 3 { 
                capture(Right { data: (0..4).map(|i| self.state[x][y - i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x, y - i))})
            }
            // South-East
            if x + 3 < 19 && y + 3 < 19 {
                capture(Right { data: (0..4).map(|i| self.state[x + i][y + i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x + i, y + i))})
            }
            // North-East
            if x + 3 < 19 && y >= 3 {
                capture(Right { data: (0..4).map(|i| self.state[x + i][y - i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x + i, y - i))})
            }
            // South-West
            if x >= 3 && y + 3 < 19 {
                capture(Right { data: (0..4).map(|i| self.state[x - i][y + i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x - i, y + i))})
            }
            // North-West
            if x >= 3 && y >= 3 {
                capture(Right { data: (0..4).map(|i| self.state[x - i][y - i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x - i, y - i))})
            }
        }
        board.clone()
    }

    fn get_positions(p: Vec<(&'static str, Vec<usize>)>, t: Vec<Right>) -> Vec<(usize, usize)> {
        let mut pos = Vec::new();
        for right in t {
            for &(ref pattern, ref vec) in &p {
                if let Some(offset) = right.data.find(pattern) {
                    for i in vec {
                        pos.push((right.fun)(i + offset));
                    }
                }
            }
        }
        pos
    }

    pub fn check_threats(&self) -> Vec<(usize, usize)> {
        let p = vec![("WWWW-", vec![4]), ("BBBB-", vec![4]),
        ("WWWW-W", vec![3]), ("BBBB-B", vec![3]),
        ("WW-WW", vec![2]), ("BB-BB", vec![2]),
        ("W-WWW", vec![1]), ("B-BBB", vec![1]),
        ("-WWWW", vec![0]), ("-BBBB", vec![0]),
        ("--WWW", vec![1]), ("--BBB", vec![1]),
        ("WWW--", vec![3]), ("BBB--", vec![3]),
        ("-WWW-", vec![0, 4]), ("-BBB-", vec![0, 4]),
        ("W-WW-", vec![1, 4]), ("B-BB-", vec![1, 4]),
        ("-WW-W", vec![3, 0]), ("-BB-B", vec![3, 0])];

        let t = self.explode();
        Board::get_positions(p, t)
    }

    fn five_aligned_capture(&self) -> Vec<(usize, usize)> {
        let p = vec![("BWW-", vec![1, 2]), ("-WWB", vec![1, 2]),
        ("WBB-", vec![1, 2]), ("-BBW", vec![1, 2])];

        let t = self.explode();
        Board::get_positions(p, t)
    }

    pub fn check_capture_pos(&self, color: &Square) -> Vec<(usize, usize)>
    {
        let p = match *color { 
            Square::Black => vec![("BWW-", vec![3]), ("-WWB", vec![0])],
            Square::White => vec![("-BBW", vec![0]), ("WBB-", vec![3])],
            Square::Empty => vec![],
        };

        let t = self.explode();
        Board::get_positions(p, t)
    }

    pub fn check_aligned(&self, pos: (usize, usize), color: &Square) -> bool {
        if self.five_aligned(pos, color) {
            let mut test_board = self.clone();
            let to_remove = self.five_aligned_capture();
            to_remove.iter()
                .fold(0, |acc, &(x, y)| { test_board.state[x][y] = Square::Empty; acc });
            test_board.five_aligned(pos, color)
        }
        else { false }
    }

    fn rec_explo(&self, color: &Square, x: i32, y: i32,
                 add_x: i32, add_y: i32, acc: i32) -> i32 {
        if acc > 4 || x + add_x > 18 || y + add_y > 18
            || x + add_x < 0 || y + add_y < 0
                || self.state[(x + add_x) as usize][(y + add_y) as usize]
                != *color {
                    acc
                }
        else {
            self.rec_explo(color, x + add_x, y + add_y, add_x, add_y, acc + 1)
        }
    }

    pub fn five_aligned(&self, pos: (usize, usize), color: &Square) -> bool {
        let (x, y) = (pos.0 as i32, pos.1 as i32);
        (self.rec_explo(color, x, y, 1, 1, 1)
         + self.rec_explo(color, x, y, -1, -1, 0)) > 4
            || (self.rec_explo(color, x, y, 1, 0, 1)
                + self.rec_explo(color, x, y, -1, 0, 0)) > 4
            || (self.rec_explo(color, x, y, 0, 1, 1)
                + self.rec_explo(color, x, y, 0, -1, 0)) > 4
            || (self.rec_explo(color, x, y, 1, -1, 1)
                + self.rec_explo(color, x, y, -1, 1, 0)) > 4
    }

    pub fn check_patterns(&self, color: &Square, current_color: &Square) -> i32 {
        let patterns = vec![("xxxxx", 100240), ("xxxx-", 1280), ("-xxxx", 1280),
        ("xxx-x", 1280), ("x-xxx", 1280), ("xx-xx", 1280), ("xxx--", 160),
        ("--xxx", 160), ("-xxx-", 160), ("-x-xx", 40), ("xx-x-", 40),
        ("--xx-", 10), ("-xx--", 10), ("yxx-", -80), ("-xxy", -80)];
        let player_patterns = patterns.iter().map(|&(s, score)|
                                                  (s.replace("x", color.to_str())
                                                   .replace("y", color.opposite().to_str()),
                                                   if *color != *current_color {
                                                       (score as f64 * 0.75) as i32
                                                   } else {
                                                       score
                                                   })).collect::<Vec<_>>();
        let opponent_patterns = patterns.iter().map(|&(s, score)|
                                                    (s.replace("x", color.opposite().to_str())
                                                     .replace("y", color.to_str()),
                                                     if color.opposite() != *current_color {
                                                         (-score as f64 * 0.75) as i32
                                                     } else {
                                                         -score
                                                     })).collect::<Vec<_>>();

        let t = self.explode();
        let capture_heuristic = |x| if x >= 10 { 500000 } else { x * x * x * x };
        t.iter().fold(0, |acc, right| 
                      acc + player_patterns.iter().chain(opponent_patterns.iter())
                      .fold(0, |acc, &(ref pattern, score)|
                            if right.data.find(pattern).is_some() {
                                acc + score
                            } else {
                                acc
                            }))
        + if *color != *current_color { 
            (capture_heuristic(self.get_score(color)) as f64 * 0.75) as i32
        } else {
            capture_heuristic(self.get_score(color))
        }
        - if color.opposite() != *current_color {
            (capture_heuristic(self.get_score(&color.opposite())) as f64 * 0.75) as i32
        } else {
            capture_heuristic(self.get_score(&color.opposite()))
        }
    }

    pub fn check_free_threes(&self, x: i32, y: i32, color: &Square) -> bool {
        let sq_to_char = |sq: &Square| match *sq {
            Square::Black => 'B', Square::White => 'W', Square::Empty => ' '
        };
        let p = vec![" x xx ", " xx x ", "  xxx ", " xxx  "].iter()
            .map(|s| s.replace("x", match *color {
                Square::Black => "B", Square::White => "W", _ => " "
            }))
        .collect::<Vec<String>>();
        let mut t = vec![String::new(); 4];
        t[0] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| (x + i) < 19 && (x + i) >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][y as usize]))
            .collect::<String>();
        t[1] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| y + i < 19 && y + i >= 0)
            .map(|i| sq_to_char(&self.state[x as usize][(y + i) as usize]))
            .collect::<String>();
        t[2] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| x + i < 19 && x + i >= 0 && y + i < 19 && y + i >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][(y + i) as usize]))
            .collect::<String>();
        t[3] = (0..9).map(|i| i as i32 - 4)
            .filter(|i| x + i < 19 && x + i >= 0 && y - i < 19 && y - i >= 0)
            .map(|i| sq_to_char(&self.state[(x + i) as usize][(y - i) as usize]))
            .collect::<String>();
        t.iter().filter(|s| s.find(&p[0]).is_some()
                        || s.find(&p[1]).is_some()
                        || s.find(&p[2]).is_some()
                        || s.find(&p[3]).is_some()).count() > 1
    }

    pub fn check_full_board(&self) -> bool {
        !self.state.iter().any(|e| e.iter().any(|i| *i == Square::Empty))
    }
}
