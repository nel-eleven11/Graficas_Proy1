// main.rs

use minifb::{Key, Window, WindowOptions};
use std::{default, time::Duration};
mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;

fn cell_to_color(cell: char) -> u32 {

    let default_color = 0x000000;
    match cell {
        '+' => 0xFF00FF,
        '-'  => 0xDD11DD,
        'p' => 0x0000FF,
        'g' => 0xFF00,
        '|' => 0xCC11CC,
        _ => default_color,
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

fn render(framebuffer: &mut Framebuffer) {
  let maze = load_maze("./maze.txt");
  let block_size = 70;  // 10 pixels each block

  for row in 0..maze.len() {
    for col in 0..maze[row].len() {
      draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
    }
  } 
}

fn main() {
  let window_width = 900;
  let window_height = 600;

  let framebuffer_width = 900;
  let framebuffer_height = 600;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Maze Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  // move the window around
  window.set_position(100, 100);
  window.update();

  framebuffer.set_background_color(0x333355);

  while window.is_open() {
    // listen to inputs
    if window.is_key_down(Key::Escape) {
      break;
    }

    // Clear the framebuffer
    framebuffer.clear();

    // Draw some stuff
    render(&mut framebuffer);

    // Update the window with the framebuffer contents
    window
      .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
      .unwrap();

    std::thread::sleep(frame_delay);
  }
}