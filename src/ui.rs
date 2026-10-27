use macroquad::{color, math::Vec2, shapes::draw_rectangle};

const SCALING_FACTOR: u32 = 10;
const HEIGHT: u32 = 32;
const WIDTH: u32 = 64;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub on: bool,
    pub pos: Vec2,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            on: false,
            pos: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}

pub struct Screen {
    pub pixels: [Pixel; 2048],
}

impl Screen {
    pub fn new() -> Self {
        let mut pixels = [Pixel::default(); (HEIGHT * WIDTH) as usize];
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                let p = Pixel {
                    on: false,
                    pos: Vec2 {
                        x: c as f32,
                        y: r as f32,
                    },
                };
                pixels[(r * WIDTH + c) as usize] = p;
            }
        }
        Self { pixels }
    }

    pub fn paint(&self) {
        for pixel in self.pixels {
            if pixel.on {
                draw_rectangle(
                    pixel.pos.x * (SCALING_FACTOR) as f32,
                    pixel.pos.y * (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    color::WHITE,
                );
            } else {
                draw_rectangle(
                    pixel.pos.x * (SCALING_FACTOR) as f32,
                    pixel.pos.y * (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    color::BLACK,
                );
            }
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixels.iter_mut() {
            p.on = false;
        }
    }

    pub fn flip_pixel_with_pos_ang_get_flipped_off_flag(&mut self, pos: Vec2, state: bool) -> bool {
        if self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on && state {
            self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on = false;
            return true;
        }
        if !self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on {
            self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on = true;
            return false;
        }
        self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on = false;
        return false;
    }
}
