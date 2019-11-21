use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

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
    a: u8,
}

impl Pixel {
    pub fn new_raw(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }
    pub fn new_indexed(r: u8, g: u8, b: u8, color_depth_data: &ColorDepthData) -> Pixel {
        Pixel {
            a: 255,
            r: calculate_color(r, color_depth_data),
            g: calculate_color(g, color_depth_data),
            b: calculate_color(b, color_depth_data),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationMethod {
    None,
    Random,
    SmoothPixels,
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
            },
            ColorDepth::Bit21 => ColorDepthData {
                width: 2048,
                height: 1024,
                steps: 127,
            },
            ColorDepth::Bit18 => ColorDepthData {
                width: 512,
                height: 512,
                steps: 63,
            },
            ColorDepth::Bit15 => ColorDepthData {
                width: 256,
                height: 128,
                steps: 31,
            },
            ColorDepth::Bit12 => ColorDepthData {
                width: 64,
                height: 64,
                steps: 15,
            },
            ColorDepth::Bit9 => ColorDepthData {
                width: 32,
                height: 16,
                steps: 7,
            },
            ColorDepth::Bit6 => ColorDepthData {
                width: 8,
                height: 8,
                steps: 3,
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

fn default_pixels(color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    let steps = color_depth_data.steps;

    for r in 0..=steps {
        let pr = calculate_color(r, color_depth_data);
        for g in 0..=steps {
            let pg = calculate_color(g, color_depth_data);
            for b in 0..=steps {
                let pb = calculate_color(b, color_depth_data);
                pixels.push(Pixel::new_raw(pr, pg, pb));
            }
        }
    }

    return pixels;
}

//TODO: This function
fn smooth_pixels(color_depth_data: &ColorDepthData) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    let steps = color_depth_data.steps;
    let blue_step = color_depth_data.width / (steps as u16 + 1);
    let mut pixel_count = 0;
    let mut pb = 0;

    for r in 0..=steps {
        for g in 0..=steps {
            for _ in 0..=steps {
                pixels.push(Pixel::new_indexed(r, g, pb, color_depth_data));
                pixel_count += 1;
                if pixel_count >= blue_step {
                    pixel_count = 0;
                    if pb == steps {
                        pb = 0
                    } else {
                        pb += 1
                    }
                }
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
        GenerationMethod::SmoothPixels => smooth_pixels(color_depth_data),
    }
}

fn calculate_color_raw(index: u8, steps: u8) -> u8 {
    (index as f64 * (255.0 / steps as f64)).floor() as u8
}

fn calculate_color(index: u8, color_depth_data: &ColorDepthData) -> u8 {
    calculate_color_raw(index, color_depth_data.steps)
}
