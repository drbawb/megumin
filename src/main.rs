#[macro_use] extern crate glium;
extern crate image;
extern crate rand;
extern crate rusttype;

#[allow(dead_code)] mod input;
#[allow(dead_code)] mod units;
mod entities;
mod render;

use std::time::{Duration, Instant};
use std::thread;

// TODO: move window construction to render module?
use glium::DisplayBuild;
use glium::glutin::{Event, ElementState, VirtualKeyCode as VKC, WindowBuilder};

use input::Input;
use render::{RenderGroup, RenderJob};

static TARGET_FPS_MS: u64 = 1000 / 120;

fn main() {
    // setup hardware
    println!("initializing display ...");
    let display = WindowBuilder::new()
                                .with_depth_buffer(24)
                                .with_title("megumin")
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
    let mut controller  =  Input::new();
    let mut renderer    = RenderGroup::new(&display, &draw_params);
    let mut render_jobs = vec![];
   
    // TODO: some sort of entity buffer
    let mut map    = entities::TileMap::new(&mut renderer);
    let mut player = entities::Sprite::new(&mut renderer);

    // the runloop is a fairly straightforward game loop, it spends time performing
    // three major functions:
    //
    // - buffering input
    // - integrating entities over time (elapsed since last frame)
    // - rendering active entities
    //
    // at the top of each frame we compute the time elapsed since the last
    // iteration of the runloop. if the game is running smoothly this should
    // be approximately `TARGET_FPS_MS` milliseconds
    //
    // input is buffered into a series of tables. these tables are optionally
    // used by entities to determine their behavior for the next simulation step.
    //
    // this delta is used to drive the simulation step at a constant rate in
    // terms of milliseconds elapsed. this value will be higher if the runloop
    // is running behind. physics should therefore be in a coherent state wrt
    // the computer's real-time clock.
    //
    // after each active entitiy has been simulated we begin rendering the
    // world. this is done by grabbing the backbuffer, clearing it, and
    // allowing each entity to mutate the render queue serially.
    //
    // there is probably potential for threading & perf wins here, not sure.
    // (possibly need to designate "layers" (fg/mg/bg) as sync points
    //  thus sprites would be drawn in a potentially consistent order.)
    //
    //  the renderer is then instructed to commit the render queue to the
    //  backbuffer. the details of this are a mystery.
    //
    // TODO: cap the `frame_dt` to a fixed timestep to allow for easier
    // debugging, etc.
    //
    // TODO: display building config. (vsync, resolution?, windowed?, etc.)
    //

    // game clock
    let target_fps      = Duration::from_millis(TARGET_FPS_MS);
    let mut frame_start = Instant::now();

    println!("starting game loop ...");
    'runloop: loop {
        // top of frame
        let frame_dt = Instant::now() - frame_start;
        frame_start = Instant::now();
        controller.begin_new_frame();
        render_jobs.clear();

        // store frame inputs in buffer
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

        // process input buffer
        if controller.was_key_pressed(VKC::Escape) { break 'runloop }
        player.update(&controller, frame_dt);
        map.update(&controller, frame_dt, player.velocity());

        // TODO: use depth buffer instead of relying on draw order
        // prepare render queue
        render_jobs.push(RenderJob::ClearScreen(0.0, 0.0, 0.0, 1.0));
        render_jobs.push(RenderJob::ClearDepth(1.0));
        map.draw(&mut render_jobs);
        player.draw(&mut render_jobs);

        // draw queue to back buffer
        let mut frame = display.draw();
        renderer.draw(&render_jobs[..], &mut frame);
        frame.finish().unwrap();

        // handle frame timing
        let dt = (Instant::now()).duration_since(frame_start);
        if dt > target_fps { println!("missed frame {:?}", dt); continue }
        let draw_time = target_fps - dt;
        thread::sleep(draw_time);
    }

    println!("goodbye ...");
}
