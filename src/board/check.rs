use board::board::Board;
use board::square::Square;
use std::cmp;

impl Board
{
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

        struct Right {
            data: String,
            fun: Box<Fn(usize) -> (usize, usize)>,
        }

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
            // North-East
            if x + 3 < 19 && y + 3 < 19 {
                capture(Right { data: (0..4).map(|i| self.state[x + i][y + i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x + i, y + i))})
            }
            // North-West
            if x + 3 < 19 && y >= 3 {
                capture(Right { data: (0..4).map(|i| self.state[x + i][y - i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x + i, y - i))})
            }
            // South-East
            if x >= 3 && y + 3 < 19 {
                capture(Right { data: (0..4).map(|i| self.state[x - i][y + i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x - i, y + i))})
            }
            // South-West
            if x >= 3 && y >= 3 {
                capture(Right { data: (0..4).map(|i| self.state[x - i][y - i].to_char())
                        .collect::<String>(),
                        fun: Box::new(move |i| (x - i, y - i))})
            }
        }
        board.clone()
    }

    fn five_aligned_capture(&self) -> Vec<(usize, usize)> {
        let sq_to_char = |sq: &Square| match *sq {
            Square::Black => 'B',
            Square::White => 'W', Square::Empty => '-'
        };

        let p = vec![("BWW-", vec![1, 2]), ("-WWB", vec![1, 2]),
        ("WBB-", vec![1, 2]), ("-BBW", vec![1, 2])];

        struct Right {
            data: String,
            fun: Box<Fn(usize) -> (usize, usize)>,
        }

        let mut t: Vec<Right> = Vec::new();
        {
            let mut vert = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[i][j]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (i, v as usize)) }
                    ).collect::<Vec<Right>>();
            let mut hor = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[j][i]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (v as usize, i)) }
                    ).collect::<Vec<Right>>();
            let mut diagup = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::max(0, 18 - i) + j) as usize]))
                        .collect::<String>(), 
                    fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::max(0, 18 - i as i32) as usize + v))
                }).collect::<Vec<Right>>();
            let mut diagdown = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::min(18, i) - j) as usize]))
                        .collect::<String>(),
                        fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::min(18, i as i32) as usize - v))
                }).collect::<Vec<Right>>();
            t.append(&mut vert);
            t.append(&mut hor);
            t.append(&mut diagup);
            t.append(&mut diagdown);
        }

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

    pub fn check_threats(&self) -> Vec<(usize, usize)> {
        let sq_to_char = |sq: &Square| match *sq {
            Square::Black => 'B',
            Square::White => 'W', Square::Empty => '-'
        };

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

        struct Right {
            data: String,
            fun: Box<Fn(usize) -> (usize, usize)>,
        }

        let mut t: Vec<Right> = Vec::new();
        {
            let mut vert = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[i][j]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (i, v as usize)) }
                    ).collect::<Vec<Right>>();
            let mut hor = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[j][i]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (v as usize, i)) } 
                    ).collect::<Vec<Right>>();
            let mut diagup = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::max(0, 18 - i) + j) as usize]))
                        .collect::<String>(), 
                    fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::max(0, 18 - i as i32) as usize + v))
                }).collect::<Vec<Right>>();
            let mut diagdown = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::min(18, i) - j) as usize]))
                        .collect::<String>(),
                        fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::min(18, i as i32) as usize - v))
                }).collect::<Vec<Right>>();
            t.append(&mut vert);
            t.append(&mut hor);
            t.append(&mut diagup);
            t.append(&mut diagdown);
        }

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

    pub fn check_capture_pos(&self, color: &Square) -> Vec<(usize, usize)>
    {
        let sq_to_char = |sq: &Square| match *sq {
            Square::Black => 'B',
            Square::White => 'W', Square::Empty => '-'
        };

        let p = match *color { 
            Square::Black => vec![("BWW-", vec![3]), ("-WWB", vec![0])],
            Square::White => vec![("-BBW", vec![0]), ("WBB-", vec![3])],
            Square::Empty => vec![],
        };

        struct Right {
            data: String,
            fun: Box<Fn(usize) -> (usize, usize)>,
        }

        let mut t: Vec<Right> = Vec::new();
        {
            let mut vert = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[i][j]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (i, v as usize)) }
                    ).collect::<Vec<Right>>();
            let mut hor = (0..19).map(|i| Right { data: (0..19)
                .map(|j| sq_to_char(&self.state[j][i]))
                    .collect::<String>(),
                    fun: Box::new(move |v| (v as usize, i)) } 
                    ).collect::<Vec<Right>>();
            let mut diagup = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::max(0, 18 - i) + j) as usize]))
                        .collect::<String>(), 
                    fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::max(0, 18 - i as i32) as usize + v))
                }).collect::<Vec<Right>>();
            let mut diagdown = (0..37)
                .map(|i| Right { data: (0..19 - (19 - (i as i32 + 1)).abs())
                    .map(|j| sq_to_char(
                            &self.state[(cmp::max(0, i - 18) + j) as usize][(cmp::min(18, i) - j) as usize]))
                        .collect::<String>(),
                        fun: Box::new(move |v| (cmp::max(0, i as i32 - 18) as usize + v, cmp::min(18, i as i32) as usize - v))
                }).collect::<Vec<Right>>();
            t.append(&mut vert);
            t.append(&mut hor);
            t.append(&mut diagup);
            t.append(&mut diagdown);
        }

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

    pub fn check_patterns(&self, color: &Square, current_color: &Square) -> i32 {
        let sq_to_char = |sq: &Square| match *sq {
            Square::Black => 'B', Square::White => 'W', Square::Empty => '-'
        };

        let patterns = vec![("xxxxx", 10240), ("xxxx-", 1280), ("-xxxx", 1280),
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
        let mut t = Vec::new();
        {
            let mut vert = (0..19).map(|i| (0..19)
                                       .map(|j| sq_to_char(&self.state[i][j]))
                                       .collect::<String>())
                .collect::<Vec<_>>();
            let mut hor = (0..19)
                .map(|i| (0..19)
                     .map(|j| sq_to_char(&self.state[j][i]))
                     .collect::<String>())
                .collect::<Vec<_>>();
            let mut diagup = (0..38)
                .map(|i: i32| (0..19)
                     .filter(|j: &i32| (i - 19) + j < 19 && (i - 19) + j >= 0)
                     .map(|j: i32| sq_to_char(
                             &self.state[((i - 19) + j) as usize][j as usize]))
                     .collect::<String>())
                .collect::<Vec<_>>();
            let mut diagdown = (0..38)
                .map(|i: i32| (0..19)
                     .filter(|j: &i32| (i - j) < 19 && (i - j) >= 0)
                     .map(|j: i32| sq_to_char(
                             &self.state[(i - j) as usize][j as usize]))
                     .collect::<String>())
                .collect::<Vec<_>>();
            t.append(&mut vert);
            t.append(&mut hor);
            t.append(&mut diagup);
            t.append(&mut diagdown);
        }
        let capture_heuristic = |x| if x == 10 { 500000 } else { x * 11 };
        t.iter().fold(0, |acc, s| 
                      acc + player_patterns.iter().chain(opponent_patterns.iter())
                      .fold(0, |acc, &(ref pattern, score)|
                            if s.find(pattern).is_some() {
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
