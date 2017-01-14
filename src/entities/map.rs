use std::time::Duration;

use rand::{thread_rng, Rng};

use input::Input;
use render::{TexRect, RenderGroup, RenderJob};
use units::dt2ms;
use units::drawing::RGBA;
use units::linear::V2;

static MAP_SIZE: usize = 256;
static STAR_FG: RGBA = (255, 255, 255, 255);
static STAR_BG: RGBA = (  0,   0,   0, 255);

struct Starfield {
    star_buf_id:  usize,
    star_buf_mem: Vec<Vec<RGBA>>,
}

impl Starfield {
    fn new(display: &mut RenderGroup, tw: usize, th: usize) -> Starfield {
        let mut star_buf_cpu = vec![vec![STAR_BG; tw]; th];

        let mut rng = thread_rng();
        for si in 0..100 {
            let x = rng.gen_range(0, tw);
            let y = rng.gen_range(0, th);
            star_buf_cpu[y][x] = STAR_FG;
        }

        let star_buf_gpu = display.store_texture(star_buf_cpu.clone());

        Starfield {
            star_buf_id:  star_buf_gpu,
            star_buf_mem: star_buf_cpu,
        }
    }
}

pub struct World {
    player_pos: V2,

    tile_width:  usize,
    tile_height: usize,

    skybox: Vec<Starfield>,
}

impl World {
    pub fn new(display: &mut RenderGroup, camera_center: V2, tile_size: (usize, usize)) -> World {
        let (tw,th) = (tile_size.0, tile_size.1);
        let mut skies = Vec::with_capacity(9);
        for i in 0..9 {
            skies.push(Starfield::new(display, tw, th));
        }

        World {
            player_pos: camera_center,

            tile_width:  tw,
            tile_height: th,

            skybox: skies,
        }
    }

    pub fn update(&mut self, mut pos: V2) {
        ::std::mem::swap(&mut pos, &mut self.player_pos);

        // take integer portion as seed
        let ox = pos.x.trunc();
        let oy = pos.y.trunc();
        let tx = self.player_pos.x.trunc();
        let ty = self.player_pos.y.trunc();

        if tx > ox { // walked right
            println!("right {} -> {}", ox, tx);
        } else if tx < ox { // walked left
            println!("left {} -> {}", ox, tx);
        }

        if ty > oy { // walked up
            println!("up");
        } else if ty < oy { // walked down
            println!("down");
        }

    }
    
    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        let (w,h) = (1.0, 1.0);
        let x1 = (self.player_pos.x *  2.0) - 1.0;
        let y1 = (self.player_pos.y * -2.0) + 1.0;
        let center = self.skybox[4].star_buf_id;

        jobs.push(RenderJob::UniformOffset([x1, y1]));
        jobs.push(RenderJob::Draw(TexRect::from(center, 0.0, 0.0, 0.0, w, h)));
        jobs.push(RenderJob::ResetUniforms);
    }
}
