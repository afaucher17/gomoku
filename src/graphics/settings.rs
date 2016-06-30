pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

pub struct Settings {
    pub win_size: Vec2f,
    pub cell_size: f64,
    pub board_margin: Vec2f,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            win_size: Vec2f { x: 1300.0, y: 840.0 },
            cell_size: 40.0,
            board_margin: Vec2f { x: 40.0, y: 40.0 },
        }
    }
}
