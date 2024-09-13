extern crate image;

use image::{ImageReader, Pixel};
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    image: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub color_array: Vec<Vec<u32>>, 
}


impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        let color_array = vec![vec![0; width as usize]; height as usize]; // Inicializar con un vector vacío
        Texture { image: img, width, height, color_array }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> u32 {
        if x > self.width || y > self.height {
            0xFFFFFF
        } else {
            let pixel = self.image.get_pixel(x, y).to_rgb();
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32{
        if x>= self.width || y >= self.height { 
            return 0xFFFFFF
        }
        self.color_array[x as usize][y as usize]
    }
}