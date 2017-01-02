use glium::{self, Frame, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::Texture2d;

use units::drawing::V2;

static SHD_SQUARE_VTX: &'static str = include_str!("../assets/shaders/square.glsv");
static SHD_SQUARE_FRG: &'static str = include_str!("../assets/shaders/square.glsf");

pub struct Square {
    prog:  Program,
    tex:   Texture2d,
    vbuf:  VertexBuffer<V2>,
    ibuf:  NoIndices,

    ofs: [f32; 2],
}

impl Square {
    pub fn new<F: Facade>(display: &F) -> Self {
        // build rainbow box stripes
        let mut buf = vec![vec![(0u8,0u8,0u8, 0u8); 256]; 256];
        // for row in   0.. 64 { for col in 0..256 { buf[row][col] = (255,   0,   0, 255); } }
        // for row in  64..128 { for col in 0..256 { buf[row][col] = (  0, 255,   0, 255); } }
        // for row in 128..192 { for col in 0..256 { buf[row][col] = (  0,   0, 255, 255); } }
        // for row in 192..256 { for col in 0..256 { buf[row][col] = (255,   0, 255, 255); } }

        // draw boxes w/ stride of 8*8 pixels
        // each box has an alternating color (chosen by array offset modulo 2)
        // each box takes up 32*32 pixels, yielding (32*8 * 32*8) total pixels, e.g 256*256
        for sh in 0..8 {
            for sw in 0..8 {
                let color = (sh + sw) % 2 == 0;
                let y = sh * 32;
                let x = sw * 32;

                // draw boxes
                for row in y..(y+32) {
                    for col in x..(x+32) {
                        buf[row][col] = if color { (255,0,0,255) } else { (0,0,0,255) };
                    }
                }

            }
        }

        // stuff boxes into texture 
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
        let vbuf = VertexBuffer::new(display, &quad[..]).expect("could not allocate vbuf");
        let indices = NoIndices(PrimitiveType::TrianglesList);

        Square { 
            prog: program,
            tex:  texture,
            vbuf: vbuf,
            ibuf: indices,

            ofs: [0.0, 0.0],
        }
    }

    pub fn up(&mut self) {
        self.ofs[1] += 0.01;
        if self.ofs[1] >= 1.0 { self.ofs[1] = 0.0; }
    }

    pub fn right(&mut self) {
        self.ofs[0] += 0.01;
        if self.ofs[0] >= 1.0 { self.ofs[0] = 0.0; }
    }

    pub fn down(&mut self) {
        self.ofs[1] -= 0.01;
        if self.ofs[1] <= 0.0 { self.ofs[1] = 1.0; }
    }

    pub fn left(&mut self) {
        self.ofs[0] -= 0.01;
        if self.ofs[0] <= 0.0 { self.ofs[0] = 1.0; }
    }

    pub fn draw(&self, frame: &mut Frame, params: &glium::DrawParameters) {
        let uniforms = uniform! {
            tex: &self.tex,
            tofs: self.ofs,
        };

        frame.draw(&self.vbuf, &self.ibuf, &self.prog, &uniforms, params)
             .expect("could not draw tri");

    }

    pub fn dump(&self) {
        println!("square ofs: {:?}", self.ofs);
    }
}
