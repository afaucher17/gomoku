#![macro_use]
extern crate gomoku;
extern crate glium;
extern crate glutin;

use glium::DisplayBuild;
use gomoku::board::{BoardState};
use gomoku::game::{Game};
use gomoku::graphics::{Settings, App};

fn main() {
    let settings = Settings::new();

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Gomoku".to_string())
        .with_dimensions(settings.win_size.x as u32, settings.win_size.y as u32)
        .with_vsync()
        .build_glium()
        .unwrap();
//    window.set_window_resize_callback(Some(redraw as fn(u32, u32)));

    let app = App::new(&display);
    let mut mouse_pos = [0f64, 0f64];
    let mut game = Game::new(true);

    'main: loop {
        app.on_render(&display, &game);
        for event in display.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
                    let pos;
                    {
                        pos = app.on_click(&mouse_pos, display.get_window().unwrap().get_inner_size_pixels().unwrap())
                    }
                    match game.board.game_state {
                        BoardState::InProgress | BoardState::FiveAligned(_, _) => game.play(pos),
                        _ => ()
                    }
                }
                glutin::Event::MouseMoved(x, y) => mouse_pos = [x as f64, y as f64],
                _ => {}
            }
        }
        game.update();
    }
}
