// screen.rs

use crate::framebuffer::Framebuffer;
use crate::texture::Texture;

pub struct Screen {
    background: Option<Texture>, // Imagen de fondo opcional
    pub text: Vec<(String, usize, usize, usize, u32)>, // Texto: (contenido, x, y, escala, color)
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            background: None,
            text: Vec::new(),
        }
    }

    // Establecer una imagen de fondo
    pub fn set_background(&mut self, image_path: &str) {
        self.background = Some(Texture::new(image_path));
    }

    // Agregar texto con color
    pub fn add_text(&mut self, content: &str, x: usize, y: usize, scale: usize, color: u32) {
        self.text.push((content.to_string(), x, y, scale, color));
    }

    // Renderizar la pantalla
    pub fn render(&self, framebuffer: &mut Framebuffer) {
        // Dibujar la imagen de fondo (si existe)
        if let Some(bg) = &self.background {
            for y in 0..framebuffer.height {
                for x in 0..framebuffer.width {
                    let tx = (x as u32 * bg.width / framebuffer.width as u32) as u32;
                    let ty = (y as u32 * bg.height / framebuffer.height as u32) as u32;
                    let color = bg.get_pixel_color(tx, ty);
                    framebuffer.set_current_color(color);
                    framebuffer.point(x, y);
                }
            }
        } else {
            framebuffer.clear(); // Si no hay fondo, limpia el framebuffer
        }

        // Dibujar textos
        for (content, x, y, scale, color) in &self.text {
            framebuffer.set_current_color(*color);
            framebuffer.draw_text(content, *x, *y, *scale);
        }
    }
}
