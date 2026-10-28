use std::collections::HashMap;

use macroquad::{
    color::{self, BLACK, WHITE},
    input::KeyCode,
    math::Vec2,
    shapes::draw_rectangle,
    text::draw_text,
};

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
            let x = pixel.pos.x * (SCALING_FACTOR) as f32;
            let y = pixel.pos.y * (SCALING_FACTOR) as f32;
            if pixel.on {
                draw_rectangle(
                    x,
                    y,
                    (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    color::WHITE,
                );
            } else {
                draw_rectangle(
                    x,
                    y,
                    (SCALING_FACTOR) as f32,
                    (SCALING_FACTOR) as f32,
                    color::BLACK,
                );
            }
        }

        // TODO: Put this in debug flag
        // let font_size = 12.0;
        // for pixel in &self.pixels {
        //     let x = pixel.pos.x;
        //     let y = pixel.pos.y;
        //     let sx = pixel.pos.x * (SCALING_FACTOR) as f32;
        //     let sy = pixel.pos.y * (SCALING_FACTOR) as f32;
        //     let (text, color) = if pixel.on {
        //         (format!("{x}|{y}"), BLACK)
        //     } else {
        //         (format!("{x}|{y}"), WHITE)
        //     };
        //     draw_text(&text, sx, sy + font_size, font_size, color);
        // }
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
        if !self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on && state {
            self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on = true;
            return false;
        }
        self.pixels[(pos.y * WIDTH as f32 + pos.x) as usize].on = false;
        return false;
    }
}

pub fn map_actual_key_to_chip_key(k: KeyCode) -> Option<u8> {
    let mut char_map: HashMap<KeyCode, u8> = HashMap::new();
    char_map.insert(KeyCode::Key1, 0x01);
    char_map.insert(KeyCode::Key2, 0x02);
    char_map.insert(KeyCode::Key3, 0x03);
    char_map.insert(KeyCode::Key4, 0x0C);
    char_map.insert(KeyCode::Q, 0x04);
    char_map.insert(KeyCode::W, 0x05);
    char_map.insert(KeyCode::E, 0x06);
    char_map.insert(KeyCode::R, 0x0D);
    char_map.insert(KeyCode::A, 0x07);
    char_map.insert(KeyCode::S, 0x08);
    char_map.insert(KeyCode::D, 0x09);
    char_map.insert(KeyCode::F, 0x0E);
    char_map.insert(KeyCode::Z, 0x0A);
    char_map.insert(KeyCode::Z, 0x00);
    char_map.insert(KeyCode::C, 0x0B);
    char_map.insert(KeyCode::V, 0x0F);

    if let Some(k) = char_map.get(&k) {
        return Some(k.to_owned());
    }

    return None;
}
