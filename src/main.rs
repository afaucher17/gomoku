#[macro_use]
extern crate gomoku;
extern crate piston_window;
extern crate find_folder;
extern crate opengl_graphics;

use piston_window::*;
use gomoku::game::{Game};
use gomoku::graphics::{Settings, App};

fn main() {
    let settings = Settings::new();
    let opengl = OpenGL::V3_2; 
    let mut window: PistonWindow =
        WindowSettings::new("Gomoku", [settings.win_size.x as u32, settings.win_size.y as u32])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    let app = App::new(/*settings, &mut window*/);
    let mut game = Game::new(true);
    let mut mouse_pos: [f64; 2] = [0.0; 2];
    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => app.on_render(&e, &mut window, &game),
            Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
                let mut pos = None;
                {
                    pos = app.on_click(&mouse_pos);
                }
                game.play(pos);
            },
            Event::Input(Input::Move(Motion::MouseCursor(_, _))) => mouse_pos = e.mouse_cursor_args().unwrap(),
            _ => ()
        }
        game.update();
    }
}
