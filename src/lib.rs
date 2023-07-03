mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Screen {
    width: u32,
    height: u32,
    pixels: Vec<bool>,
}

#[wasm_bindgen]
impl Screen {
    pub fn new() -> Screen {
        let width = 64;
        let height = 32;

        let pixels = (0..width * height)
            .map(|i| i % 2 == 0 || i % 7 == 0)
            .collect();

        Screen {
            width,
            height,
            pixels,
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        self.pixels[(y * self.width + x) as usize] = value;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
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

#[wasm_bindgen]
pub fn greet() {
    utils::set_panic_hook();
    alert("Hello, chip8!");
}
