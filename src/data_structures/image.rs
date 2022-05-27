use crate::data_structures::Color;
use std::fs::File;
use std::io::Write;

static BMP_HEADER: [u8; 54] = [
    b'B', b'M', 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 24,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub struct Image {
    pixels: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let pixels: Vec<u32> = vec![0xffffffff; width * height];
        Image {
            pixels,
            width,
            height,
        }
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
        let mut bytes = BMP_HEADER.to_vec();

        // Set the width and height values in the header
        bytes[0x12..0x16].copy_from_slice(&(self.width as u32).to_le_bytes());
        bytes[0x16..0x1A].copy_from_slice(&(self.height as u32).to_le_bytes());

        // Write the pixel data to the file in the right order
        for row in self.pixels.chunks(self.width).rev() {
            for pixel in row {
                bytes.push(((pixel & 0xff000000) >> 24) as u8);
                bytes.push(((pixel & 0x00ff0000) >> 16) as u8);
                bytes.push(((pixel & 0x0000ff00) >> 8) as u8);
            }
        }

        // Set the file length in the header
        let len = bytes.len() as u32;
        bytes[0x02..0x06].copy_from_slice(&len.to_le_bytes());

        // Write to disk
        let mut file = File::create(filepath).unwrap();
        file.write_all(&bytes).unwrap();
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
