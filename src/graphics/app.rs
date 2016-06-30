use graphics::piston_window::*;
use graphics::piston_window::ellipse::circle;
use graphics::graphics::math::Matrix2d;
use graphics::find_folder;
//use graphics::Settings;

use std::path::PathBuf;

use board::{Board, Move, BoardState, Square};
use game::{Game};

pub struct App {
    font: PathBuf,
    /*settings: Settings,
      black_text: Option<usize>,//Texture<Resources>>,
      white_text: Option<usize>,//Texture<Resources>>,*/
}

impl App {
    pub fn new(/*settings: Settings, window: &mut PistonWindow*/) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let font = assets.join("FiraSans-Regular.ttf");
        /*let black_text = assets.join("black.png");
          let black_text = Texture::from_path(
          &mut window.factory,
          &black_text,
          Flip::None,
          &TextureSettings::new())
          .unwrap();
          let white_text = assets.join("white.png");
          let white_text = Texture::from_path(
          &mut window.factory,
          &white_text,
          Flip::None,
          &TextureSettings::new())
          .unwrap();*/

        App {
            font: font,
            /*settings: settings,
              black_text: None,
              white_text: None,*/
        }
    }

    pub fn on_render(&self, e: &Event, win: &mut PistonWindow, game: &Game)
    {
        let factory = win.factory.clone();
        let mut glyphs = Glyphs::new(&self.font, factory).unwrap();
        win.draw_2d(e, |c, g| {
            clear(color::WHITE, g);
            {
                Rectangle::new([1.0, 0.91, 0.5, 1.0]).
                    draw([0.0, 0.0, 840.0, 840.0],
                         &c.draw_state,
                         c.transform,
                         g);
                let grid = grid::Grid {
                    cols: 19,
                    rows: 19,
                    units: 40.0,
                };
                let rec_trans = c.transform.trans(40.0, 40.0);
                grid.draw(&Line::new([0.0, 0.0, 0.0, 1.0], 1.0),
                &c.draw_state,
                rec_trans,
                g);

                self.draw_board(c.transform, &game.board, g);
            }
            {
                let state_trans = c.transform.trans(880.0, 40.0);
                let state = match game.board.game_state {
                    BoardState::Victory(Square::Black) => "Black Victory!",
                    BoardState::Victory(Square::White) => "White Victory!",
                    BoardState::Victory(Square::Empty) => "Empty Victory!",
                    BoardState::FiveAligned(Square::Black) => "Black has five aligned",
                    BoardState::FiveAligned(Square::White) => "White has five aligned",
                    BoardState::FiveAligned(Square::Empty) => "Empty has five aligned",
                    BoardState::Draw => "Draw",
                    BoardState::InProgress => "Game in progress",
                };
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw(state, &mut glyphs, &c.draw_state, state_trans, g);
            }
            {
                let b_cap_trans = c.transform.trans(880.0, 80.0);
                let b_capture = format!("{}{}", game.board.b_capture.to_string(), " stones taken by Black");
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw(b_capture.as_str(), &mut glyphs, &c.draw_state, b_cap_trans, g);
            }
            {
                let w_cap_trans = c.transform.trans(880.0, 120.0);
                let w_capture = format!("{}{}", game.board.w_capture.to_string(), " stones taken by White");
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw(w_capture.as_str(), &mut glyphs, &c.draw_state, w_cap_trans, g);
            }
            {
                let last_trans = c.transform.trans(880.0, 160.0);
                let last_move = match game.last_move {
                    Some(Move::Illegal) => "Illegal move".to_string(),
                    Some(Move::DoubleThrees) => "Double Three move".to_string(),
                    Some(Move::Legal(_, (x, y))) => format!("{} ({}, {})", "Last move:", x.to_string(), y.to_string()),
                    Some(Move::OutOfBounds) => "OutOfBounds should never happen".to_string(),
                    Some(Move::Other(message)) => message.to_string(),
                    _ => "No moves yet".to_string()
                };
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw(last_move.as_str(), &mut glyphs, &c.draw_state, last_trans, g);
            }
        });
    }

    pub fn on_click(&self, mouse_pos: &[f64; 2]) -> Option<(usize, usize)> {
        let mut x = mouse_pos[0] - 40.0;
        x = x / 40.0;
        let mut y = mouse_pos[1] - 40.0;
        y = y / 40.0;
        if (x.fract().abs() < 0.3 || (x.fract() > 0.7 && x > 0.0)) && (y.fract().abs() < 0.3 || (y.fract() > 0.7 && x > 0.0)) {
            if x < 0.0 { x = 0.0 }
            if y < 0.0 { y = 0.0 }
            Some((x.round() as usize, y.round() as usize))
        }
        else { None }
    }

    fn draw_board<G: Graphics>(&self, transform: Matrix2d, board: &Board, g: &mut G)
    {
        for i in 0..19
        {
            for j in 0..19
            {
                let scale = transform.trans(40.0 + i as f64 * 40.0, 40.0 + j as f64 * 40.0).scale(0.1, 0.1);
                match board.state[i][j] {
                    Square::White => Ellipse::new([1.0, 1.0, 1.0, 1.0]).draw(circle(0.0, 0.0, 100.0), &DrawState::default(), scale, g),
                    Square::Black => Ellipse::new([0.0, 0.0, 0.0, 1.0]).draw(circle(0.0, 0.0, 100.0), &DrawState::default(), scale, g),
                    Square::Empty => (),
                }
            }
        }
    }
}
