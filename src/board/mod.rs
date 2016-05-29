use std::fmt;

#[derive(Clone)]
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
}
