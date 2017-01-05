#[macro_use] extern crate glium;
extern crate rusttype;

#[allow(dead_code)] mod input;
#[allow(dead_code)] mod units;
mod render;

use std::time::{Duration, Instant};
use std::thread;

// TODO: move window construction to render module?
use glium::DisplayBuild;
use glium::glutin::{Event, ElementState, WindowBuilder};
use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{RenderGroup, RenderJob};

static TARGET_FPS_MS: u64 = 1000 / 120;

enum Direction { Up, Right, Down, Left }

fn dt2ms(dt: Duration) -> u64 {
    (dt.as_secs() * 1000) + (dt.subsec_nanos() as u64 / 1_000_000)
}

struct ScrollyBox { ofs: [f32; 2] }
impl ScrollyBox {
    pub fn new() -> Self {
        ScrollyBox { ofs: [0.0, 0.0] }
    }

    pub fn update(&mut self, dt: Duration, dir: Direction) {
        let scroll_v = 0.001; // (.001px * 1000ms) = 1 texture height / sec.
        let (vx, vy) = match dir {
            Direction::Up    => (      0.0,  scroll_v),
            Direction::Down  => (      0.0, -scroll_v),
            Direction::Left  => (-scroll_v,       0.0),
            Direction::Right => ( scroll_v,       0.0),
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


fn main() {
    // setup hardware
    println!("initializing display ...");
    let display = WindowBuilder::new()
                                .with_depth_buffer(24)
                                .build_glium()
                                .expect("could not open window");

    let draw_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },

        .. Default::default()
    };


    // TODO: engine state block
    let mut renderer = RenderGroup::new(&display, &draw_params);
    let mut controller =  Input::new();
    let mut render_jobs = vec![];
    let mut block = ScrollyBox::new();

    // game clock
    let target_fps = Duration::from_millis(TARGET_FPS_MS);
    let mut frame_start = Instant::now();

    println!("starting game loop ...");
    'runloop: loop {
        // top of frame
        let frame_dt = Instant::now() - frame_start;
        frame_start = Instant::now();
        controller.begin_new_frame();
        render_jobs.clear();

        // handle input
        for ev in display.poll_events() {
            match ev {
                Event::Closed => break 'runloop,

                // keyboard
                Event::KeyboardInput(ElementState::Pressed,  _code, Some(cap)) => controller.key_down_event(cap),
                Event::KeyboardInput(ElementState::Released, _code, Some(cap)) => controller.key_up_event(cap),
                Event::KeyboardInput(_, code, None) => println!("uknown key code: {}", code),

                // mouse (x grows right, y grows downward)
                Event::MouseMoved(mx, my) => controller.move_cursor(mx, my),
                _ => (),
            }
        }

        // exit immediately on escape
        if controller.was_key_pressed(VKC::Escape) { break 'runloop }

        // scroll square interior
        if controller.is_key_held(VKC::Up)    { block.update(frame_dt, Direction::Up)    }
        if controller.is_key_held(VKC::Right) { block.update(frame_dt, Direction::Right) }
        if controller.is_key_held(VKC::Down)  { block.update(frame_dt, Direction::Down)  }
        if controller.is_key_held(VKC::Left)  { block.update(frame_dt, Direction::Left)  }

        // prepare render queue
        render_jobs.push(RenderJob::ClearScreen(0.0, 0.0, 0.0, 1.0));
        render_jobs.push(RenderJob::ClearDepth(1.0));
        block.draw(&mut render_jobs);

        // draw queue to back buffer
        let mut frame = display.draw();
        renderer.draw(&render_jobs[..], &mut frame);
        frame.finish().unwrap();

        // handle timing
        let dt = (Instant::now()).duration_since(frame_start);
        if dt > target_fps { println!("missed frame {:?}", dt); continue }
        let draw_time = target_fps - dt;
        thread::sleep(draw_time);
    }

    println!("goodbye ...");
}
