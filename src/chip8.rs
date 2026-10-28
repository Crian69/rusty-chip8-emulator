use macroquad::math::Vec2;

use crate::ui::Screen;
use std::fs;
pub struct Chip8 {
    memory: [u8; 4096],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
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
            sp: 0x00,
            registers: [0; 16],
        }
    }

    fn not_implemented_instruction(ins: &u16) {
        println!("Instruction Not Implemented: {:X}", ins)
    }

    fn u8_to_bit_array(val: u8) -> [u8; 8] {
        let mut bits = [0u8; 8];
        for i in 0..8 {
            bits[7 - i] = (val >> i) & 1;
        }
        bits
    }

    pub fn load_rom(&mut self) {
        let contents = fs::read("roms/itle.ch8").expect("Rom Not Found");
        let start_index = 0x200;
        let end_index = start_index + contents.len();
        self.memory[start_index..end_index].copy_from_slice(&contents);
    }

    fn get_register_value(&self, r: u8) -> u8 {
        let r = r as usize;
        self.memory[self.registers[r] as usize]
    }

    fn set_register_value(&mut self, r: u8, v: u8) {
        self.registers[r as usize] = v;
    }

    fn set_flag(&mut self) {
        self.set_register_value(15, 1);
    }
    fn reset_flag(&mut self) {
        self.set_register_value(15, 0);
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
                } else if instruction == 0x00EE {
                    self.pc = self.stack[self.sp as usize];
                    self.sp = self.sp - 1;
                } else {
                    // prolly 0NNN
                    Chip8::not_implemented_instruction(&instruction);
                }
            }
            0x01 => {
                self.pc = second_third_fouth_nibble_nnn;
            }
            0x02 => {
                self.sp = self.sp + 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = second_third_fouth_nibble_nnn;
            }
            0x03 => {
                let vx_val = self.get_register_value(second_nibble_x);
                if vx_val == second_byte_nn {
                    self.pc += 2;
                }
            }
            0x04 => {
                let vx_val = self.get_register_value(second_nibble_x);
                if vx_val != second_byte_nn {
                    self.pc += 2;
                }
            }
            0x05 => {
                let vx_val = self.get_register_value(second_nibble_x);
                let vy_val = self.get_register_value(third_nibble_y);
                if vx_val == vy_val {
                    self.pc += 2;
                }
            }
            0x06 => self.registers[second_nibble_x as usize] = second_byte_nn,
            0x07 => {
                let s = second_byte_nn.wrapping_add(self.registers[second_nibble_x as usize]);
                self.registers[second_nibble_x as usize] = s
            }
            0x08 => {
                match fourth_nibble_n {
                    0x00 => {
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y),
                        );
                    }
                    0x01 => {
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y)
                                ^ self.get_register_value(second_nibble_x),
                        );
                    }
                    0x02 => {
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y)
                                & self.get_register_value(second_nibble_x),
                        );
                    }
                    0x03 => {
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y)
                                | self.get_register_value(second_nibble_x),
                        );
                    }
                    0x04 => {
                        let (sum, of) = self
                            .get_register_value(third_nibble_y)
                            .overflowing_add(self.get_register_value(second_nibble_x));
                        self.set_register_value(second_nibble_x, sum);
                        if of {
                            self.set_flag();
                        }
                    }
                    0x05 => {
                        self.set_flag();
                        let (d, of) = self
                            .get_register_value(second_nibble_x)
                            .overflowing_sub(self.get_register_value(third_nibble_y));
                        self.set_register_value(second_nibble_x, d);
                        if of {
                            self.reset_flag();
                        }
                    }
                    0x07 => {
                        self.set_flag();
                        let (d, of) = self
                            .get_register_value(third_nibble_y)
                            .overflowing_sub(self.get_register_value(second_nibble_x));
                        self.set_register_value(second_nibble_x, d);
                        if of {
                            self.reset_flag();
                        }
                    }
                    0x06 => {
                        // TODO in config
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y),
                        );

                        let shifted_bit = self.get_register_value(second_nibble_x) & 0x01;
                        let shifted = self.get_register_value(second_nibble_x) >> 1;
                        self.set_register_value(second_nibble_x, shifted);
                        if shifted_bit != 0 {
                            self.set_flag();
                        } else {
                            self.reset_flag();
                        }
                    }
                    0x0E => {
                        // TODO in config
                        self.set_register_value(
                            second_nibble_x,
                            self.get_register_value(third_nibble_y),
                        );

                        let shifted_bit = (self.get_register_value(second_nibble_x) & 0x80) >> 7;
                        let shifted = self.get_register_value(second_nibble_x) << 1;
                        self.set_register_value(second_nibble_x, shifted);
                        if shifted_bit != 0 {
                            self.set_flag();
                        } else {
                            self.reset_flag();
                        }
                    }
                    _ => {}
                }
            }
            0x09 => {
                let vx_val = self.get_register_value(second_nibble_x);
                let vy_val = self.get_register_value(third_nibble_y);
                if vx_val != vy_val {
                    self.pc += 2;
                }
            }
            0x0A => self.i = second_third_fouth_nibble_nnn,
            0x0B => {
                // todo put in config
                let mut addr: u16;
                if false {
                    addr = second_third_fouth_nibble_nnn
                        + self.get_register_value(second_nibble_x) as u16;
                }
                addr = second_third_fouth_nibble_nnn + self.get_register_value(0) as u16;
                self.pc = addr
            }
            0x0C => {
                let mut r: u8 = rand::random();
                r = r & second_byte_nn;
                self.set_register_value(second_nibble_x, r);
            }
            0x0D => {
                let x = self.registers[second_nibble_x as usize] % 64;
                let mut y = self.registers[third_nibble_y as usize] % 32;
                self.registers[15] = 0; // reset flag

                for s in 0..fourth_nibble_n {
                    let spirit = Self::u8_to_bit_array(self.memory[(self.i + s as u16) as usize]);
                    for (i, b) in spirit.iter().enumerate() {
                        let flipped = screen.flip_pixel_with_pos_ang_get_flipped_off_flag(
                            Vec2::new((x + i as u8) as f32, y as f32),
                            b.to_owned() != 0,
                        );
                        if flipped {
                            self.set_flag();
                        }
                        if x + b > 63 {
                            break;
                        }
                    }
                    y = y + 1;
                    if y > 31 {
                        break;
                    }
                }
            }
            0x0E => {
                Chip8::not_implemented_instruction(&instruction);
            }
            0x0F => match second_byte_nn {
                0x1E => {
                    self.i = self
                        .i
                        .wrapping_add(self.get_register_value(second_nibble_x) as u16);
                }
                _ => {
                    Chip8::not_implemented_instruction(&instruction);
                }
            },
            _ => {
                Chip8::not_implemented_instruction(&instruction);
            }
        }
    }
}
