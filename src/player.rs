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

// Tamaño del bloque (debe coincidir con el bloque en `main.rs`)
const BLOCK_SIZE: f32 = 70.0;

// Verifica si la nueva posición del jugador colisionará con una pared
fn is_collision(new_pos: Vec2, maze: &Vec<Vec<char>>) -> bool {
    let maze_row = (new_pos.y / BLOCK_SIZE) as usize;
    let maze_col = (new_pos.x / BLOCK_SIZE) as usize;

    // Asegúrate de no salir de los límites del laberinto
    if maze_row >= maze.len() || maze_col >= maze[0].len() {
        return true; // Bloquea el movimiento si está fuera del laberinto
    }

    // Verifica si la celda es una pared
    maze[maze_row][maze_col] != ' '
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 15.0;
    const MOUSE_SENSITIVITY: f32 = 0.005; // Ajusta la sensibilidad del ratón

    let mut new_pos = player.pos;

    // Movimiento hacia adelante (W) y hacia atrás (S)
    if window.is_key_down(Key::W) {
        new_pos.x += MOVE_SPEED * player.a.cos();
        new_pos.y += MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(Key::S) {
        new_pos.x -= MOVE_SPEED * player.a.cos();
        new_pos.y -= MOVE_SPEED * player.a.sin();
    }

    // Movimiento lateral (A y D)
    if window.is_key_down(Key::A) {
        new_pos.x -= MOVE_SPEED * (player.a + PI / 2.0).cos();
        new_pos.y -= MOVE_SPEED * (player.a + PI / 2.0).sin();
    }
    if window.is_key_down(Key::D) {
        new_pos.x += MOVE_SPEED * (player.a + PI / 2.0).cos();
        new_pos.y += MOVE_SPEED * (player.a + PI / 2.0).sin();
    }

    // Verifica si la nueva posición es válida (sin colisión)
    if !is_collision(new_pos, maze) {
        player.pos = new_pos;
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
