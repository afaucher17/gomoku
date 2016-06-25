#[macro_use]
extern crate gomoku;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use gomoku::board::{Board, Square};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Gomoku", [1200, 840])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

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

    let grid = grid::Grid {
        cols: 19,
        rows: 19,
        units: 40.0,
    };

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

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear(color::WHITE, g);
            let transform = c.transform.trans(40.0, 40.0);
            Rectangle::new([1.0, 0.91, 0.5, 1.0]).
                draw([0.0, 0.0, 840.0, 840.0],
                     &c.draw_state,
                     c.transform,
                     g
                    );
            grid.draw(&Line::new([0.0, 0.0, 0.0, 1.0], 1.0),
            &c.draw_state,
            transform,
            g
            );

            for i in 0..19
            {
                for j in 0..19
                {
                    let scale = c.transform.trans(27.5 + i as f64 * 40.0, 27.5 + j as f64 * 40.0).scale(0.1, 0.1);
                    match board.state[i][j] {
                        Square::White => Image::new().draw(&white_text, &DrawState::default(), scale, g),
                        Square::Black => Image::new().draw(&black_text, &DrawState::default(), scale, g),
                        Square::Empty => (),
                    }
                }
            }
        })
    }
}
