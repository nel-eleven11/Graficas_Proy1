// texture.rs

extern crate image;

use image::{ImageReader, Pixel};
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    image: DynamicImage,
    pub width: u32,
    pub height: u32,
    color_array: Vec<Vec<u32>>,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        let mut texture = Texture { 
            image: img, 
            width, 
            height, 
            color_array: vec![vec![0; height as usize]; width as usize],
        };
        texture.load_color_array();
        texture
    }

    fn load_color_array(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.image.get_pixel(x, y).to_rgb();
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                self.color_array[x as usize][y as usize] = color;
            }
        }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            0xFF00FF
        } else {
            self.color_array[x as usize][y as usize]
        }
    }
}