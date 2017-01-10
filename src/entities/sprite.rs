use std::f32::consts as r32;
use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{self, Rect, TexRect, RenderJob, RenderGroup};
use units::{dt2ms, Direction};
use units::linear::V2;

// TODO: how to factor aspect out of here...
static SHIP_ACCEL_X: f32  = (128.0 / 1280.0) * 0.001 * 0.001; // px/s^2
static SHIP_ACCEL_Y: f32  = (128.0 /  720.0) * 0.001 * 0.001; // px/s^2
static SHIP_VMAX_X: f32   = (256.0 / 1280.0) * 0.001;         // px/s
static SHIP_VMAX_Y: f32   = (256.0 /  720.0) * 0.001;         // px/s
static SHIP_ROT:  f32   = r32::PI * 0.001;                    // rad/s
static BULLET_VMAX: f32 = 0.0007;

pub struct Particle {
     x: f32,  y: f32,
    vx: f32, vy: f32,

    pub is_alive: bool,
}

impl Particle {
    pub fn at_speed(x: f32, y: f32, vx: f32, vy: f32) -> Self {

        Particle {
             x:  x,  y:  y,
            vx: vx, vy: vy,

            is_alive: true,
        }
    }
}


pub struct Sprite {
    pos: V2, vel: V2,
    rotation: f32,

    particles: Vec<Particle>,

    tx_crate: usize,
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
            pos: V2::at(0.5, 0.5),
            vel: V2::at(0.0, 0.0),
            rotation: 0.0,

            particles: Vec::with_capacity(render::MAX_PARTICLES),

            tx_crate: display.load_tga("assets/sprites/loader/8xcrate.tga"),
            tx_idle:  display.load_tga("assets/sprites/loader/loadertex.tga"),
            tx_fly_w: display.load_tga("assets/sprites/loader/loaderw.tga"),
            tx_fly_a: display.load_tga("assets/sprites/loader/loadera.tga"),
            tx_fly_s: display.load_tga("assets/sprites/loader/loaders.tga"),
            tx_fly_d: display.load_tga("assets/sprites/loader/loaderd.tga"),
            tx_fly_q: display.load_tga("assets/sprites/loader/loaderq.tga"),
            tx_fly_e: display.load_tga("assets/sprites/loader/loadere.tga"),

            // tx_idle:  display.load_tga("assets/sprites/ship/SHIPB001.tga"),
            // tx_fly_w: display.load_tga("assets/sprites/ship/SHIPW001.tga"),
            // tx_fly_a: display.load_tga("assets/sprites/ship/SHIPA001.tga"),
            // tx_fly_s: display.load_tga("assets/sprites/ship/SHIPS001.tga"),
            // tx_fly_d: display.load_tga("assets/sprites/ship/SHIPD001.tga"),
            // tx_fly_q: display.load_tga("assets/sprites/ship/SHIPQ001.tga"),
            // tx_fly_e: display.load_tga("assets/sprites/ship/SHIPE001.tga"),

