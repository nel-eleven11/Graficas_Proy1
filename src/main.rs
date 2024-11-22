// main.rs

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
use nalgebra_glm::{Vec2};
use once_cell::sync::Lazy;
use std::sync::Arc;
mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;
mod player;
use player::{Player, process_events};
mod raycast;
use raycast::{cast_ray, Intersect};
mod texture;
use texture::Texture;
mod enemy;
use enemy::{Enemy, ENEMY_TEXTURE};

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall1.jpg")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall2.jpg")));
static WALL3: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall3.jpg")));
static WALL4: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall4.jpg")));
static WALL5: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall5.jpg")));
static UI_SPRITE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/player2.png")));
const TRANSPARENT_COLOR: u32 = 0xED1C24;

fn cell_to_color(cell: char) -> u32 {
    let default_color = 0x000000;
    match cell {
        '+' => 0xFF00FF,
        '-'  => 0xDD11DD,
        'g' => 0xFF00,
        '|' => 0xCC11CC,
        _ => default_color,
    }
}

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    match cell {
        '+' => WALL4.get_pixel_color(tx, ty),
        '-' => WALL2.get_pixel_color(tx, ty),
        '|' => WALL1.get_pixel_color(tx, ty),
        'g' => WALL5.get_pixel_color(tx, ty),
        _ => WALL3.get_pixel_color(tx, ty),
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            if cell != ' ' {
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);               
            }
        }
    }   
}

fn draw_sprite(framebuffer: &mut Framebuffer, player: &Player, enemy: &Enemy, z_buffer: &mut [f32]) {

    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
  
    if sprite_a < -1.0 {  // sprite is behind
        return;
    }
  
    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();
    
    if sprite_d < 50.0 {
        return;
    }
  
    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;
    let sprite_size = (screen_height / sprite_d) * 70.0;
    let start_x = ((sprite_a - player.a) * (screen_height / player.fov) + screen_width / 2.0 - sprite_size / 2.0) as f32;
    let start_y = ((screen_height / 2.0) - (sprite_size / 2.0)) as f32;
    let sprite_size = sprite_size as usize;
    // println!("sprite_a: {:#?} sprite_d: {:#?} sprite_size: {:#?}", sprite_a, sprite_d, sprite_size);
  
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;
    let end_x = (start_x + sprite_size).min(framebuffer.width);
    let end_y = (start_y + sprite_size).min(framebuffer.height);
  
    for x in start_x..end_x {
      // Check if this column of the sprite is in front of what's in the z-buffer
        if sprite_d < z_buffer[x] {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                let color = ENEMY_TEXTURE.get_pixel_color(tx, ty);
                if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
                }
            }
            // Update the z-buffer for this column 
            z_buffer[x] = sprite_d;
        }
    }
}

fn render_ui(framebuffer: &mut Framebuffer) {
    let ui_width = 512 as u32; // Adjust this to match your UI sprite width
    let ui_height = 512 as u32; // Adjust this to match your UI sprite height
    let ui_x = ((framebuffer.width as f32 / 2.0) - (ui_width as f32 / 2.0)) as u32; // X position of the UI sprite
    let ui_y = (framebuffer.height - ui_height as usize) as u32; // Y position of the UI sprite
  
    for y in 0..ui_height {
        for x in 0..ui_width {
            let color = UI_SPRITE.get_pixel_color(x as u32, y as u32);
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.point((ui_x + x) as usize, (ui_y + y) as usize);
            }
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 70; //100
  
    // draw the minimap
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }
    // draw the player
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
  
    // draw what the player sees
    let num_rays = 50;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}
  
fn render3d(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut [f32]) {
    let maze = load_maze("./maze.txt");
    let block_size = 100; 
    let num_rays = framebuffer.width;
  
    // Precalculate half height of the framebuffer
    let hh = framebuffer.height as f32 / 2.0;  
  
    // draw the sky and the floor
    for i in 0..framebuffer.width {
        framebuffer.set_current_color(0x383838);
        for j in 0..(framebuffer.height / 2) {
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(0x717171);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }
  
    framebuffer.set_current_color(0x717171);
  
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 70.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        z_buffer[i] = distance_to_wall;
  
        for y in stake_top..stake_bottom {
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0; // texture
            let color = cell_to_texture_color(intersect.impact, intersect.tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}
  
fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut [f32]) {
    let enemies = vec![
        Enemy::new(250.0, 250.0),
        Enemy::new(450.0, 450.0),
        Enemy::new(650.0, 650.0),
    ];
  
    for enemy in &enemies {
        draw_sprite(framebuffer, &player, enemy, z_buffer);
    }
}

fn main() {
    let window_width = 900; //1300
    let window_height = 640;  //900

    let framebuffer_width = 900; //1300
    let framebuffer_height = 640; //900

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(100, 100);
    window.update();

    // initialize values
    framebuffer.set_background_color(0x333355);
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
        last_mouse_x: None,
    };

    let mut mode = "3D";

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        process_events(&window, &mut player);

        // Clear the framebuffer
        framebuffer.clear();

        // Draw some stuff
        if mode == "2D" {
            render2d(&mut framebuffer, &player);
        } else {
            let mut z_buffer = vec![f32::INFINITY; framebuffer.width];
            render3d(&mut framebuffer, &player, &mut z_buffer);
            render_enemies(&mut framebuffer, &player, &mut z_buffer);
            render_ui(&mut framebuffer);
        }
    // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}