use crate::data_structures::Color;
use std::fs::File;
use std::io::Write;

pub struct Image {
    pixels: Vec<u32>,
    pub width: usize,
    pub height: usize
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let pixels: Vec<u32> = vec![0xffffffff; width * height];
        Image { pixels, width, height }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let bytes = color.to_bytes();
        self.pixels[y * self.width + x] = bytes;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let bytes = self.pixels[y * self.width + x];
        Color::from_bytes(bytes)
    }

    pub fn save(&self, filepath: &str) {
        let mut output_string = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in &self.pixels {
            let r = (pixel & 0xff000000) >> 24;
            let g = (pixel & 0x00ff0000) >> 16;
            let b = (pixel & 0x0000ff00) >> 8;

            output_string.push_str(&format!("{} {} {}\n", r, g, b));
        }

        let mut file = File::create(filepath).unwrap();
        file.write(output_string.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let image = Image::new(64, 64);
        assert!(image.pixels.len() == 64 * 64)
    }
}
