#[macro_use]
extern crate gomoku;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use gomoku::board::{Board, Square};
use gomoku::graphics::{Settings, App};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = Settings:new();
    let mut window: PistonWindow =
        WindowSettings::new("Gomoku", [settings.win_size.x as u32, settings.win_size.y as u32])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let app = App::new();

    let board = Board::from(concat!(
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "________B__________\n",
            "_________B_________\n",
            "___________________\n",
            "___________________\n",
            "____________B______\n",
            "_____________W_____\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________\n",
            "___________________"));
    let ref mut gl = GlGraphics::new(opengl);
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.on_render(&args, gl, cache);
        }
    }
}
