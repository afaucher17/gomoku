#[macro_use]
extern crate gomoku;
extern crate piston_window;
extern crate find_folder;
extern crate opengl_graphics;

use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use gomoku::board::{Board, Square};
use gomoku::graphics::{Settings, App};
use std::sync::mpsc;

fn main() {
    let settings = Settings::new();
    let opengl = OpenGL::V3_2; 
    let mut window: PistonWindow =
        WindowSettings::new("Gomoku", [settings.win_size.x as u32, settings.win_size.y as u32])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    let app = App::new(settings, &mut window);

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

    let (tx, rx) = mpsc::channel();

    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => app.on_render(&e, &mut window, &board),
            _ => ()
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                // I get the board position from app, I get none if the user click is not in the
                // board
            }
        }
    }
}
