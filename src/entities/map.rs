use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;
use rand::{thread_rng, Rng};

use input::Input;
use render::{TexRect, RenderGroup, RenderJob};
use units::{dt2ms, Direction};

static MAP_SIZE: usize = 256;
static SCROLL_V: f32 = 0.001; // (.001px * 1000ms) = 1 texture height / sec.// game settings

pub struct TileMap {
    ofs_x: f32, ofs_y: f32,
    rotation: f32,
    stars: usize,
}

impl TileMap {
    pub fn new(display: &mut RenderGroup) -> Self {
        let mut buf = vec![vec![(0,0,0,0); MAP_SIZE]; MAP_SIZE];

        let rtex = display.store_texture(fill_star(&mut buf)); 
        TileMap {
            ofs_x:    0.0, 
            ofs_y:    0.0,
            rotation: 0.0,

            stars: rtex,
        }
    }

    pub fn update(&mut self, controller: &Input, dt: Duration, dv: (f32,f32)) {
        self.integrate(dt, dv);
       //       if controller.is_key_held(VKC::W) { self.integrate(dt, dv, Direction::Up)    }
       //  else if controller.is_key_held(VKC::A) { self.integrate(dt, dv, Direction::Left)  }
       //  else if controller.is_key_held(VKC::S) { self.integrate(dt, dv, Direction::Down)  }
       //  else if controller.is_key_held(VKC::D) { self.integrate(dt, dv, Direction::Right) }
       if controller.is_key_held(VKC::Q) { self.rotate(dt, Direction::Left)  }
       if controller.is_key_held(VKC::E) { self.rotate(dt, Direction::Right) }
    }

    fn integrate(&mut self, dt: Duration, dv: (f32,f32)) {
        // TODO: real vectors ...
        // integrate velocity over time => offset distance
        self.ofs_x += -dv.0 * dt2ms(dt) as f32;
        self.ofs_y += -dv.1 * dt2ms(dt) as f32;
    }

    fn rotate(&mut self, dt: Duration, dir: Direction) {
        let vr = match dir {
            Direction::Left  => -SCROLL_V,
            Direction::Right =>  SCROLL_V,
            _ => panic!("tilemap cannot rotate this direction ..."),
        };

        self.rotation += vr * dt2ms(dt) as f32;
    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // TODO: normalized coords
        let (w,h) = (1.0, 1.0);
        jobs.push(RenderJob::UniformOffset([self.ofs_x, self.ofs_y]));
        // jobs.push(RenderJob::UniformRotate([self.rotation, 0.0]));
        jobs.push(RenderJob::Draw(TexRect::from(self.stars, 0.0, 0.0, 0.0, w, h)));
        jobs.push(RenderJob::ResetUniforms);
    }
}

// builds star fields
fn fill_star(buf: &mut Vec<Vec<(u8,u8,u8,u8)>>) -> Vec<Vec<(u8,u8,u8,u8)>> {
    let mut rng   = thread_rng();
    
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            if !rng.gen_weighted_bool(256) { continue }
            buf[y][x] = (255,255,255,255);
        }
    }

    buf.clone()
}

// fn fill_64_64(buf: &mut Vec<Vec<(u8,u8,u8,u8)>>, color: (u8,u8,u8,u8)) -> Vec<Vec<(u8,u8,u8,u8)>> {
//     for row in 0..64 {
//         for col in 0..64 { buf[row][col] = color }
//     }
// 
//     buf.clone()
// }
