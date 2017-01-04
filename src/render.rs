use glium::{Program, Surface, Texture2d, VertexBuffer};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::index::{NoIndices, PrimitiveType};

use units::drawing::V2;

// renderer settings
static SCREEN_W: f32 = 1024.0;
static SCREEN_H: f32 =  768.0;
static MAX_RECTS: usize = 1000;

// shader etc ...
static SHD_SQUARE_VTX: &'static str = include_str!("../assets/shaders/square.glsv");
static SHD_SQUARE_FRG: &'static str = include_str!("../assets/shaders/square.glsf");

struct BasicShader {
    pub vbuf: VertexBuffer<V2>,
    pub ibuf: NoIndices,
    pub blank_tex: Texture2d,
    pub rect_prog: Program,
}

impl BasicShader {
    pub fn new<F: Facade>(display: &F) -> Self {
        let mut buf = vec![vec![(0u8, 0u8, 0u8, 0u8); 256]; 256];
        gen_checkers(&mut buf);

        let texture = Texture2d::new(display, buf)
                                .expect("could not build fallback texture");

        let program = Program::from_source(display, SHD_SQUARE_VTX, SHD_SQUARE_FRG, None)
                              .expect("could not load basic shader");


        let verts_buffer = VertexBuffer::empty_dynamic(display, (MAX_RECTS * 6))
                                        .expect("could not allocate empty vertex buffer");

        BasicShader {
            vbuf: verts_buffer,
            ibuf: NoIndices(PrimitiveType::TrianglesList),
            blank_tex: texture,
            rect_prog: program,
        }
    }
}

fn gen_checkers(buf: &mut Vec<Vec<(u8,u8,u8,u8)>>) {
    for sh in 0..8 {
        for sw in 0..8 {
            let color = (sh + sw) % 2 == 0;
            let y = sh * 32;
            let x = sw * 32;

            // draw boxes
            for row in y..(y+32) {
                for col in x..(x+32) {
                    buf[row][col] = if color { (255,0,255,255) } else { (0,0,0,255) };
                }
            }

        }
    }
}

/// Glutin renderer implementation
/// Stores a reference to the glutin window along w/ a basic shader
/// program and GL parameters.
///
/// These are ultimately used to render a single scene by operating on a list
/// of render jobs in an order decided by the renderer.
pub struct RenderGroup<'scn> {
    gpu:     &'scn GlutinFacade,
    config:  &'scn DrawParameters<'scn>,
    shader:  BasicShader,
}

impl<'scn> RenderGroup<'scn> {
    pub fn new(display: &'scn GlutinFacade, draw_params: &'scn DrawParameters<'scn>) -> RenderGroup<'scn> {
        let gpu_program = BasicShader::new(display);

        RenderGroup {
            config: draw_params,
            gpu:    display,
            shader: gpu_program,
        }
    }

    pub fn draw(&mut self, draw_list: &[RenderJob]) {
         let mut frame = self.gpu.draw();
         let mut ofs = [0.0, 0.0];

         for job in draw_list {
            match *job {
                RenderJob::ClearDepth(depth)    => frame.clear_depth(depth),
                RenderJob::ClearScreen(r,g,b,a) => frame.clear_color(r,g,b,a),
                RenderJob::UniformOffset(uofs)  => ofs = uofs,
                RenderJob::DrawRect(rect) => {

                    // rect bounds
                    let x1 = rect.x; let x2 = rect.x + (rect.w as i32);
                    let y1 = rect.y; let y2 = rect.y + (rect.h as i32);

                    let x1 = (x1 as f32) / SCREEN_W;
                    let x2 = (x2 as f32) / SCREEN_W;
                    let y1 = (y1 as f32) / SCREEN_H;
                    let y2 = (y2 as f32) / SCREEN_W;

                    { // render a quad into the vertex buffer
                        self.shader.vbuf.invalidate();
                        let mut writer = self.shader.vbuf.map_write();
                        writer.set(0, V2 { pos: [x2, y2], uv: [ 1.0,  1.0] });
                        writer.set(1, V2 { pos: [x1, y2], uv: [ 0.0,  1.0] });
                        writer.set(2, V2 { pos: [x1, y1], uv: [ 0.0,  0.0] });
                        writer.set(3, V2 { pos: [x2, y2], uv: [ 1.0,  1.0] });
                        writer.set(4, V2 { pos: [x2, y1], uv: [ 1.0,  0.0] });
                        writer.set(5, V2 { pos: [x1, y1], uv: [ 0.0,  0.0] });
                    }

                    let uniforms = uniform! {
                        tex:  &self.shader.blank_tex,
                        tofs: ofs,
                    };

                    frame.draw(&self.shader.vbuf, 
                               &self.shader.ibuf, 
                               &self.shader.rect_prog, 
                               &uniforms, 
                               self.config).expect("could not draw tri");

                },
            }
        }       

        frame.finish().expect("did not finish rendering frame ...");
    }
}

// renderer primitives below here ...

#[derive(Copy,Clone)]
pub struct Rect {
    pub x: i32, pub y: i32,
    pub w: u32, pub h: u32,
}


#[derive(Copy, Clone)]
pub enum RenderJob {
    ClearDepth(f32),
    ClearScreen(f32, f32, f32, f32),
    DrawRect(Rect),
    UniformOffset([f32; 2]),
}
