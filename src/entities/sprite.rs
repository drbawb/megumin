use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{RenderJob, RenderGroup};

static SCROLL_V: f32 = 0.001; // (.001px * 1000ms) = 1 texture height / sec.

pub enum Direction { Up, Right, Down, Left }

pub struct Sprite {
    x: f32, y: f32,
    
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
            x: 0.5, y: 0.5,

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
             if controller.is_key_held(VKC::W) { self.draw_tex = Some(self.tx_fly_w); self.integrate(dt, Direction::Up)    }
        else if controller.is_key_held(VKC::A) { self.draw_tex = Some(self.tx_fly_a); self.integrate(dt, Direction::Left)  }
        else if controller.is_key_held(VKC::S) { self.draw_tex = Some(self.tx_fly_s); self.integrate(dt, Direction::Down)  }
        else if controller.is_key_held(VKC::D) { self.draw_tex = Some(self.tx_fly_d); self.integrate(dt, Direction::Right) }
        else if controller.is_key_held(VKC::Q) { self.draw_tex = Some(self.tx_fly_q); }
        else if controller.is_key_held(VKC::E) { self.draw_tex = Some(self.tx_fly_e); }
        else { self.draw_tex = None }
    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // TODO: normalized coords
        let (w,h) = (0.15, 0.15);
        jobs.push(RenderJob::TexRect(self.tx_idle, self.x, self.y, -0.5, w, h));

        match self.draw_tex {
            Some(tex_id) => jobs.push(RenderJob::TexRect(tex_id, self.x, self.y, -0.55, w, h)),
            None => {},
        }
    }

    fn integrate(&mut self, dt: Duration, dir: Direction) {
        let (vx, vy) = match dir {
            Direction::Up    => (      0.0, -SCROLL_V),
            Direction::Down  => (      0.0,  SCROLL_V),
            Direction::Left  => (-SCROLL_V,       0.0),
            Direction::Right => ( SCROLL_V,       0.0),
        };

        // TODO: real vectors ...
        // integrate velocity over time => offset distance
        // self.x += vx * dt2ms(dt) as f32;
        // self.y += vy * dt2ms(dt) as f32;
    }
}
