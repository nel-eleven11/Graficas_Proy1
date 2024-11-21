// players.rs

use nalgebra_glm::{Vec2};
use minifb::{Key, Window};
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // angle of view 
} 

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
  
    if window.is_key_down(Key::Left) {
      player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
      player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Down) {
      player.pos.x = player.pos.x - MOVE_SPEED * player.a.cos();
      player.pos.y = player.pos.y - MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(Key::Up) {
      player.pos.x = player.pos.x + MOVE_SPEED * player.a.cos();
      player.pos.y = player.pos.y + MOVE_SPEED * player.a.sin();
    }
  }