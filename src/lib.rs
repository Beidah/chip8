mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_SIZE: usize = 16;
const KEYBOARD_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;

#[wasm_bindgen]
pub struct Screen {
    width: usize,
    height: usize,
    pixels: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

#[wasm_bindgen]
impl Screen {
    pub fn new() -> Screen {
        let width = SCREEN_WIDTH;
        let height = SCREEN_HEIGHT;

        let pixels = [false; SCREEN_WIDTH * SCREEN_HEIGHT];

        Screen {
            width,
            height,
            pixels,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[y * self.width + x] = value;
    }

    fn clear(&mut self) {
        self.pixels = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> *const bool {
        self.pixels.as_ptr()
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F]
];

#[wasm_bindgen]
pub struct Emu {
    pc: u16,
    memory: [u8; RAM_SIZE],
    screen: Screen,
    registers: [u8; REGISTER_SIZE],
    i_register: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; KEYBOARD_SIZE],
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Emu {
    pub fn new() -> Emu {
        utils::set_panic_hook();
        let mut new_emu = Emu {
            screen: Screen::new(),
            memory: [0; RAM_SIZE],
            pc: START_ADDR,
            i_register: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; REGISTER_SIZE],
            keys: [false; KEYBOARD_SIZE],
            sp: 0,
        };

        new_emu.memory[..FONTS.len()].copy_from_slice(&FONTS);

        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.memory = [0; RAM_SIZE];
        self.memory[..FONTS.len()].copy_from_slice(&FONTS);
        self.screen.clear();
        self.registers = [0; REGISTER_SIZE];
        self.i_register = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; KEYBOARD_SIZE];
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    pub fn load(&mut self, program: &[u8]) {
        let start = START_ADDR as usize;
        let end = start + program.len();
        self.memory[start..end].copy_from_slice(program);
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            // if self.sound_timer == 1 {
            //     // BEEP?
            // }
            self.sound_timer -= 1;
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);
        self.pc += 2;
        opcode
    }

    pub fn execute(&mut self, opcode: u16) {
        // log!("Executing opcode: {opcode:x}");

        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // Clear Screen
            (0, 0, 0xE, 0) => self.screen.clear(),
            // Return
            (0, 0, 0xE, 0xE) => {
                let addr = self.pop();
                self.pc = addr;
            }
            // Jump NNN
            (1, _, _, _) => {
                let nnn = opcode & 0x0FFF;
                self.pc = nnn;
            }
            // Call NNN
            (2, _, _, _) => {
                let nnn = opcode & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            // Jump VX NNN EQ
            (3, x, _, _) => {
                let nn = (opcode & 0xFF) as u8;
                if self.registers[x as usize] == nn {
                    self.pc += 2;
                }
            }
            // Jump VX NNN Not EQ
            (4, x, _, _) => {
                let nn = (opcode & 0xFF) as u8;
                if self.registers[x as usize] != nn {
                    self.pc += 2;
                }
            }
            // Jump VX VY EQ
            (5, x, y, 0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            (6, x, _, _) => {
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = nn;
            }
            (7, x, _, _) => {
                let n = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(n);
            }
            (9, x, y, 0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                let nnn = opcode & 0x0FFF;
                self.i_register = nnn;
            }
            (0xD, x, y, n) => {
                let x_coord = self.registers[x as usize] as u16;
                let y_coord = self.registers[y as usize] as u16;

                let mut flipped = false;

                for height in 0..n {
                    let addr = self.i_register + height;
                    let pixels = self.memory[addr as usize];
                    for width in 0..8 {
                        if (pixels & (0b1000_0000 >> width)) != 0 {
                            let x = (x_coord + width) as usize % SCREEN_WIDTH;
                            let y = (y_coord + height) as usize % SCREEN_HEIGHT;

                            let index = self.screen.get_index(x, y);

                            flipped |= self.screen.pixels[index];
                            self.screen.set_pixel(x, y, true);
                        }
                    }
                }

                if flipped {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            (_, _, _, _) => {
                self.pc -= 2;
                log!("Unimplemented opcode {:X}", opcode);
                // unimplemented!("Unimplemented opcode {:X}", opcode)
            }
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn width(&self) -> usize {
        self.screen.width()
    }

    pub fn height(&self) -> usize {
        self.screen.height()
    }

    pub fn pixels(&self) -> *const bool {
        self.screen.pixels()
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
