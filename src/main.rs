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

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall1.jpg")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall2.jpg")));
static WALL3: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall3.jpg")));
static WALL4: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall4.jpg")));
static WALL5: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/wall5.jpg")));

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
  
fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 70; //100
    let num_rays = framebuffer.width;

    // let hw = framebuffer.width as f32 / 2.0;   // precalculated half width
    let hh = framebuffer.height as f32 / 2.0;  // precalculated half height

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

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = intersect.distance * (a - player.a).cos();// how far is this wall from the player
        let distance_to_projection_plane = 100.0; // how far is the "player" from the "camera"
        // this ratio doesn't really matter as long as it is a function of distance
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Calculate texture coordinates
        // Assume the wall texture width is 128 pixels
        //

        for y in stake_top..stake_bottom {
            // Calculate the vertical offset in the texture
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0; // texture
            // size
            let tx = intersect.tx;
            // Get color from the texture
            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y); // Draw the point in the framebuffer
        }
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
    };

    let mut mode = "2D";

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
            render3d(&mut framebuffer, &player);
        }
    // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}