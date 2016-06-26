extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use gomoku::board::{Board, Square};

pub struct App {
    settings: settings::Settings,
    textures: HashMap<String, Texture>,
}

impl App {
    pub fn new(settings: settings::Settings) -> App {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let black_text = Texture::from_path(
            &mut window.factory,
            assets.join("black.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();
        let white_text = Texture::from_path(
            &mut window.factory,
            assets.join("white.png"),
            Flip::None,
            &TextureSettings::new()
            ).unwrap();

        let mut textures = HashMap::new();
        textures.insert("black", black_text);
        textures.insert("white", white_text);

        App {
            settings: settings,
            textures: textures,
        }
    }

    pub fn on_render(self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache, board: &Board)
    {
        gl.draw(&args.viewport(), |c, g| {
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
            self.draw_board(c.transform, board);
        })
    }

    fn draw_board(self, transform: Matrix2d, board: &Board)
    {
        for i in 0..19
        {
            for j in 0..19
            {
                let scale = transform.trans(27.5 + i as f64 * 40.0, 27.5 + j as f64 * 40.0).scale(0.1, 0.1);
                match board.state[i][j] {
                    Square::White => Image::new().draw(self.textures.get("white").unwrap_or(Texture::new()), &DrawState::default(), scale, g),
                    Square::Black => Image::new().draw(self.textures.get("black").unwrap_or(Texture::new()), &DrawState::default(), scale, g),
                    Square::Empty => (),
                }
            }
        }
    }
}
