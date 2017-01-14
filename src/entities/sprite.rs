use std::f32::consts as r32;
use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{self, Rect, TexRect, RenderJob, RenderGroup};
use units::{dt2ms, Direction};
use units::linear::V2;

// TODO: how to factor aspect out of here...
static SHIP_ACCEL:  f32  = (128.0 / 1280.0) * 0.001 * 0.001; // px/s^2
static SHIP_VMAX:   f32  = (512.0 / 1280.0) * 0.001;         // px/s
static SHIP_ROT:    f32  = r32::PI * 0.001;                  // rad/s
static BULLET_VMAX: f32  = 0.0007;

pub struct Particle {
    pos: V2, vel: V2,
    pub is_alive: bool,
}

impl Particle {
    pub fn at_speed(x: f32, y: f32, vx: f32, vy: f32) -> Self {

        Particle {
            pos: V2::at(x, y),
            vel: V2::at(vx, vy),

            is_alive: true,
        }
    }
}


pub struct Sprite {
    pos: V2, vel: V2,
    rotation: f32,

    particles: Vec<Particle>,
    rev_ap_engaged: bool,
    rev_ap_active:  bool,
    rev_ap_heading: V2,

    tx_crate: usize,
    tx_idle:  usize,
    tx_fly_w: usize,
    tx_fly_a: usize,
    tx_fly_s: usize,
    tx_fly_d: usize,

    tx_fly_q: usize,
    tx_fly_e: usize,

    engine_tex: Option<usize>,
    thrust_tex: Option<usize>,
}

impl Sprite {
    pub fn new(display: &mut RenderGroup) -> Self {
        Sprite {
            pos: V2::at(0.5, 0.5),
            vel: V2::at(0.0, 0.0),
            rotation: 0.0,

            // misc storage.
            particles: Vec::with_capacity(render::MAX_PARTICLES),
            rev_ap_engaged: false,
            rev_ap_active:  false,
            rev_ap_heading: V2::at(0.0, 1.0),

            // texture storage
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

            // renderer flags
            engine_tex: None,
            thrust_tex: None,

        }
    }

    pub fn update(&mut self, controller: &Input, dt: Duration) {
        self.engine_tex = None;
        self.thrust_tex = None;
        self.step_particles(dt);

        // check if player wants us to auto-invert the heading
        // if so we set the desired heading to our current heading rotated 180deg.
        if !controller.is_key_held(VKC::S) { self.rev_ap_engaged = false; }
        if controller.was_key_pressed(VKC::S) && !self.rev_ap_engaged { 
            self.rev_ap_engaged = true;
            self.rev_ap_active  = true;
            self.rev_ap_heading = self.vel.norm().rot(r32::PI);

            println!("ap {:?} vnorm", self.vel.norm());
            println!("ap {:?} tgt hdg", self.rev_ap_heading);
        }

        if self.rev_ap_active && self.rev_ap_engaged { self.autopilot_reverse(dt); return }

        // otherwise integrate normal movement
             if controller.is_key_held(VKC::W) { self.engine_tex = Some(self.tx_fly_w); self.integrate(dt, Direction::Up)    }
        else if controller.is_key_held(VKC::A) { self.engine_tex = Some(self.tx_fly_a); self.integrate(dt, Direction::Left)  }
        else if controller.is_key_held(VKC::D) { self.engine_tex = Some(self.tx_fly_d); self.integrate(dt, Direction::Right) }

             if controller.is_key_held(VKC::Q) { self.thrust_tex = Some(self.tx_fly_q); self.rotate(dt, Direction::Left)     }
        else if controller.is_key_held(VKC::E) { self.thrust_tex = Some(self.tx_fly_e); self.rotate(dt, Direction::Right)    }

        // fire ze missiles
        if controller.was_key_pressed(VKC::Space) {  self.pewpew(); }
    }

    fn step_particles(&mut self, dt: Duration) {
        for particle in &mut self.particles {
            // apply force in direction of heading
            particle.pos += particle.vel * dt2ms(dt) as f32;
            let on_x = particle.pos.x > -1.0 && particle.pos.x < 1.0;
            let on_y = particle.pos.y > -1.0 && particle.pos.y < 1.0;
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

        // draw our engine & thruster sprites w/ current orientation
        jobs.push(RenderJob::UniformRotate([self.rotation, 0.0]));
        jobs.push(RenderJob::Draw(TexRect::from(self.tx_idle, cx, cy, -0.5, w, h)));
        if let Some(tx) = self.engine_tex { jobs.push(RenderJob::Draw(TexRect::from(tx, cx, cy, -0.54, w, h))) }
        if let Some(tx) = self.thrust_tex { jobs.push(RenderJob::Draw(TexRect::from(tx, cx, cy, -0.55, w, h))) }
        jobs.push(RenderJob::ResetUniforms);
       
        // draw particles 
        let rects: Vec<Rect> = self.particles.iter()
                                             .map(|p| Rect { x: p.pos.x, y: p.pos.y, z: -0.56, w: w / 2.0, h: h / 2.0 })
                                             .collect();

        if rects.is_empty() { return }
        jobs.push(RenderJob::DrawMany(self.tx_crate, rects));
    }

    pub fn velocity(&self) -> (f32, f32) { (self.vel.x, self.vel.y) }

    fn autopilot_reverse(&mut self, dt: Duration) {
        let origin = V2::at(0.0, 1.0);
        let cur  = origin.rot(self.rotation);
        let dest = self.rev_ap_heading;
        if dest.x == 0.0 && dest.y == 0.0 { self.rev_ap_active = false; return }

        // rotate & compare to determine if we stop
        // we discretize this into degrees to avoid FP error
        self.rotate(dt, Direction::Left);

        let rad_to_deg = 180.0 / r32::PI;
        let src_deg = origin.rot(self.rotation).theta() * rad_to_deg;
        let dst_deg = dest.theta() * rad_to_deg;

        let max_diff = 1.0;
        let abs_diff = f32::abs(src_deg.trunc() - dst_deg.trunc());
        if (abs_diff <= max_diff) { self.rev_ap_active = false; }
    }

    fn integrate(&mut self, dt: Duration, dir: Direction) {
        let (ax, ay) = match dir {
            Direction::Up    => (        0.0,  SHIP_ACCEL),
            Direction::Down  => (        0.0, -SHIP_ACCEL),
            Direction::Left  => ( SHIP_ACCEL,         0.0),
            Direction::Right => (-SHIP_ACCEL,         0.0),
        };

        // apply force in direction of heading
        let acc  = V2::at(ax, ay).rot(self.rotation);
        self.vel += acc * (dt2ms(dt) as f32);

        // clamp magnitude of the vector ^^,
        if self.vel.len() > SHIP_VMAX {
            self.vel = self.vel.set_len(SHIP_VMAX);
        }
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
