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

        let rtex = display.store_texture(fill_64_64(&mut buf, (255,0,0,255)));
        let gtex = display.store_texture(fill_64_64(&mut buf, (0,255,0,255)));
        let btex = display.store_texture(fill_64_64(&mut buf, (0,0,255,255)));
        let ytex = display.store_texture(fill_64_64(&mut buf, (255,255,0,255)));

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

fn fill_64_64(buf: &mut Vec<Vec<(u8,u8,u8,u8)>>, color: (u8,u8,u8,u8)) -> Vec<Vec<(u8,u8,u8,u8)>> {
    for row in 0..64 {
        for col in 0..64 { buf[row][col] = color }
    }

    buf.clone()
}
