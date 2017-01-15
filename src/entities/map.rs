use std::time::Duration;

use rand::{thread_rng, Rng, SeedableRng, XorShiftRng};

use input::Input;
use render::{TexRect, Rect, RenderGroup, RenderJob};
use units::dt2ms;
use units::drawing::RGBA;
use units::linear::V2;

static TILE_SIZE: usize = 256; // GCD(1280,720) = 40
static MAP_SIZE: usize  = 256;

static STAR_BG:   RGBA     = (  0,   0,   0, 255);
static STAR_FG:   RGBA     = (255, 255, 255, 255);
static STAR_SEED: [u32; 4] = [157, 27, 24, 133];

pub struct World {
    player_pos: V2,

    tile_width:  usize,
    tile_height: usize,

    tx_star_bg: usize,
    tx_star_fg: usize,

    entropy:   XorShiftRng,
    starfield: Vec<Rect>,
}

impl World {
    pub fn new(display: &mut RenderGroup, camera_center: V2, tile_size: (usize, usize)) -> World {
        let (tw,th) = (tile_size.0, tile_size.1);

        let fg_bitmap  = vec![vec![STAR_FG; 1]; 1];
        let bg_bitmap  = vec![vec![STAR_BG; 1]; 1];
        let star_fg_id = display.store_texture(fg_bitmap);
        let star_bg_id = display.store_texture(bg_bitmap);

        World {
            player_pos: camera_center,

            tile_width:  tw,
            tile_height: th,

            tx_star_bg: star_bg_id,
            tx_star_fg: star_fg_id,

            entropy:   XorShiftRng::from_seed(STAR_SEED),
            starfield: vec![],
        }
    }

    pub fn update(&mut self, gpu: &mut RenderGroup, pos: V2) {
        self.player_pos = pos;
        self.starfield.clear();

        // convert player pos to tile space
        let sw = 1280.0; let sh = 720.0;
        let ox = (self.player_pos.x - 0.5) * sw;
        let oy = (self.player_pos.y - 0.5) * sh;
        println!("player origin: {},{}", ox,oy);


        // round boundaries to pixels
        // move them into tilespace
        // let left  = ((ox - (sw / 2.0)) / (sw / 2.0)).floor() as i32;
        // let bot   = ((oy - (sh / 2.0)) / (sh / 2.0)).floor() as i32;
        // let right = ((ox + (sw / 2.0)) / (sw / 2.0)).ceil()  as i32;
        // let top   = ((oy + (sh / 2.0)) / (sh / 2.0)).ceil()  as i32;
       
        let half_sw = sw / 2.0;
        let half_sh = sh / 2.0;
        let left  = ((ox - (half_sw * 2.0)) / (sw / 2.0)).floor() as i32;
        let bot   = ((oy - (half_sh * 2.0)) / (sh / 2.0)).floor() as i32;
        let right = ((ox + (half_sw * 2.0)) / (sw / 2.0)).ceil()  as i32;
        let top   = ((ox + (half_sh * 2.0)) / (sh / 2.0)).ceil()  as i32;

        println!("{},{} => {},{}", left,bot, right,top);
        for y in bot..top { // -1 => 1
            for x in left..right { // -1 => 1
                self.entropy.reseed([x as u32, y as u32, 0xDEADBEEF, 0xCAFEBABE]);

                for i in 0..50 {
                    // generate tile relative coord for star
                    let px = self.entropy.gen_range(0,1280);
                    let py = self.entropy.gen_range(0,720);
                    let rel_x = (px as f32) / 1280.0;
                    let rel_y = (py as f32) /  720.0;

                    // generate tile absolute coord in screen space
                    let abs_x = (x as f32) + rel_x;
                    let abs_y = (y as f32) + rel_y;

                    self.starfield.push(Rect {x: abs_x, y: abs_y, z: -0.6, w: 0.00125, h: 0.00125});
                }
            }
        }
    }
    
    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        let (w,h) = (1.0, 1.0);
        let x1 = (self.player_pos.x *  2.0) - 1.0;
        let y1 = (self.player_pos.y * -2.0) + 1.0;

        jobs.push(RenderJob::UniformOffset([x1, y1]));
        jobs.push(RenderJob::Draw(TexRect::from(self.tx_star_bg, 0.0, 0.0, 0.0, w, h)));
        jobs.push(RenderJob::ResetUniforms);

        if !self.starfield.is_empty() {
            jobs.push(RenderJob::UniformTranslate([-x1, y1]));
            jobs.push(RenderJob::DrawMany(self.tx_star_fg, self.starfield.clone()));
            jobs.push(RenderJob::ResetUniforms);
        }
    }
}
