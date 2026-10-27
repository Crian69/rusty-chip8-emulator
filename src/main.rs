mod chip8;
mod ui;

use macroquad::prelude::*;

#[macroquad::main("CHIP 8")]
async fn main() {
    let mut screen = ui::Screen::new();
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom();
    loop {
        clear_background(WHITE);
        chip8.run(&mut screen);
        screen.paint();
        next_frame().await
    }
}
