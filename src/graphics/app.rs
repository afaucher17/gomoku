use glium;

use std::io::Cursor;

use graphics::image;
use graphics::{glium_text, cgmath};

use graphics::glium_text::{TextSystem, FontTexture};

use glium::Surface;
use glium::{VertexBuffer, Program, Frame};
use glium::texture::texture2d::Texture2d;
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
    text_system: TextSystem,
    font: FontTexture,
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

        void main() {
            color = texture(tex, v_tex_coords);
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

    fn init_text_system(display: &GlutinFacade) -> TextSystem
    {
        glium_text::TextSystem::new(display)
    }

    fn init_font(display: &GlutinFacade) -> FontTexture
    {
       glium_text::FontTexture::new(display,
            &include_bytes!("../../resources/FiraSans-Regular.ttf")[..], 70).unwrap()
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
        let text_system = App::init_text_system(display);
        let font = App::init_font(display);

        App {
            texture_grid: texture_grid,
            texture_black: texture_black,
            texture_white: texture_white,
            vertex_buffer: vertex_buffer,
            program: program,
            text_system: text_system,
            font: font,
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
                            [-1.0 + (j as f32 + 1.1) * 0.075, 1.0 - (i as f32 + 1.0) * 0.1, 0.0, 1.0f32],
                        ],
                        tex: tex,
                    }
                };
                match board.state[j][i] {
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

    pub fn draw_text(&self, display: &GlutinFacade, game: &Game, target: &mut Frame)
    {

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
        let state_text = glium_text::TextDisplay::new(&(self.text_system), &(self.font), state);

        let b_capture = format!("{}{}", game.board.b_capture.to_string(), " stones taken by Black");
        let b_capture_text = glium_text::TextDisplay::new(&(self.text_system), &(self.font), b_capture.as_str());

        let w_capture = format!("{}{}", game.board.w_capture.to_string(), " stones taken by White");
        let w_capture_text = glium_text::TextDisplay::new(&(self.text_system), &(self.font), w_capture.as_str());

        let last_move = match game.last_move {
            Some(Move::Illegal) => "Illegal move".to_string(),
            Some(Move::DoubleThrees) => "Double Three move".to_string(),
            Some(Move::Legal(_, (x, y), ref color, _)) => format!("{} {} at ({}, {})", "Last move:", color, x.to_string(), y.to_string()),
            Some(Move::OutOfBounds) => "Out of Bounds".to_string(),
            Some(Move::Other(message)) => message.to_string(),
            _ => "No moves yet".to_string(),
        };
        let last_move_text = glium_text::TextDisplay::new(&(self.text_system), &(self.font), last_move.as_str());




        let (w, h) = display.get_framebuffer_dimensions();

        let state_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            0.25 / 10.0, 0.0, 0.0, 0.0,
            0.0, 0.25 * (w as f32) / (h as f32) / 10.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.52, 0.91, 0.0, 1.0f32,
            ).into();

        let b_capture_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            0.25 / 10.0, 0.0, 0.0, 0.0,
            0.0, 0.25 * (w as f32) / (h as f32) / 10.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.52, 0.81, 0.0, 1.0f32,
            ).into();

        let w_capture_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            0.25 / 10.0, 0.0, 0.0, 0.0,
            0.0, 0.25 * (w as f32) / (h as f32) / 10.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.52, 0.71, 0.0, 1.0f32,
            ).into();

        let last_move_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            0.25 / 10.0, 0.0, 0.0, 0.0,
            0.0, 0.25 * (w as f32) / (h as f32) / 10.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.52, 0.61, 0.0, 1.0f32,
            ).into();

         let time_matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            0.25 / 10.0, 0.0, 0.0, 0.0,
            0.0, 0.25 * (w as f32) / (h as f32) / 10.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.52, 0.51, 0.0, 1.0f32,
            ).into();
       

        glium_text::draw(&state_text, &(self.text_system), target, state_matrix, (0.0, 0.0, 0.0, 1.0));
        glium_text::draw(&b_capture_text, &(self.text_system), target, b_capture_matrix, (0.0, 0.0, 0.0, 1.0));
        glium_text::draw(&w_capture_text, &(self.text_system), target, w_capture_matrix, (0.0, 0.0, 0.0, 1.0));
        glium_text::draw(&last_move_text, &(self.text_system), target, last_move_matrix, (0.0, 0.0, 0.0, 1.0));

        if let Some(Move::Legal(_, _, _, time)) = game.last_move {
            let time = format!("Last move duration: {:.2}", time.num_milliseconds() as f64 / 1000.0);
            let time_text = glium_text::TextDisplay::new(&(self.text_system), &(self.font), time.as_str());
            glium_text::draw(&time_text, &(self.text_system), target, time_matrix, (0.0, 0.0, 0.0, 1.0));
        };
    }


    pub fn on_render(&self, display: &GlutinFacade, game: &Game)
    {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
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
        self.draw_text(display, game, &mut target);
        target.finish().unwrap();
    }

    pub fn on_click(&self, mouse_pos: &[f64; 2], size_pixels: (u32, u32)) -> Option<(usize, usize)> {
        let (spx, spy) = size_pixels;
        let ratiox = 0.75 * spx as f64 / 20.0;
        let ratioy = spy as f64 / 20.0;
        let mut x = mouse_pos[0] - ratiox;
        x = x / ratiox;
        let mut y = mouse_pos[1] - ratioy;
        y = y / ratioy;
        if (x.fract().abs() < 0.3 || (x.fract() > 0.7 && x > 0.0)) && (y.fract().abs() < 0.3 || (y.fract() > 0.7 && x > 0.0)) {
            if x < 0.0 { x = 0.0 }
            if y < 0.0 { y = 0.0 }
            Some((x.round() as usize, y.round() as usize))
        }
        else { None }
    }
}
