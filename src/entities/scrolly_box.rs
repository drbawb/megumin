use std::time::Duration;

use glium::glutin::VirtualKeyCode as VKC;

use units::dt2ms;
use input::Input;
use render::{self, RenderJob};


static SCROLL_V: f32 = 0.001; // (.001px * 1000ms) = 1 texture height / sec.

pub enum Direction { Up, Right, Down, Left }

pub struct ScrollyBox { ofs: [f32; 2] }

impl ScrollyBox {
    pub fn new() -> Self {
        ScrollyBox { ofs: [0.0, 0.0] }
    }

    pub fn update(&mut self, controller: &Input, dt: Duration) {

        if controller.is_key_held(VKC::Up)    { self.integrate(dt, Direction::Up)    }
        if controller.is_key_held(VKC::Right) { self.integrate(dt, Direction::Right) }
        if controller.is_key_held(VKC::Down)  { self.integrate(dt, Direction::Down)  }
        if controller.is_key_held(VKC::Left)  { self.integrate(dt, Direction::Left)  }
    }

    fn integrate(&mut self, dt: Duration, dir: Direction) {

        let (vx, vy) = match dir {
            Direction::Up    => (      0.0,  SCROLL_V),
            Direction::Down  => (      0.0, -SCROLL_V),
            Direction::Left  => (-SCROLL_V,       0.0),
            Direction::Right => ( SCROLL_V,       0.0),
        };

        // TODO: real vectors ...
        // integrate velocity over time => offset distance
        self.ofs[0] += vx * dt2ms(dt) as f32;
        self.ofs[1] += vy * dt2ms(dt) as f32;

    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        jobs.push(RenderJob::UniformOffset(self.ofs));
        jobs.push(RenderJob::DrawRect(render::Rect { x: -256, y: -256, w: 512, h: 512 }));
    }
}