            draw_tex: None,

        }
    }

    pub fn update(&mut self, controller: &Input, dt: Duration) {
        self.draw_tex = None;
        self.step_particles(dt);

        // handle controller input
             if controller.is_key_held(VKC::W) { self.draw_tex = Some(self.tx_fly_w); self.integrate(dt, Direction::Up)    }
        else if controller.is_key_held(VKC::A) { self.draw_tex = Some(self.tx_fly_a); self.integrate(dt, Direction::Left)  }
        else if controller.is_key_held(VKC::S) { self.draw_tex = Some(self.tx_fly_s); self.integrate(dt, Direction::Down)  }
        else if controller.is_key_held(VKC::D) { self.draw_tex = Some(self.tx_fly_d); self.integrate(dt, Direction::Right) }

             if controller.is_key_held(VKC::Q) { self.draw_tex = Some(self.tx_fly_q); self.rotate(dt, Direction::Left)     }
        else if controller.is_key_held(VKC::E) { self.draw_tex = Some(self.tx_fly_e); self.rotate(dt, Direction::Right)    }

        if controller.was_key_pressed(VKC::Space) {  self.pewpew(); }
    }

    fn step_particles(&mut self, dt: Duration) {
        for particle in &mut self.particles {
            // apply force in direction of heading
            particle.x +=  particle.vx * dt2ms(dt) as f32;
            particle.y +=  particle.vy * dt2ms(dt) as f32;

            let on_x = particle.x > -1.0 && particle.x < 1.0;
            let on_y = particle.y > -1.0 && particle.y < 1.0;
            particle.is_alive = on_x && on_y;
        }

        self.particles.retain(|p| p.is_alive);
    }

    fn pewpew(&mut self) {
        if self.particles.len() == render::MAX_PARTICLES { return }

        let (w,h) = (0.035, 0.035);
        let cx = self.pos.x - (w / 2.0);
        let cy = self.pos.y - (h / 2.0);

        // fire from current heading, no accel time
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        let (vx, vy) = (0.0f32, -BULLET_VMAX); // Q: why is this backwards?
        let rvx = (cos_r * vx) - (sin_r * vy);
        let rvy = (sin_r * vx) + (cos_r * vy);

        self.particles.push(Particle::at_speed(cx, cy, rvx, rvy));
    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // TODO: normalized coords
        let (w,h) = (0.035, 0.035);
        let cx = self.pos.x - (w / 2.0);
        let cy = self.pos.y - (h / 2.0);

        // rotate our sprite space &
        jobs.push(RenderJob::UniformRotate([self.rotation, 0.0]));
        jobs.push(RenderJob::Draw(TexRect::from(self.tx_idle, cx, cy, -0.5, w, h)));

        match self.draw_tex {
            Some(tex_id) => jobs.push(RenderJob::Draw(TexRect::from(tex_id, cx, cy, -0.55, w, h))),
            None => {},
        }
        jobs.push(RenderJob::ResetUniforms);
       
        // draw particles 
        let rects: Vec<Rect> = self.particles.iter()
                                             .map(|p| Rect { x: p.x, y: p.y, z: -0.56, w: w / 2.0, h: h / 2.0 })
                                             .collect();

        if rects.is_empty() { return }
        jobs.push(RenderJob::DrawMany(self.tx_crate, rects));
    }

    pub fn velocity(&self) -> (f32, f32) { (self.vel.x, self.vel.y) }

    fn integrate(&mut self, dt: Duration, dir: Direction) {
        let (ax, ay) = match dir {
            Direction::Up    => (        0.0,  SHIP_ACCEL_Y),
            Direction::Down  => (        0.0, -SHIP_ACCEL_Y),
            Direction::Left  => ( SHIP_ACCEL_X,         0.0),
            Direction::Right => (-SHIP_ACCEL_X,         0.0),
        };

        let (max_x, max_y): (f32,f32) = match dir {
            Direction::Up    => (       0.0,  SHIP_VMAX_Y),
            Direction::Down  => (       0.0, -SHIP_VMAX_Y),
            Direction::Left  => ( SHIP_VMAX_X,        0.0),
            Direction::Right => (-SHIP_VMAX_X,        0.0),
        };

        // perform rotaiton of acceleration vector by hand
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        let rax = (cos_r * ax) - (sin_r * ay);
        let ray = (sin_r * ax) + (cos_r * ay);
        let max_rx = (cos_r * max_x) - (sin_r * max_y);
        let max_ry = (sin_r * max_x) + (cos_r * max_y);

        // apply force in direction of heading
        self.vel = self.vel + V2::at(rax * dt2ms(dt) as f32, ray * dt2ms(dt) as f32);
        
        if rax < 0.0 { self.vel.x = f32::max(self.vel.x, max_rx); } 
        else { self.vel.x = f32::min(self.vel.x, max_rx); }

        if ray < 0.0 { self.vel.y = f32::max(self.vel.y, max_ry); }
        else { self.vel.y = f32::min(self.vel.y, max_ry); }
    }

    fn rotate(&mut self, dt: Duration, dir: Direction) {
        let vr = match dir {
            Direction::Left  => -SHIP_ROT,
            Direction::Right =>  SHIP_ROT,
            _ => panic!("tilemap cannot rotate this direction ..."),
        };

        self.rotation += vr * dt2ms(dt) as f32;
    }
}
