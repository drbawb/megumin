#[macro_use] extern crate glium;
extern crate rusttype;

#[allow(dead_code)] mod input;
#[allow(dead_code)] mod units;
mod render;

use std::time::{Duration, Instant};
use std::thread;

use glium::DisplayBuild;
use glium::glutin::{Event, ElementState, WindowBuilder};
use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use render::{RenderGroup, RenderJob};

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

    let mut renderer = RenderGroup::new(&display, &draw_params);

    // TODO: engine state block
    let mut controller =  Input::new();
    let mut render_jobs = vec![];

    // game clock
    let target_fps = Duration::from_millis(1000 / 60);
    let mut frame_start;

    println!("starting game loop ...");
    'runloop: loop {
        // top of frame
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

        // handle escape key
        if controller.was_key_pressed(VKC::Escape) { break 'runloop }

        // scroll square interior
        // if controller.is_key_held(VKC::Up)    { square.up();    }
        // if controller.is_key_held(VKC::Right) { square.right(); }
        // if controller.is_key_held(VKC::Down)  { square.down();  }
        // if controller.is_key_held(VKC::Left)  { square.left();  }

        // draw the square to the rear framebuffer
        render_jobs.push(RenderJob::ClearDepth(1.0));
        render_jobs.push(RenderJob::ClearScreen(0.0, 0.0, 0.0, 1.0));
        render_jobs.push(RenderJob::DrawRect(render::Rect { x: 0, y: 0, w: 512, h: 512 }));
        renderer.draw(&render_jobs[..]);

        let dt = (Instant::now()).duration_since(frame_start);
        if dt > target_fps { println!("missed frame"); continue }

        let draw_time = target_fps - dt;
        thread::sleep(draw_time);
    }

    println!("goodbye ...");
}
