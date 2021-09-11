use rand::Rng;
use piston_window::*;

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    pub fn new() -> Self {
        Self {
            x: 10,
            y: 10
        }
    }

    pub fn rand_position(&mut self, max_x: u32, max_y: u32) -> () {
        self.x = rand::thread_rng().gen_range(0..max_x);
        self.y = rand::thread_rng().gen_range(0..max_y);
    }

    pub fn draw<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G, w: u32, h: u32) -> () {
        let part: [f64; 4] = [
            self.x as f64 * w as f64,
            self.y as f64 * h as f64,
            w as f64, 
            h as f64,
        ];
        rectangle(
            [0.7, 0.3, 0.3, 1.0],
            part,
            context.transform,
            graphics
        );
    }

    pub fn get_position(&self) -> [u32; 2] {
        [self.x, self.y]
    }
}