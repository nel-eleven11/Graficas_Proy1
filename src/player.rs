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
pub fn is_collision(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let maze_x = (x / block_size as f32) as isize;
    let maze_y = (y / block_size as f32) as isize;

    // Verificar si los índices están dentro de los límites del laberinto
    if maze_x < 0 || maze_y < 0 || maze_y as usize >= maze.len() || maze_x as usize >= maze[0].len() {
        return true; // Considerar fuera de los límites como una colisión
    }

    let cell = maze[maze_y as usize][maze_x as usize];
    cell != ' ' && cell != 'g' // Ignorar 'g' como colisión
}


pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const MOVE_SPEED: f32 = 0.75;
    const ROTATION_SPEED: f32 = PI / 80.0;
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
    if !is_collision(new_pos.x, new_pos.y, maze, BLOCK_SIZE as usize) {
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

pub fn check_win_condition(player: &Player, maze: &Vec<Vec<char>>) -> bool {
    let player_row = (player.pos.y / BLOCK_SIZE) as usize;
    let player_col = (player.pos.x / BLOCK_SIZE) as usize;

    if player_row >= maze.len() || player_col >= maze[0].len() {
        return false; // El jugador está fuera de los límites
    }

    maze[player_row][player_col] == 'g' // Verifica si la celda actual es 'g'
}
