use macroquad::math::Vec2;

use crate::ui::Screen;
use std::fs;
pub struct Chip8 {
    memory: [u8; 4096],
    i: u16,
    pc: u16,
    stack: [u8; 16],
    dt: u8,
    st: u8,
    registers: [u8; 16],
}

const INSTRUCTIONS_PER_CYCLE: u16 = 700;

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0x00; 4096],
            i: 0x00,
            pc: 0x200,
            stack: [0x00; 16],
            dt: 0x00,
            st: 0x00,
            registers: [0; 16],
        }
    }

    fn not_implemented_instruction(ins: &u16) {
        println!("Instruction Not Implemented: {:X}", ins)
    }

    fn u8_to_bit_array(val: u8) -> [u8; 8] {
        let mut bits = [0u8; 8];
        for i in 0..8 {
            // Shift right and extract the least significant bit
            bits[7 - i] = (val >> i) & 1;
        }
        bits
    }

    pub fn load_rom(&mut self) {
        let contents = fs::read("roms/logo.ch8").expect("Rom Not Found");
        let start_index = 0x200;
        let end_index = start_index + contents.len();
        // Copy the contents safely into your memory array
        self.memory[start_index..end_index].copy_from_slice(&contents);
    }

    pub fn run(&mut self, screen: &mut Screen) {
        // fetch
        let pc_plus_1 = self.pc + 1;
        let instruction_first_byte = self.memory[self.pc as usize];
        let instruction_second_byte = self.memory[pc_plus_1 as usize];

        self.pc = self.pc + 2;

        // decode
        let instruction = u16::from_be_bytes([instruction_first_byte, instruction_second_byte]);
        let first_nibble = instruction_first_byte >> 4;
        let second_nibble_x = instruction_first_byte & 0x0F; // always look up the register
        let third_nibble_y = instruction_second_byte >> 4; // always look up the register
        let fourth_nibble_n = instruction_second_byte & 0x0F;
        let second_byte_nn = instruction_second_byte; // for the sake of readability
        let second_third_fouth_nibble_nnn = instruction & 0x0FFF;

        // execute
        match first_nibble {
            0x00 => {
                if instruction == 0x00E0 {
                    screen.clear();
                } else {
                    Chip8::not_implemented_instruction(&instruction);
                }
            }
            0x01 => {
                self.pc = second_third_fouth_nibble_nnn;
            }
            0x02 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x03 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x04 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x05 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x06 => self.registers[second_nibble_x as usize] = second_byte_nn,
            0x07 => {
                self.registers[second_nibble_x as usize] =
                    second_byte_nn + self.registers[second_nibble_x as usize]
            }
            0x08 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x09 => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x0A => self.i = second_third_fouth_nibble_nnn,
            0x0B => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x0C => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x0D => {
                let x = self.registers[second_nibble_x as usize] % 64;
                let mut y = self.registers[third_nibble_y as usize] % 32;
                self.registers[15] = 0; // reset flag
                let mut flipped = false;

                for s in 0..fourth_nibble_n {
                    let spirit = Self::u8_to_bit_array(self.memory[(self.i + s as u16) as usize]);
                    for (i,b) in spirit.iter().enumerate() {
                        flipped = screen.flip_pixel_with_pos_ang_get_flipped_off_flag(
                            Vec2::new((x + i as u8) as f32, y as f32),
                            b.to_owned() != 0,
                        );
                        if x + b > 63 {
                            break;
                        }
                    }
                    y = y + 1;
                    if y > 31 {
                        break;
                    }
                }
                if flipped {
                    self.registers[15] = 1
                }
            }
            0x0E => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x0F => {
                Chip8::not_implemented_instruction(&instruction);
            }
            _ => {
                Chip8::not_implemented_instruction(&instruction);
            }
        }
    }
}
