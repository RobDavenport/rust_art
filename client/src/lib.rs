use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const U8MAX: u8 = u8::max_value();
const DIMENSION: u32 = 4096;

#[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
pub struct PixelMap {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl PixelMap {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn new() -> PixelMap {
        let width = DIMENSION;
        let height = DIMENSION;

        let mut pixels: Vec<Pixel> = Vec::new();

        for r in 0..=U8MAX {
            for g in 0..=U8MAX {
                for b in 0..=U8MAX {
                    pixels.push(Pixel { r, g, b });
                }
            }
        }

        PixelMap {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}