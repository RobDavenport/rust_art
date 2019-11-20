use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a: 255
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationMethod {
    None,
    Random,
    SmoothPixels
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorDepth {
    Bit24,
    Bit21,
    Bit18,
    Bit15,
    Bit12,
    Bit9,
    Bit6,
}

#[wasm_bindgen]
pub struct ColorDepthData {
    width: u16,
    height: u16,
    steps: u8,
    bitmask: u32
}

impl ColorDepthData {

    pub fn max_colors(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    pub fn new(color_depth: ColorDepth) -> ColorDepthData {
        match color_depth {
            ColorDepth::Bit24 => ColorDepthData {
                width: 4096,
                height: 4096,
                steps: 255,
                bitmask: 0b11111111_11111111_11111111_11111111,
            },
            ColorDepth::Bit21 => ColorDepthData {
                width: 2048,
                height: 1024,
                steps: 127,
                bitmask: 0b11111111_01111111_01111111_01111111
            },
            ColorDepth::Bit18 => ColorDepthData {
                width: 512,
                height: 512,
                steps: 63, 
                bitmask: 0b11111111_00111111_00111111_00111111
            },
            ColorDepth::Bit15 => ColorDepthData {
                width: 256,
                height: 128,
                steps: 31,
                bitmask: 0b11111111_00011111_00011111_00011111
            },
            ColorDepth::Bit12 => ColorDepthData {
                width: 64,
                height: 64,
                steps: 15,
                bitmask: 0b11111111_00001111_00001111_00001111
            },
            ColorDepth::Bit9 => ColorDepthData {
                width: 32,
                height: 16,
                steps: 7,
                bitmask: 0b11111111_00000111_00000111_00000111
            },
            ColorDepth::Bit6 => ColorDepthData {
                width: 8,
                height: 8,
                steps: 3,
                bitmask: 0b11111111_00000011_00000011_00000011
            },
        }
    }
}

#[wasm_bindgen]
pub struct PixelMap {
    color_depth_data: ColorDepthData,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl PixelMap {
    pub fn new(gen: GenerationMethod, color_depth: ColorDepth) -> PixelMap {
        let color_depth_data = ColorDepthData::new(color_depth);
        let pixels = generate_pixels(gen, &color_depth_data);

        PixelMap {
            color_depth_data,
            pixels,
        }
    }

    pub fn width(&self) -> u16 {
        self.color_depth_data.width
    }

    pub fn height(&self) -> u16 {
        self.color_depth_data.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}

//TODO: Optimize this interpolation
fn default_pixels(color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    let steps = color_depth_data.steps;
    let contrast = 255.0 / steps as f32;

    for r in 0..=steps {
        for g in 0..=steps {
            for b in 0..=steps {
                let pr = (contrast * r as f32).floor() as u8;
                let pg = (contrast * g as f32).floor() as u8;
                let pb = (contrast * b as f32).floor() as u8;
                pixels.push(Pixel::new(pr, pg, pb));
            }
        }
    }
    
    return pixels;
}

//TODO: This function
fn smooth_pixels(color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    let steps = color_depth_data.steps;
    let contrast = 255.0 / steps as f32;

    for r in 0..=steps {
        for g in 0..=steps {
            for b in 0..=steps {
                let pr = (contrast * r as f32).floor() as u8;
                let pg = (contrast * 0 as f32).floor() as u8;
                let pb = (contrast * 0 as f32).floor() as u8;
                pixels.push(Pixel::new(pr, pg, pb as u8));
            }
        }
    }
    
    return pixels;
}

fn random_pixels(color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    let mut rng = thread_rng();
    let mut pixels = default_pixels(color_depth_data);
    pixels.shuffle(&mut rng);

    return pixels;
}

fn generate_pixels(gen: GenerationMethod, color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    match gen {
        GenerationMethod::None => default_pixels(color_depth_data),
        GenerationMethod::Random => random_pixels(color_depth_data),
        GenerationMethod::SmoothPixels => smooth_pixels(color_depth_data)
    }
}