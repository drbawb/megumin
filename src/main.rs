#[macro_use] extern crate glium;
extern crate rusttype;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Duration, Instant};
use std::thread;


use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::glutin::{Event, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{Texture2d};


use rusttype::FontCollection;
use rusttype::{Point as RTPoint, Scale as RTScale};

static FONT_PATH: &'static str = "./assets/DroidSansMono.ttf";

#[derive(Copy, Clone, Debug)]
struct V2 { pos: [f32; 2], tex_coords: [f32; 2] }
implement_vertex!(V2, pos, tex_coords);

#[derive(Copy, Clone, Debug)]
struct V3 { pos: [f32; 3] }
implement_vertex!(V3, pos);

static V_SHADE_TEXT: &'static str = r#"
#version 140

in vec2 pos;
in vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(pos, 0.0, 1.0);
}
"#;

static F_SHADE_TEXT: &'static str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}
"#;


fn main() {
    println!("initializing display ...");
    let display = WindowBuilder::new()
                                .build_glium()
                                .expect("could not open window");

    // read font file
    let mut otf_buf  = vec![];
    let mut otf_file = File::open(&Path::new(FONT_PATH))
                            .expect("could not read file ...");

    let size = otf_file.read_to_end(&mut otf_buf).expect("could not read font file");
    println!("read {} bytes from {}", size, FONT_PATH);

    
    let font = FontCollection::from_bytes(&otf_buf[..])
                              .into_font()
                              .expect("loaded one font ...");

 
    // render font 
    let layout = font.layout("hi wtf", RTScale { x: 32.0, y: 32.0 }, RTPoint { x: 0.0, y: 0.0 });
   
    // build rainbow box stripes
    let mut buf = vec![vec![(0u8,0u8,0u8, 0u8); 256]; 256];
    for row in   0.. 64 { for col in 0..256 { buf[row][col] = (255,   0,   0, 255); } }
    for row in  64..128 { for col in 0..256 { buf[row][col] = (  0, 255,   0, 255); } }
    for row in 128..192 { for col in 0..256 { buf[row][col] = (  0,   0, 255, 255); } }
    for row in 192..256 { for col in 0..256 { buf[row][col] = (255,   0, 255, 255); } }

    // layout characters
    // let mut x_ofs = 0;
    // for glyph in layout {
    //     glyph.draw(|x,y,v| {
    //         let coverage = (v * 255.0) as u8;
    //         println!("({},{}), v: {})", x, y, coverage);
    //         buf[y as usize][(x_ofs + x) as usize] = (255,255,0,coverage);

    //     });

    //     match glyph.pixel_bounding_box() {
    //         Some(bb) => x_ofs += bb.width() as u32,
    //         None => x_ofs += 16,
    //     };
    // }

    let texture = Texture2d::new(&display, buf).unwrap();

    // setup shaders ...
    let vtx_shader_basic = r#"
    #version 140

    in vec2 pos;
    in vec2 tex_coords;
    out vec2 vt_coords;

    void main() {
        vt_coords = tex_coords;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
    "#;

    let frag_shader_basic = r#"
    #version 140

    in vec2 vt_coords;
    out vec4 color;

    uniform sampler2D tex;
    uniform float tofs;

    void main() {
        color = texture(tex, vec2(vt_coords.x, vt_coords.y + tofs));
    }
    "#;

    let program = Program::from_source(&display, vtx_shader_basic, frag_shader_basic, None)
                          .expect("could not load basic shader");

    println!("starting game loop ...");

    let mut frame_time = Duration::from_millis(1000 / 60); // 60FPS
    let mut t_ofs = 0.0f32;

    let q_a = V2 { pos: [-0.5, -0.5], tex_coords: [ 0.0,  1.0] };
    let q_b = V2 { pos: [ 0.5, -0.5], tex_coords: [ 1.0,  1.0] };
    let q_c = V2 { pos: [ -0.5, 0.5], tex_coords: [ 0.0,  0.0] };
    let q_d = V2 { pos: [ 0.5,  0.5], tex_coords: [ 1.0,  0.0] };
    let quad = vec![q_c, q_a, q_b, q_c, q_d, q_b];

    'runloop: loop {
        // handle input
        for ev in display.poll_events() {
            match ev {
                Event::Closed => break 'runloop,
                _ => (),
            }
        }

        let vbuf = VertexBuffer::new(&display, &quad).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // shitty animation function
        t_ofs += 0.01;
        
        let uniforms = uniform! {
            tex: &texture,
            tofs: t_ofs,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&vbuf, &indices, &program, &uniforms, &draw_params)
             .expect("could not draw tri");

        frame.finish().expect("could not close frame");
        thread::sleep(frame_time);
    }

    println!("goodbye ...");
}
