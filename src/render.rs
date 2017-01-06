use glium::{Program, Surface, Texture2d, VertexBuffer};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::index::{IndexBuffer, PrimitiveType};

use units::drawing::V2;

// renderer settings
static MAX_RECTS: usize = 1000;
static MAX_TEXTURES: usize = 128;

// shader etc ...
static SHD_SQUARE_VTX: &'static str = include_str!("../assets/shaders/square.glsv");
static SHD_SQUARE_FRG: &'static str = include_str!("../assets/shaders/square.glsf");

/// BasicShader is a simple GPU program: 
/// - plots verts as triangles (index buffer = identity)
/// - the verts are interleaved w/ coords in UV space
/// - UV coords are passed to the fragment shader for simple 2D texturing.
struct BasicShader {
    pub vbuf: VertexBuffer<V2>,
    pub ibuf: IndexBuffer<u16>,
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


        let verts_buffer = VertexBuffer::empty_dynamic(display, (MAX_RECTS * 4))
                                        .expect("could not allocate empty vertex buffer");

        let index_buffer = IndexBuffer::empty_dynamic(display, PrimitiveType::TrianglesList, (MAX_RECTS * 6))
                                       .expect("could not allocate empty index buffer");

        BasicShader {
            vbuf: verts_buffer,
            ibuf: index_buffer,
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
/// program and GL parameters. This rendergroup is only valid in the
/// thread which owns this GPU context, and must not outlive this context.
///
pub struct RenderGroup<'scn> {
    gpu:    &'scn GlutinFacade,
    config:  &'scn DrawParameters<'scn>,
    shader:  BasicShader,

    textures: Vec<Texture2d>,
}

impl<'scn> RenderGroup<'scn> {
    pub fn new(display: &'scn GlutinFacade, draw_params: &'scn DrawParameters<'scn>) -> RenderGroup<'scn> {
        let gpu_program = BasicShader::new(display);

        RenderGroup {
            config: draw_params,
            gpu:   display,
            shader: gpu_program,

            textures: Vec::with_capacity(MAX_TEXTURES),
        }
    }

    pub fn draw<S: Surface>(&mut self, draw_list: &[RenderJob], frame: &mut S) {
         let mut ofs = [0.0, 0.0];
         for job in draw_list {
            match *job {
                RenderJob::ClearDepth(depth)    => frame.clear_depth(depth),
                RenderJob::ClearScreen(r,g,b,a) => frame.clear_color(r,g,b,a),
                RenderJob::UniformOffset(uofs)  => ofs = uofs,
                RenderJob::DrawRect(rect) => {
                    // draws a fixed size rectangle which is aspect corrected to the screen

                    // TODO: ugly casts ...
                    // give the world coordinate system some thought and define appropriate
                    // types in the units lib for this engine ...
                    
                    // rect bounds
                    let x1 = rect.x; let x2 = rect.x + (rect.w);
                    let y1 = rect.y; let y2 = rect.y + (rect.h);

                    // TODO: cpu aspect correction, fix w/ projection uniform
                    let (screen_w, screen_h) = self.gpu.get_framebuffer_dimensions();
                    let x1 = (x1 as f32) / (screen_w as f32);
                    let x2 = (x2 as f32) / (screen_w as f32);
                    let y1 = (y1 as f32) / (screen_h as f32);
                    let y2 = (y2 as f32) / (screen_h as f32);

                    // TODO: better way to do this? memmove, etc?
                    { // render a quad into the vertex buffer
                        self.shader.ibuf.invalidate();
                        self.shader.vbuf.invalidate();

                        let ibuf = self.shader.ibuf.slice_mut(0..6).unwrap();
                        let vbuf = self.shader.vbuf.slice_mut(0..4).unwrap();

                        vbuf.write(&[
                            V2 { pos: [x1, y1], uv: [ 0.0,  0.0] },
                            V2 { pos: [x1, y2], uv: [ 0.0,  1.0] },
                            V2 { pos: [x2, y1], uv: [ 1.0,  0.0] },
                            V2 { pos: [x2, y2], uv: [ 1.0,  1.0] },
                        ]);

                        ibuf.write(&[0,1,3, 0,2,3]);
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

                RenderJob::TexRect(texture_id, x, y, w, h) => {
                    // draws a normalized rectangle w/ a texture 
                    
                    // TODO: translate normalized coordinates (0=>1) to unit square
                    // f(x) = (2x) - 1
                    // f(0) = (2*0) - 1 = -1
                    // f(1) = (2*1) - 1 =  1
                    // (grows right, e.g: +)
                    //
                    // f(y) = (-2x) + 1
                    // f(0) = (-2*0) + 1 =  1
                    // f(1) = (-2*1) + 1 = -1
                    // (grows down, e.g: -)
                    // 
                    let x1 = (x *  2.0) - 1.0; let x2 = x1 + (w * 2.0);
                    let y1 = (y * -2.0) + 1.0; let y2 = y1 - (h * 2.0);

                    let uniforms = uniform! {
                        tex:  &self.textures[texture_id],
                        tofs: [0.0f32, 0.0],
                    };

                    { // render a quad into the vertex buffer
                        self.shader.ibuf.invalidate();
                        self.shader.vbuf.invalidate();

                        let ibuf = self.shader.ibuf.slice_mut(0..6).unwrap();
                        let vbuf = self.shader.vbuf.slice_mut(0..4).unwrap();

                        vbuf.write(&[
                            V2 { pos: [x1, y1], uv: [ 0.0,  0.0] },
                            V2 { pos: [x1, y2], uv: [ 0.0,  1.0] },
                            V2 { pos: [x2, y1], uv: [ 1.0,  0.0] },
                            V2 { pos: [x2, y2], uv: [ 1.0,  1.0] },
                        ]);

                        ibuf.write(&[0,1,3, 0,2,3]);
                    }

                    frame.draw(&self.shader.vbuf, 
                               &self.shader.ibuf, 
                               &self.shader.rect_prog, 
                               &uniforms, 
                               self.config).expect("could not draw tri");

                },
            }
        }       
    }

    // TODO: generic source? slice? etc.
    // TODO: enumerated color formats?
    // TODO: return result type
    /// Stores a 2D pixel buffer into a static texture and returns an
    /// integer handle to it which can be used to instruct the renderer
    /// to bank-in that texture for a program pass.
    pub fn store_texture(&mut self, buf: Vec<Vec<(u8,u8,u8,u8)>>) -> usize {
        let next_idx = self.textures.len();
        let texture  = Texture2d::new(self.gpu, buf)
                                 .expect("could not load userspace texture");

        self.textures.push(texture); next_idx
    }
}

// renderer primitives below here ...

#[derive(Copy,Clone)]
pub struct Rect {
    pub x: i32, pub y: i32,
    pub w: i32, pub h: i32,
}

pub enum RenderJob {
    ClearDepth(f32),
    ClearScreen(f32, f32, f32, f32),
    DrawRect(Rect),
    UniformOffset([f32; 2]), // TODO: grosssssss... state in my renderer?
    TexRect(usize, f32, f32, f32, f32),
}
