// framebuffer.rs

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if index < self.buffer.len() {
                self.buffer[index] = self.current_color;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

	pub fn draw_text(&mut self, text: &str, x: usize, y: usize, scale: usize) {
        let font = include_bytes!("../assets/font8x8_basic.bin"); // Fuente básica de 8x8
        let mut cursor_x = x;

        for ch in text.chars() {
            if let Some(index) = (ch as usize).checked_mul(8) {
                for row in 0..8 {
                    let byte = font[index + row];
                    for col in 0..8 {
                        if (byte >> col) & 1 != 0 {
                            //let pixel_x = cursor_x + col * scale;
                            //let pixel_y = y + row * scale;

                            for sx in 0..scale {
                                for sy in 0..scale {
                                    let px = cursor_x + col * scale + sx;
                                    let py = y + row * scale + sy;
                            
                                    if px < self.width && py < self.height {
                                        self.point(px, py);
                                    }
                                }
                            }
                        }
                    }
                }
                cursor_x += 8 * scale + 1; // Espacio entre caracteres
            }
        }
    }
}