#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Square
{
    Black,
    White,
    Empty,
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

