use graphics::piston_window::*;
use graphics::opengl_graphics::GlGraphics;
use graphics::opengl_graphics::glyph_cache::GlyphCache;
use graphics::graphics::math::Matrix2d;
use graphics::find_folder;

use std::collections::HashMap;
use graphics::gfx_core;

use graphics::Settings;
use board::{Board, Square};

pub struct App<R: gfx_core::Resources> {
    settings: Settings,
    black_text: Texture<R>,
    white_text: Texture<R>,
}

impl<R: gfx_core::Resources> App<R> {
    pub fn new(settings: Settings, window: &mut PistonWindow) -> Self {
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
        gl.draw(args.viewport(), |c, g| {
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
        })
    }

    fn draw_board<G: Graphics>(self, transform: Matrix2d, board: &Board, g: &mut G)
    {
        for i in 0..19
        {
            for j in 0..19
            {
                let scale = transform.trans(27.5 + i as f64 * 40.0, 27.5 + j as f64 * 40.0).scale(0.1, 0.1);
                match board.state[i][j] {
                    Square::White => Image::new().draw(self.textures.get("white").unwrap(), &DrawState::default(), scale, g),
                    Square::Black => Image::new().draw(self.textures.get("black").unwrap(), &DrawState::default(), scale, g),
                    Square::Empty => (),
                }
            }
        }
    }
}
