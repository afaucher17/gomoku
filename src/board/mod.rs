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
