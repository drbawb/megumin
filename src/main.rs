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
    println!("initializing display ...");
    let display = WindowBuilder::new()
                                .build_glium()
                                .expect("could not open window");

    // TODO: engine state block
    let mut controller =  Input::new();
    let mut frame_time = Duration::from_millis(1000 / 60); // 60FPS
    let mut square = Square::new(&display);

    println!("starting game loop ...");
    'runloop: loop {
        // handle input
        controller.begin_new_frame();
        for ev in display.poll_events() {
            match ev {
                Event::Closed => break 'runloop,

                Event::KeyboardInput(ElementState::Pressed, code, Some(cap))  => controller.key_down_event(cap),
                Event::KeyboardInput(ElementState::Released, code, Some(cap)) => controller.key_up_event(cap),
                Event::KeyboardInput(_, code, None) => panic!("uknown key code: {}", code),
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
        thread::sleep(frame_time);
    }

    println!("goodbye ...");
}
