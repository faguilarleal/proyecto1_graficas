use crate::bm::write_bmp_file;
use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
    line_color: u32,
}

pub fn extract_color_component(color_value: u32, shift: u32) -> u8 {
    ((color_value >> shift) & 0xFF) as u8
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let buffer = vec![0; width * height];
        let cells = vec![false; width * height]; // Inicializar las células como muertas
        Framebuffer {
            width,
            height,
            buffer,
            background_color: 0xFFFFFF, // Color de fondo predeterminado (negro)
            current_color: 0x000000, // Color de dibujo predeterminado (blanco)
            line_color: 0xFFFFFF, // Color de línea predeterminado (blanco)
          
        }
    }

  
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }
    

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_line_color(&mut self, color: u32) {
        self.line_color = color;
    }
    
    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let inverted_y = self.height - 1 - y;
            self.buffer[inverted_y * self.width + x] = color;
        }
    }
    

    
    
    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<Color> = self.buffer.iter()
            .map(|&color_value| {
                Color::new(
                    extract_color_component(color_value, 16),
                    extract_color_component(color_value, 8),
                    extract_color_component(color_value, 0)
                )
            })
            .collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }
    
}
