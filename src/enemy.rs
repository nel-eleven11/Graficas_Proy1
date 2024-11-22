// enemy.rs

use nalgebra_glm::Vec2;
use once_cell::sync::Lazy;
use std::sync::Arc;
use crate::texture::Texture;

pub static ENEMY_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sprite1.png")));

pub struct Enemy {
    pub pos: Vec2,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Enemy {
            pos: Vec2::new(x, y),
        }
    }
}