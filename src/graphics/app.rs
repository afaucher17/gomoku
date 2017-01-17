use graphics;
use graphics::image;
use glium;
use graphics::glutin;
use glium::Surface;
use std::io::Cursor;
use glium::{VertexBuffer, Program, Frame, DisplayBuild};
use glium::texture::texture2d::Texture2d;
use glium::texture::RawImage2d;
use glium::backend::glutin_backend::GlutinFacade;

use board::{Board, Move, BoardState, Square};
use game::{Game};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct App {
    texture_grid: Texture2d,
    texture_black: Texture2d,
    texture_white: Texture2d,
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
}

impl App {
    fn init_program(display: &GlutinFacade) -> Program {
        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#;
        let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        voidmain() {
            color = texture(tex, v_tex_coords)<F15>;
        }
        "#;

        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
    }

    fn init_vertex_buffer(display: &GlutinFacade) -> VertexBuffer<Vertex> {
        let vertex1 = Vertex { position: [ -1.0, 1.0 ], tex_coords: [ 0.0, 1.0 ]};
        let vertex2 = Vertex { position: [ 0.5, 1.0], tex_coords: [ 1.0, 1.0 ]};
        let vertex3 = Vertex { position: [ -1.0, -1.0], tex_coords: [ 0.0, 0.0 ] };
        let vertex4 = Vertex { position: [ 0.5, -1.0], tex_coords: [ 1.0, 0.0 ] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        glium::VertexBuffer::new(display, &shape).unwrap()
    }

    fn init_texture(display: &GlutinFacade, cursor: Cursor<&[u8]>) -> Texture2d {
        let image = image::load(cursor, image::PNG).unwrap().to_rgba();

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new(display, image).unwrap()
    }

    pub fn load(&mut self, display: &GlutinFacade)
    {
        self.vertex_buffer = App::init_vertex_buffer(display);
        self.program = App::init_program(display);
    }

    pub fn new(display: &GlutinFacade) -> Self {
        implement_vertex!(Vertex, position, tex_coords);

        let texture_grid = App::init_texture(display, Cursor::new(&include_bytes!("../../resources/grid-color.png")[..]));
        let texture_black = App::init_texture(display, Cursor::new(&include_bytes!("../../resources/black_stone.png")[..]));
        let texture_white = App::init_texture(display, Cursor::new(&include_bytes!("../../resources/white_stone.png")[..]));
        let vertex_buffer = App::init_vertex_buffer(display);
        let program = App::init_program(display);

        App {
            texture_grid: texture_grid,
            texture_black: texture_black,
            texture_white: texture_white,
            vertex_buffer: vertex_buffer,
            program: program,
        }
    }

    fn draw_board(&self, board: &Board, target: &mut Frame)
    {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        for i in 0..19
        {
            for j in 0..19
            {

                let create_uniforms = |tex| {
                    uniform! {
                        matrix: [
                            [0.035, 0.0, 0.0, 0.0],
                            [0.0, 0.035, 0.0, 0.0],
                            [0.0, 0.0, 0.035, 0.0],
                            [-1.0 + (j as f32 + 1.1) * 0.075, -1.0 + (i as f32 + 1.0) * 0.1, 0.0, 1.0f32],
                        ],
                        tex: tex,
                    }
                };
                match board.state[i][j] {
                    Square::White => 
                        target.draw(&(self.vertex_buffer), &indices, &(self.program),
                                    &create_uniforms(&(self.texture_white)), &Default::default()).unwrap(),
                    Square::Black =>
                        target.draw(&(self.vertex_buffer), &indices, &(self.program),
                                    &create_uniforms(&(self.texture_black)), &Default::default()).unwrap(),
                    Square::Empty => ()
                }
            }
        }
    }



    pub fn on_render(&self, display: &GlutinFacade, game: &Game)
    {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &(self.texture_grid),
        };

        target.draw(&(self.vertex_buffer), &indices, &(self.program), &uniforms, &Default::default()).unwrap();
        self.draw_board(&game.board, &mut target);
        target.finish().unwrap();

        /*    let factory = win.factory.clone();
              let mut glyphs = Glyphs::new(&self.font, factory).unwrap();
              win.draw_2d(e, |c, g| {
              clear(color::WHITE, g);
              {
              Rectangle::new([1.0, 0.91, 0.5, 1.0]).
              draw([0.0, 0.0, 840.0, 840.0],
              &c.draw_state,
              c.transform,
              g);
              let grid = grid::Grid {
              cols: 19,
              rows: 19,
              units: 40.0,
              };
              let rec_trans = c.transform.trans(40.0, 40.0);
              grid.draw(&Line::new([0.0, 0.0, 0.0, 1.0], 1.0),
              &c.draw_state,
              rec_trans,
              g);

              self.draw_board(c.transform, &game.board, g);
              }
              {
              let state_trans = c.transform.trans(880.0, 40.0);
              let state = match game.board.game_state {
              BoardState::Victory(Square::Black) => "Black Victory!",
              BoardState::Victory(Square::White) => "White Victory!",
              BoardState::Victory(Square::Empty) => "Empty Victory!",
              BoardState::FiveAligned(Square::Black) => "Black has five aligned",
              BoardState::FiveAligned(Square::White) => "White has five aligned",
              BoardState::FiveAligned(Square::Empty) => "Empty has five aligned",
              BoardState::Draw => "Draw",
              BoardState::InProgress => "Game in progress",
              };
              text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
              .draw(state, &mut glyphs, &c.draw_state, state_trans, g);
              }
              {
              let b_cap_trans = c.transform.trans(880.0, 80.0);
              let b_capture = format!("{}{}", game.board.b_capture.to_string(), " stones taken by Black");
              text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
              .draw(b_capture.as_str(), &mut glyphs, &c.draw_state, b_cap_trans, g);
              }
              {
              let w_cap_trans = c.transform.trans(880.0, 120.0);
              let w_capture = format!("{}{}", game.board.w_capture.to_string(), " stones taken by White");
              text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
              .draw(w_capture.as_str(), &mut glyphs, &c.draw_state, w_cap_trans, g);
              }
              {
              let last_trans = c.transform.trans(880.0, 160.0);
              let last_move = match game.last_move {
              Some(Move::Illegal) => "Illegal move".to_string(),
              Some(Move::DoubleThrees) => "Double Three move".to_string(),
              Some(Move::Legal(_, (x, y), ref color, _)) => format!("{} {} at ({}, {})", "Last move:", color, x.to_string(), y.to_string()),
              Some(Move::OutOfBounds) => "OutOfBounds should never happen".to_string(),
              Some(Move::Other(message)) => message.to_string(),
              _ => "No moves yet".to_string()
              };
              text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
              .draw(last_move.as_str(), &mut glyphs, &c.draw_state, last_trans, g);
              }
              {
              let time_trans = c.transform.trans(880.0, 200.0);
              if let Some(Move::Legal(_, _, _, time)) = game.last_move {
              let time_text = format!("Last move duration: {:.2}", time.num_milliseconds() as f64 / 1000.0);
              text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
              .draw(time_text.as_str(), &mut glyphs, &c.draw_state, time_trans, g);
              }
              }
    });*/
    }

    pub fn on_click(&self, mouse_pos: &[f64; 2]) -> Option<(usize, usize)> {
        let mut x = mouse_pos[0] - 40.0;
        x = x / 40.0;
        let mut y = mouse_pos[1] - 40.0;
        y = y / 40.0;
        if (x.fract().abs() < 0.3 || (x.fract() > 0.7 && x > 0.0)) && (y.fract().abs() < 0.3 || (y.fract() > 0.7 && x > 0.0)) {
            if x < 0.0 { x = 0.0 }
            if y < 0.0 { y = 0.0 }
            Some((x.round() as usize, y.round() as usize))
        }
        else { None }
    }
}
