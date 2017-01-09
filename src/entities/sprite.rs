use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{RenderJob, RenderGroup};
use units::dt2ms;

static SHIP_ACCEL: f32 = 0.00001; // .0001 increments => .001
static SHIP_VMAX: f32  = 0.001;  // (.001px * 1000ms) = 1 texture height / sec.

pub enum Direction { Up, Right, Down, Left }

pub struct Sprite {
     x: f32,  y: f32,
    vx: f32, vy: f32,
    rotation: f32,
    
    tx_idle:  usize,
    tx_fly_w: usize,
    tx_fly_a: usize,
    tx_fly_s: usize,
    tx_fly_d: usize,

    tx_fly_q: usize,
    tx_fly_e: usize,

    draw_tex: Option<usize>,

}

impl Sprite {
    pub fn new(display: &mut RenderGroup) -> Self {
        Sprite {
             x: 0.5,  y: 0.5,
            vx: 0.0, vy: 0.0,
            rotation: 0.0,

            tx_idle:  display.load_tga("assets/sprites/SHIPB001.tga"),
            tx_fly_w: display.load_tga("assets/sprites/SHIPW001.tga"),
            tx_fly_a: display.load_tga("assets/sprites/SHIPA001.tga"),
            tx_fly_s: display.load_tga("assets/sprites/SHIPS001.tga"),
            tx_fly_d: display.load_tga("assets/sprites/SHIPD001.tga"),
            tx_fly_q: display.load_tga("assets/sprites/SHIPQ001.tga"),
            tx_fly_e: display.load_tga("assets/sprites/SHIPE001.tga"),

            draw_tex: None,

        }
    }

    pub fn update(&mut self, controller: &Input, dt: Duration) {
        self.draw_tex = None;

             if controller.is_key_held(VKC::W) { self.draw_tex = Some(self.tx_fly_w); self.integrate(dt, Direction::Up)    }
        else if controller.is_key_held(VKC::A) { self.draw_tex = Some(self.tx_fly_a); self.integrate(dt, Direction::Left)  }
        else if controller.is_key_held(VKC::S) { self.draw_tex = Some(self.tx_fly_s); self.integrate(dt, Direction::Down)  }
        else if controller.is_key_held(VKC::D) { self.draw_tex = Some(self.tx_fly_d); self.integrate(dt, Direction::Right) }

             if controller.is_key_held(VKC::Q) { self.draw_tex = Some(self.tx_fly_q); self.rotate(dt, Direction::Left)     }
        else if controller.is_key_held(VKC::E) { self.draw_tex = Some(self.tx_fly_e); self.rotate(dt, Direction::Right)    }
    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // TODO: normalized coords
        let (w,h) = (0.15, 0.15);
        let cx = self.x - (w / 2.0);
        let cy = self.y - (h / 2.0);

        // rotate our sprite space &
        jobs.push(RenderJob::UniformRotate([self.rotation, 0.0]));
        jobs.push(RenderJob::TexRect(self.tx_idle, cx, cy, -0.5, w, h));

        match self.draw_tex {
            Some(tex_id) => jobs.push(RenderJob::TexRect(tex_id, cx, cy, -0.55, w, h)),
            None => {},
        }

        jobs.push(RenderJob::ResetUniforms());
    }

    pub fn velocity(&self) -> (f32, f32) { (self.vx, self.vy) }

    fn integrate(&mut self, dt: Duration, dir: Direction) {
        let (ax, ay) = match dir {
            Direction::Up    => (        0.0,  SHIP_ACCEL),
            Direction::Down  => (        0.0, -SHIP_ACCEL),
            Direction::Left  => ( SHIP_ACCEL,         0.0),
            Direction::Right => (-SHIP_ACCEL,         0.0),
        };

        let (max_x, max_y): (f32,f32) = match dir {
            Direction::Up    => (       0.0,  SHIP_VMAX),
            Direction::Down  => (       0.0, -SHIP_VMAX),
            Direction::Left  => (-SHIP_VMAX,        0.0),
            Direction::Right => ( SHIP_VMAX,        0.0),
        };

        // perform rotaiton of acceleration vector by hand
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        // println!("acc b4: ({},{})", ax, ay);
        let rax = (cos_r * ax) - (sin_r * ay);
        let ray = (sin_r * ax) + (cos_r * ay);
        // println!("acc ar: ({},{})", ax, ay);

        println!("max before: ({},{})", max_x, max_y);
        let max_rx = (cos_r * max_x) - (sin_r * max_y);
        let max_ry = (sin_r * max_x) + (cos_r * max_y);
        println!("max after: ({},{})", max_rx, max_ry);

        // apply force in direction of heading
        self.vx = self.vx + (rax * dt2ms(dt) as f32);
        self.vy = self.vy + (ray * dt2ms(dt) as f32);
        
        if rax < 0.0 { self.vx = f32::max(self.vx, max_rx); } 
        else { self.vx = f32::min(self.vx, max_rx); }

        if ray < 0.0 { self.vy = f32::max(self.vy, max_ry); }
        else { self.vy = f32::min(self.vy, max_ry); }

        println!("vx: {:?}, vy: {:?}", self.vx, self.vy);
    }

    fn rotate(&mut self, dt: Duration, dir: Direction) {
        let vr = match dir {
            Direction::Left  => -SHIP_VMAX,
            Direction::Right =>  SHIP_VMAX,
            _ => panic!("tilemap cannot rotate this direction ..."),
        };

        self.rotation += vr * dt2ms(dt) as f32;
        println!("rotation => {}", self.rotation);
    }
}
