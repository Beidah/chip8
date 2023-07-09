mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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

pub struct Screen {
    width: usize,
    height: usize,
    pixels: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

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

    pub fn pixels(&self) -> &[bool] {
        &self.pixels
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
pub struct Chip8 {
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
impl Chip8 {
    pub fn new() -> Chip8 {
        utils::set_panic_hook();
        let mut new_emu = Chip8 {
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
            // SET VX
            (6, x, _, _) => {
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = nn;
            }
            // ADD VX
            (7, x, _, _) => {
                let n = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(n);
            }
            // SET VX VY
            (8, x, y, 0) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            // OR VX VY
            (8, x, y, 1) => {
                self.registers[x as usize] |= self.registers[y as usize];
            }
            // AND VX VY
            (8, x, y, 2) => {
                self.registers[x as usize] &= self.registers[y as usize];
            }
            // XOR VX VY
            (8, x, y, 3) => {
                self.registers[x as usize] ^= self.registers[y as usize];
            }
            // ADD VX VY
            (8, x, y, 4) => {
                let (vx, carry) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = vx;

                self.registers[0xF] = if carry { 1 } else { 0 }
            }
            // SUB VX VY
            (8, x, y, 5) => {
                let (vx, carry) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = vx;

                self.registers[0xF] = if carry { 0 } else { 1 }
            }
            // VX >> 1
            (8, x, _, 6) => {
                let lsb = self.registers[x as usize] & 1;
                self.registers[x as usize] >>= 1;
                self.registers[0xF] = lsb;
            }
            // VX = VY - VX
            (8, x, y, 7) => {
                let (vx, carry) =
                    self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = vx;

                self.registers[0xF] = if carry { 0 } else { 1 }
            }
            (8, x, _, 0xE) => {
                let msb = self.registers[x as usize] & 0xF0;
                self.registers[x as usize] <<= 1;
                self.registers[0xF] = msb;
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
            (0xB, _, _, _) => {
                let nnn = opcode & 0x0FFF;
                self.pc = self.registers[0] as u16 + nnn;
            }
            (0xC, x, _, _) => {
                let nn = (opcode & 0x00FF) as u8;
                let rng: u8 = rand::random();
                self.registers[x as usize] = rng & nn;
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
            (0xE, x, 0x9, 0xE) => {
                if self.keys[x as usize] {
                    self.pc += 2;
                }
            }
            (0xE, x, 0xA, 0x1) => {
                if !self.keys[x as usize] {
                    self.pc += 2;
                }
            }
            (0xF, x, 0x0, 0x7) => {
                self.registers[x as usize] = self.delay_timer;
            }
            (0xF, x, 0x0, 0xA) => {
                if !self.keys[x as usize] {
                    self.pc -= 2;
                }
            }
            (0xF, x, 0x1, 0x5) => {
                self.delay_timer = self.registers[x as usize];
            }
            (0xF, x, 0x1, 0x8) => {
                self.sound_timer = self.registers[x as usize];
            }
            (0xF, x, 0x1, 0xE) => {
                self.i_register += self.registers[x as usize] as u16;
            }
            (0xF, x, 0x2, 0x9) => {
                self.i_register = self.registers[x as usize] as u16 * 5;
            }
            (0xF, x, 0x3, 0x3) => {
                let vx = self.registers[x as usize];

                let hundreds = vx / 100;
                let tens = vx / 10 % 10;
                let ones = vx % 10;

                self.memory[self.i_register as usize] = hundreds;
                self.memory[(self.i_register + 1) as usize] = tens;
                self.memory[(self.i_register + 2) as usize] = ones;
            }
            (0xF, x, 0x5, 0x5) => {
                let i_reg = self.i_register as usize;
                for i in 0..=x as usize {
                    self.memory[i + i_reg] = self.registers[i];
                }
            }
            (0xF, x, 0x6, 0x5) => {
                let i_reg = self.i_register as usize;
                for i in 0..=x as usize {
                    self.registers[i] = self.memory[i + i_reg]
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
        self.screen.pixels().as_ptr()
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct EmuWasm {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl EmuWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EmuWasm, JsValue> {
        let chip8 = Chip8::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("chip-8").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into()
            .map_err(|_| JsValue::from_str("Canvas element not found"))?;

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(Self { chip8, ctx })
    }

    pub fn draw_screen(&mut self, scale: usize) {
        let disp = self.chip8.screen.pixels();
        for (i, pixel) in disp.iter().enumerate() {
            if *pixel {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }

    pub fn load_game(&mut self, game: js_sys::Uint8Array) {
        self.chip8.load(&game.to_vec());
    }

    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    pub fn reset(&mut self) {
        self.chip8.reset();
    }
}
