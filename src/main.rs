#[macro_use]
extern crate gomoku;
extern crate piston_window;

use piston_window::*;
use gomoku::board::{Board, Square};
use gomoku::game;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Gomoku", [1000, 820])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut grid = grid::Grid {
            cols: 19,
            rows: 19,
            units: 40.0,
    };

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear(color::WHITE, g);
            let transform = c.transform.trans(30.0, 30.0);
            Rectangle::new([1.0, 0.91, 0.5, 1.0]).
                draw([0.0, 0.0, 820.0, 820.0],
                     &c.draw_state,
                     c.transform,
                     g
                     );
            grid.draw(&Line::new([0.0, 0.0, 0.0, 1.0], 1.0),
            &c.draw_state,
            transform,
            g
            );
        })
    }
}
