use glium::{self, Frame, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{Texture2d};

use units::drawing::V2;

static SHD_SQUARE_VTX: &'static str = include_str!("../assets/shaders/square.glsv");
static SHD_SQUARE_FRG: &'static str = include_str!("../assets/shaders/square.glsf");

pub struct Square {
    prog:  Program,
    tex:   Texture2d,
    vbuf:  VertexBuffer<V2>,
    ibuf:  NoIndices,

    ofs: f32,
}

impl Square {
    pub fn new<F: Facade>(display: &F) -> Self {
        // build rainbow box stripes
        let mut buf = vec![vec![(0u8,0u8,0u8, 0u8); 256]; 256];
        for row in   0.. 64 { for col in 0..256 { buf[row][col] = (255,   0,   0, 255); } }
        for row in  64..128 { for col in 0..256 { buf[row][col] = (  0, 255,   0, 255); } }
        for row in 128..192 { for col in 0..256 { buf[row][col] = (  0,   0, 255, 255); } }
        for row in 192..256 { for col in 0..256 { buf[row][col] = (255,   0, 255, 255); } }
        let texture = Texture2d::new(display, buf).unwrap();

        // setup shaders ...
        let program = Program::from_source(display, SHD_SQUARE_VTX, SHD_SQUARE_FRG, None)
                              .expect("could not load basic shader");

        // build shape
        let q_a = V2 { pos: [-0.5, -0.5], uv: [ 0.0,  1.0] };
        let q_b = V2 { pos: [ 0.5, -0.5], uv: [ 1.0,  1.0] };
        let q_c = V2 { pos: [ -0.5, 0.5], uv: [ 0.0,  0.0] };
        let q_d = V2 { pos: [ 0.5,  0.5], uv: [ 1.0,  0.0] };
        let quad = vec![q_c, q_a, q_b, q_c, q_d, q_b];
        let vbuf = VertexBuffer::new(display, &quad).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        Square { 
            prog: program,
            tex: texture,
            vbuf: vbuf,
            ibuf: indices ,

            ofs: 0.0,
        }
    }

    pub fn up(&mut self) {
        self.ofs += 0.01;
        if self.ofs >= 1.0 { self.ofs = 0.0; }
    }

    pub fn down(&mut self) {
        self.ofs -= 0.01;
        if self.ofs <= 0.0 { self.ofs = 1.0; }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let uniforms = uniform! {
            tex: &self.tex,
            tofs: self.ofs,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        frame.draw(&self.vbuf, &self.ibuf, &self.prog, &uniforms, &draw_params)
             .expect("could not draw tri");



    }
}
