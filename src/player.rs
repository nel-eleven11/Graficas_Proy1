// player.rs

use nalgebra_glm::Vec2;
use minifb::{Key, Window};
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de visión
    pub fov: f32, // campo de visión
    pub last_mouse_x: Option<f32>, // para rastrear la última posición del mouse
}

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 6.0;
    const ROTATION_SPEED: f32 = PI / 20.0;
    const MOUSE_SENSITIVITY: f32 = 0.005; // Ajusta la sensibilidad del ratón

    // Movimiento con teclas WASD
    if window.is_key_down(Key::W) {
        player.pos.x += MOVE_SPEED * player.a.cos();
        player.pos.y += MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(Key::S) {
        player.pos.x -= MOVE_SPEED * player.a.cos();
        player.pos.y -= MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(Key::A) {
        player.pos.x -= MOVE_SPEED * (player.a + PI / 2.0).cos();
        player.pos.y -= MOVE_SPEED * (player.a + PI / 2.0).sin();
    }
    if window.is_key_down(Key::D) {
        player.pos.x += MOVE_SPEED * (player.a + PI / 2.0).cos();
        player.pos.y += MOVE_SPEED * (player.a + PI / 2.0).sin();
    }

    // Rotación con flechas
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }

    // Rotación con el ratón
    if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        if let Some(last_mouse_x) = player.last_mouse_x {
            let delta_x = mouse_x - last_mouse_x;
            player.a += delta_x * MOUSE_SENSITIVITY;
        }
        player.last_mouse_x = Some(mouse_x);
    } else {
        player.last_mouse_x = None;
    }

    // Limitar el ángulo entre 0 y 2PI para evitar desbordamientos
    if player.a < 0.0 {
        player.a += 2.0 * PI;
    } else if player.a > 2.0 * PI {
        player.a -= 2.0 * PI;
    }
}
