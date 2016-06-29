use graphics::piston_window::*;
use graphics::piston_window::ellipse::circle;
use graphics::opengl_graphics::GlGraphics;
use graphics::opengl_graphics::glyph_cache::GlyphCache;
use graphics::graphics::math::Matrix2d;
use graphics::gfx_device_gl::{Resources};
use graphics::find_folder;

use graphics::Settings;
use board::{Board, Square};

pub struct App {
    settings: Settings,
    black_text: Option<usize>,//Texture<Resources>>,
    white_text: Option<usize>,//Texture<Resources>>,
}

impl App {
    pub fn new(settings: Settings, window: &mut PistonWindow) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let black_text = assets.join("black.png");
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
            .unwrap();

        App {
            settings: settings,
            black_text: None,
            white_text: None,
        }
    }

    pub fn on_render(&self, e: &Event, win: &mut PistonWindow, board: &Board)
    {
        win.draw_2d(e, |c, g| {
            clear(color::WHITE, g);
            let transform = c.transform.trans(40.0, 40.0);
            Rectangle::new([1.0, 0.91, 0.5, 1.0]).
                draw([0.0, 0.0, 840.0, 840.0],
                     &c.draw_state,
                     c.transform,
                     g
                    );
            let grid = grid::Grid {
                cols: 19,
                rows: 19,
                units: 40.0,
            };
            grid.draw(&Line::new([0.0, 0.0, 0.0, 1.0], 1.0),
            &c.draw_state,
            transform,
            g
            );
            self.draw_board(c.transform, board, g);
        });
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
