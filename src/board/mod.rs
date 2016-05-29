#[derive(Clone)]
pub enum Square
{
    Black,
    White,
    None,
}

#[derive(Clone)]
pub struct Board
{
    pub state: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Board {
        Board { state : vec![vec![Square::None; 19]; 19] }
    }
}
