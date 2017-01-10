use std::fs::File;
use std::io::BufReader;

use glium::{Program, Surface, Texture2d, VertexBuffer};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::index::{IndexBuffer, PrimitiveType};
use image::{self, GenericImage, ImageFormat};

use units::drawing::V3;

// NOTE: these are not necessarily hard limits, though exceeding them
//       will at best cause reallocation on the heap, at worst this will
//       blow up OpenGL.

// renderer settings
pub static MAX_PARTICLES: usize = 256;
pub static MAX_RECTS: usize = 1000;
pub static MAX_TEXTURES: usize = 128;

// shader etc ...
static SHD_SQUARE_VTX: &'static str = include_str!("../assets/shaders/square.glsv");
static SHD_SQUARE_FRG: &'static str = include_str!("../assets/shaders/square.glsf");

/// BasicShader is a simple GPU program: 
/// - plots verts as triangles (index buffer = identity)
/// - the verts are interleaved w/ coords in UV space
/// - UV coords are passed to the fragment shader for simple 2D texturing.
struct BasicShader {
    pub vbuf: VertexBuffer<V3>,
    pub ibuf: IndexBuffer<u16>,
    pub blank_tex: Texture2d,
    pub rect_prog: Program,
}

impl BasicShader {
    pub fn new<F: Facade>(display: &F) -> Self {
        // generates a default pink/black checkered texture
        //
        // this is used by the renderer whenever a texture is requested
        // by the engine, but not available ...
        // (e.g: async load has not finished, file not found, resource loader
        //  ran out of memory, etc.)
        //
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

    verts: Vec<>
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
         let mut rot = [0.0, 0.0];

         for job in draw_list {
            match *job {
                RenderJob::ClearDepth(depth)    => frame.clear_depth(depth),
                RenderJob::ClearScreen(r,g,b,a) => frame.clear_color(r,g,b,a),
                RenderJob::UniformOffset(uofs)  => ofs = uofs,
                RenderJob::UniformRotate(urot)  => rot = urot,
                RenderJob::ResetUniforms        => { ofs = [0.0, 0.0]; rot = [0.0, 0.0] },

                RenderJob::TexRect(texture_id, x, y, z, w, h) => {
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

                    let rot = rot[0];
                    let mat = [[ rot.cos(), -rot.sin(), 0.0, 0.0],
                               [ rot.sin(),  rot.cos(), 0.0, 0.0],
                               [       0.0,        0.0, 1.0, 0.0],
                               [       0.0,        0.0, 0.0, 1.0f32]];

                    let uniforms = uniform! {
                        tex:  &self.textures[texture_id],
                        rot:  mat,
                        tofs: ofs,
                    };

                    { // render a quad into the vertex buffer
                        self.shader.ibuf.invalidate();
                        self.shader.vbuf.invalidate();

                        let ibuf = self.shader.ibuf.slice_mut(0..6).unwrap();
                        let vbuf = self.shader.vbuf.slice_mut(0..4).unwrap();

                        vbuf.write(&[
                            V3 { pos: [x1, y1, z], uv: [ 0.0,  0.0] },
                            V3 { pos: [x1, y2, z], uv: [ 0.0,  1.0] },
                            V3 { pos: [x2, y1, z], uv: [ 1.0,  0.0] },
                            V3 { pos: [x2, y2, z], uv: [ 1.0,  1.0] },
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

    pub fn load_tga(&mut self, path: &str) -> usize {
        // load the TGA and flip it so the coordinate system matches GL
        let file    = File::open(path).expect("could not read sprite");
        let buf_io  = BufReader::new(file);
        let tga_buf = image::load(buf_io, ImageFormat::TGA)
                            .expect("could not parse TGA file")
                            .flipv();

        // allocate CPU-side storage for the image
        let (dim_x, dim_y) = (tga_buf.width() as usize, tga_buf.height() as usize);
        let mut buf = vec![vec![(0u8,0u8,0u8,0u8); dim_x]; dim_y];
        let mut pixels = tga_buf.as_rgba8().unwrap().pixels();


        // copy the image into CPU-side buffer
        for y in 0..dim_y {
            for x in 0..dim_x {
                let pixel = pixels.next().unwrap();
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = pixel[3];

                buf[y][x] = (r,g,b,a);
            }
        }

        self.store_texture(buf)
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

    // TODO: grosssssss... state in my renderer?
    UniformOffset([f32; 2]),
    UniformRotate([f32; 2]),
    ResetUniforms,
    TexRect(usize, f32, f32, f32, f32, f32),
}
