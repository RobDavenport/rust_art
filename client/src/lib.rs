use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const U8MAX: u8 = u8::max_value();
const DIMENSION: u32 = 4096;
const MAXCOLORS: u32 = 256 * 256 * 256;

#[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    pub fn new(rgb: u32) -> Pixel {
        Pixel {
            r: rgb as u8 & 255,
            g: (rgb >> 8) as u8 & 255, 
            b: (rgb >> 16) as u8 & 255, 
            a: 255,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationMethod {
    None = 0,
    Random = 1,
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

    pub fn new(gen: GenerationMethod) -> PixelMap {
        let width = DIMENSION;
        let height = DIMENSION;

        let pixels = generate_pixels(gen);

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

fn default() -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();

    for p in 0..MAXCOLORS {
        pixels.push(Pixel::new(p));
    }
    
    return pixels;
}

fn random_pixels() -> Vec<Pixel> {
    let mut indicies = (0..MAXCOLORS).collect::<Vec<u32>>();
    let mut res: Vec<Pixel> = Vec::new();
    let mut rng = thread_rng();

    indicies.shuffle(&mut rng);

    for p in indicies {
        res.push(Pixel::new(p));
    }

    return res;
}

fn generate_pixels(gen: GenerationMethod) -> Vec<Pixel> {
    match gen {
        GenerationMethod::Random => random_pixels(),
        _ => default()
    }
}
