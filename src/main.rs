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
                    let mut pos = None;
                    {
                        pos = app.on_click(&mouse_pos, display.get_window().unwrap().get_inner_size_pixels().unwrap())
                    }
                    match game.board.game_state {
                        BoardState::InProgress | BoardState::FiveAligned(_) => game.play(pos),
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
/*
   fn main() {
/*    let settings = Settings::new();
let opengl = OpenGL::V3_2; 
let mut window: PistonWindow =
WindowSettings::new("Gomoku", [settings.win_size.x as u32, settings.win_size.y as u32])
.exit_on_esc(true)
.opengl(opengl)
.build()
.unwrap();
let app = App::new(/*settings, &mut window*/);*/
let mut game = Game::new(true);
/*
   let mut mouse_pos: [f64; 2] = [0.0; 2];
   while let Some(e) = window.next() {
   match e {
   Event::Render(_) => app.on_render(&e, &mut window, &game),
   Event::Input(Input::Release(Button::Mouse(MouseButton::Left))) => {
   let mut pos = None;
   {
   pos = app.on_click(&mouse_pos);
   }
   match game.board.game_state {
   BoardState::InProgress | BoardState::FiveAligned(_) => game.play(pos),
   _ => ()
   }
   },
   Event::Input(Input::Move(Motion::MouseCursor(_, _))) => mouse_pos = e.mouse_cursor_args().unwrap(),
   _ => ()
   }
   game.update();
   }*/
}*/
