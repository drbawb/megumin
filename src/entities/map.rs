use std::cell::RefCell;
use std::rc::Rc;

use rand::{Rng, SeedableRng, XorShiftRng};

use render::{Rect, RenderGroup, RenderJob};
use units::drawing::RGBA;
use units::linear::V2;

static STAR_BG:   RGBA     = (  0,   0,   0, 255);
static STAR_FG:   RGBA     = (255, 255, 255, 255);
static STAR_SEED: [u32; 4] = [157, 27, 24, 133];

pub struct World {
    player_pos: V2,

    tx_star_bg: usize,
    tx_star_fg: usize,

    entropy:   XorShiftRng,
    starfield: Rc<RefCell<Vec<Rect>>>,
}

impl World {
    pub fn new(display: &mut RenderGroup, camera_center: V2) -> World {
        let fg_bitmap  = vec![vec![STAR_FG; 1]; 1];
        let bg_bitmap  = vec![vec![STAR_BG; 1]; 1];
        let star_fg_id = display.store_texture(fg_bitmap);
        let star_bg_id = display.store_texture(bg_bitmap);

        World {
            player_pos: camera_center,

            tx_star_bg: star_bg_id,
            tx_star_fg: star_fg_id,

            // TODO: magic number for capacity
            // center + cardinal directions + diagonals = 9 screens
            // 50 stars per screen, 1 rect per star...
            entropy:   XorShiftRng::from_seed(STAR_SEED),
            starfield: Rc::new(RefCell::new(Vec::with_capacity(3 * 3 * 50))),
        }
    }

    pub fn update(&mut self, pos: V2) {
        let mut starfield = self.starfield.borrow_mut();
        self.player_pos = pos;
        starfield.clear();

        // convert player pos to tile space
        let ox = self.player_pos.x - 0.5;
        let oy = self.player_pos.y - 0.5;

        // figure out visible screen boundaries
        let left  = (ox - 1.0).floor() as i32;
        let bot   = (oy - 1.0).floor() as i32;
        let right = (ox + 1.0).ceil()  as i32;
        let top   = (oy + 1.0).ceil()  as i32;

        for y in bot..top { // -1 => 1
            for x in left..right { // -1 => 1
                self.entropy.reseed([x as u32, y as u32, 0xDEADBEEF, 0xCAFEBABE]);

                for _star in 0..50 {
                    // generate tile relative coord for star
                    let px = self.entropy.gen_range(0,1280);
                    let py = self.entropy.gen_range(0,720);
                    let rel_x = (px as f32) / 1280.0;
                    let rel_y = (py as f32) /  720.0;

                    // generate tile absolute coord in screen space
                    let abs_x = (x as f32) + rel_x;
                    let abs_y = (y as f32) + rel_y;

                    starfield.push(Rect {x: abs_x, y: abs_y, z: -0.6, w:  1.0 / 1280.0, h: 1.0 / 720.0});
                }
            }
        }

    }
    
    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        let x1 = (self.player_pos.x * 2.0) - 1.0;
        let y1 = (self.player_pos.y * 2.0) - 1.0;

        // TODO: there is something fucky about coordinates
        // why are these inverted? why is abs_y in the integration inverted?
        // nobody knows, but it doesn't work any other way
        if !self.starfield.borrow().is_empty() {
            jobs.push(RenderJob::UniformTranslate([-x1, -y1]));
            jobs.push(RenderJob::DrawMany(self.tx_star_fg, self.starfield.clone()));
            jobs.push(RenderJob::ResetUniforms);
        }
    }
}
