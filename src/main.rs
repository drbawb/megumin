#[macro_use] extern crate glium;
extern crate rusttype;

#[allow(dead_code)] mod input;
#[allow(dead_code)] mod units;
mod square; // TODO: dev mesh

use std::time::{Duration, Instant};
use std::thread;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState, WindowBuilder};
use glium::glutin::VirtualKeyCode as VKC;

use input::Input;
use square::Square;

fn main() {
    // setup hardware
    println!("initializing display ...");
    let display = WindowBuilder::new()
                                .build_glium()
                                .expect("could not open window");

    // TODO: engine state block
    let mut controller =  Input::new();
    let mut square = Square::new(&display);

    // game clock
    let target_fps = Duration::from_millis(1000 / 60);
    let mut frame_clock = Instant::now();
    let mut frame_start = frame_clock;

    println!("starting game loop ...");
    'runloop: loop {
        frame_start = Instant::now();

        // handle input
        controller.begin_new_frame();
        for ev in display.poll_events() {
            match ev {
                Event::Closed => break 'runloop,

                Event::KeyboardInput(ElementState::Pressed,  _code, Some(cap)) => controller.key_down_event(cap),
                Event::KeyboardInput(ElementState::Released, _code, Some(cap)) => controller.key_up_event(cap),
                Event::KeyboardInput(_, code, None) => println!("uknown key code: {}", code),
                _ => (),
            }
        }

        if controller.was_key_pressed(VKC::Escape) { break 'runloop }
        if controller.is_key_held(VKC::Up)   { square.up();   }
        if controller.is_key_held(VKC::Down) { square.down(); }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        square.draw(&mut frame);
        frame.finish().expect("could not close frame");

        let dt = (Instant::now()).duration_since(frame_start);
        if dt > target_fps { println!("missed frame"); continue }

        let draw_time = target_fps - dt;
        thread::sleep(draw_time);
    }

    println!("goodbye ...");
}

// fn dt_to_millis(duration: Duration) -> u64 {
//     let secs  = duration.as_secs();
//     let nanos = duration.subsec_nanos();
// 
//     (secs * 1000) + (nanos as u64 / 1_000_000)
// }
