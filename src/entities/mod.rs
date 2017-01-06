pub use self::scrolly_box::ScrollyBox;
pub mod scrolly_box;

use render::{RenderGroup, RenderJob};

pub struct TileMap {
    rtex: usize,
    gtex: usize,
    btex: usize,
    ytex: usize,
}

impl TileMap {
    pub fn new(display: &mut RenderGroup) -> Self {
        let mut buf = vec![vec![(0,0,0,0); 64]; 64];

        let rtex = {
            for row in 0..64 {
                for col in 0..64 { buf[row][col] = (255,0,0,255) }
            }
            
            display.store_texture(buf.clone())
        };

        let gtex = {
            for row in 0..64 {
                for col in 0..64 { buf[row][col] = (0,255,0,255) }
            }
            
            display.store_texture(buf.clone())
        };

        let btex = {
            for row in 0..64 {
                for col in 0..64 { buf[row][col] = (0,0,255,255) }
            }
            
            display.store_texture(buf.clone())
        };

        let ytex = {
            for row in 0..64 {
                for col in 0..64 { buf[row][col] = (255,255,0,255) }
            }
            
            display.store_texture(buf.clone())
        };

        TileMap { rtex: rtex, gtex: gtex, btex: btex, ytex: ytex }
    }

    pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // TODO: normalized coords
        let (w,h) = (0.5, 0.5);
        jobs.push(RenderJob::TexRect(self.rtex, 0.0, 0.0, w, h));
        jobs.push(RenderJob::TexRect(self.gtex, 0.0, 0.5, w, h));
        jobs.push(RenderJob::TexRect(self.btex, 0.5, 0.0, w, h));
        jobs.push(RenderJob::TexRect(self.ytex, 0.5, 0.5, w, h));
    }
}