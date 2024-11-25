// main.rs

use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};
//use std::f32::consts::PI;
use nalgebra_glm::Vec2;
//use std::process;
use once_cell::sync::Lazy;
use std::sync::Arc;

mod framebuffer;
mod maze;
mod player;
mod raycast;
mod texture;
mod enemy;
mod audio;
mod display_stats;
mod screen;

use framebuffer::Framebuffer;
use maze::load_maze;
use player::{Player, process_events, check_win_condition};
use raycast::cast_ray;
use texture::Texture;
use enemy::{Enemy, ENEMY_TEXTURE};
use audio::AudioPlayer;
use display_stats::Timer;
use screen::Screen;

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

fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    
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
  
fn render3d(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut [f32], maze: &Vec<Vec<char>>) {
    
    let block_size = 70; 
    let num_rays = framebuffer.width;
  
    // Precalculate half height of the framebuffer
    let hh = framebuffer.height as f32 / 2.0;  
  
    // draw the sky and the floor
    for i in 0..framebuffer.width {
        framebuffer.set_current_color(0x2B2E3D);
        for j in 0..(framebuffer.height / 2) {
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(0x222530);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }
  
    framebuffer.set_current_color(0x222530);
  
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

fn show_welcome_screen(window: &mut Window, framebuffer: &mut Framebuffer) -> String {
    // Crear la pantalla de bienvenida
    let mut welcome_screen = Screen::new();
    welcome_screen.set_background("assets/main_screen.jpg"); // Imagen de fondo

    // Título e instrucciones
    welcome_screen.add_text(
        "ESCAPE THE DEATH STAR",
        framebuffer.width / 2 - 200,
        framebuffer.height / 4,
        3,
        0xFFFFFF, // Blanco
    );
    welcome_screen.add_text(
        "Selecciona un nivel y presiona Enter:",
        framebuffer.width / 2 - 250,
        framebuffer.height / 3,
        2,
        0xFFFFFF, // Blanco
    );

    let levels = vec!["Nivel 1", "Nivel 2", "Nivel 3"];
    let mut selected_level = 0; // Índice del nivel seleccionado

    loop {
        // Actualizar los textos dinámicos (niveles)
        welcome_screen.text.truncate(2); // Mantener solo título e instrucciones
        for (i, level) in levels.iter().enumerate() {
            let color = if i == selected_level { 0x00FF00 } else { 0xFFFFFF }; // Verde para seleccionado
            welcome_screen.add_text(
                level,
                framebuffer.width / 2 - 50,
                framebuffer.height / 2 + i * 30,
                2,
                color,
            );
        }

        // Renderizar la pantalla
        welcome_screen.render(framebuffer);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        // Procesar entrada del usuario
        if window.is_key_down(Key::Up) && selected_level > 0 {
            selected_level -= 1; // Mover hacia arriba en la lista
        }
        if window.is_key_down(Key::Down) && selected_level < levels.len() - 1 {
            selected_level += 1; // Mover hacia abajo en la lista
        }
        if window.is_key_down(Key::Enter) {
            // Devuelve el archivo de laberinto correspondiente al nivel seleccionado
            return format!("maze{}.txt", selected_level + 1);
        }

        // Evitar detección repetida de teclas
        std::thread::sleep(Duration::from_millis(100));
    }
}


fn show_win_screen(window: &mut Window, framebuffer: &mut Framebuffer) {

	// Crear la pantalla de victoria
	let mut win_screen = Screen::new();
	let color = 0xFFFFFF; // Color blanco
	win_screen.set_background("assets/win_screen.jpg"); // Establece una imagen de fondo
	win_screen.add_text(
		"Has escapado de la Death Star",
		framebuffer.width / 2 - 320,
		framebuffer.height / 3,
		3,
		color,

	);
	win_screen.add_text(
		"Presiona Esc para salir",
		framebuffer.width / 2 - 200,
		framebuffer.height / 2,
		2,
		color,
	);

	// Mostrar la pantalla de ganar
	loop {
		win_screen.render( framebuffer);
		window
			.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
			.unwrap();

		// Salir del bucle al presionar Esc
		if window.is_key_down(Key::Escape) {
			break;
		}

		std::thread::sleep(Duration::from_millis(16));
	}
}

fn show_defeat_screen(window: &mut Window, framebuffer: &mut Framebuffer) {
    // Crear la pantalla de derrota
    let mut defeat_screen = Screen::new();
	let color = 0xFFFFFF; // Color blanco
	defeat_screen.set_background("assets/lose_screen.jpg"); // Establece una imagen de fondo
	defeat_screen.add_text(
		"No has logrado escapar",
		framebuffer.width / 2 - 245,
		framebuffer.height / 3,
		3,
		color,
	);
	defeat_screen.add_text(
		"Presiona Esc para salir",
		framebuffer.width / 2 - 200,
		framebuffer.height / 2,
		2,
		color,
	);

	// Mostrar la pantalla de derrota
	loop {
		defeat_screen.render( framebuffer);
		window
			.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
			.unwrap();

		// Salir del bucle al presionar Esc
		if window.is_key_down(Key::Escape) {
			break;
		}

		std::thread::sleep(Duration::from_millis(16));
	}
}

fn render_minimap(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, player: &Player) {
    // Configuración del minimapa
    let minimap_size = 100; // Tamaño total del minimapa
    let cell_size = minimap_size / maze.len(); // Tamaño de cada celda del minimapa
    let minimap_x = framebuffer.width - minimap_size - 200; // Margen derecho
    let minimap_y = 5; // Margen superior

    // Dibujar el minimapa
    for (row, line) in maze.iter().enumerate() {
		for (col, &cell) in line.iter().enumerate() {
			let x = minimap_x + col * cell_size;
			let y = minimap_y + row * cell_size;
	
			// Dibujar la celda solo si está dentro de los límites del framebuffer
			if x + cell_size < framebuffer.width && y + cell_size < framebuffer.height {
				let color = match cell {
					'+' | '-' | '|' => 0x888888, // Pared (gris)
					'g' => 0x00FF00,            // Meta (verde)
					_ => 0x000000,              // Espacio vacío (negro)
				};
	
				for i in 0..cell_size {
					for j in 0..cell_size {
						framebuffer.set_current_color(color);
						framebuffer.point(x + i, y + j);
					}
				}
			}
		}
	}

    // Dibujar al jugador en el minimapa
    let player_col = (player.pos.x / 70.0) as usize; // Escalado del jugador según el bloque
    let player_row = (player.pos.y / 70.0) as usize;
    let player_x = minimap_x + player_col * cell_size + cell_size / 2;
    let player_y = minimap_y + player_row * cell_size + cell_size / 2;

    let player_color = 0xFF0000; // Color rojo para el jugador
    let player_radius = (cell_size / 4) as isize;

    for i in -player_radius..=player_radius {
		for j in -player_radius..=player_radius {
			let px = player_x as isize + i;
			let py = player_y as isize + j;
	
			// Asegurarnos de que px y py están dentro de los límites
			if px >= 0 && py >= 0 && px < framebuffer.width as isize && py < framebuffer.height as isize {
				framebuffer.set_current_color(player_color);
				framebuffer.point(px as usize, py as usize); // Convertir de nuevo a usize para dibujar
			}
		}
	}
}


fn main() {
    let window_width = 900; //1300
    let window_height = 635;  //900

    let framebuffer_width = 900; //1300
    let framebuffer_height = 635; //900

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    println!(
        "Inicializando Framebuffer con tamaño: {}x{}, buffer: {}",
        framebuffer_width,
        framebuffer_height,
        framebuffer.buffer.len()
    );

    let mut window = Window::new(
        "Rust Graphics - Maze Game",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

	// Mostrar la pantalla de bienvenida y obtener el nivel seleccionado
	let selected_level = show_welcome_screen(&mut window, &mut framebuffer);

	// Cargar el laberinto correspondiente al nivel seleccionado
	let maze = load_maze(&selected_level);

    // Inicializa al jugador
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: std::f32::consts::PI / 3.0,
        fov: std::f32::consts::PI / 3.0,
        last_mouse_x: None,
    };

    let mut mode = "3D";

    // Inicializar los reproductores de audio
    let background_music = AudioPlayer::new("assets/death_star_alarm.mp3");
    
    // Reproducir música de fondo
    background_music.play_loop();

    // Temporizador
    let mut timer = Timer::new();
    let start_time = Instant::now();
    let max_time: u64 = 30; // Tiempo máximo en segundos

    while window.is_open() {

        timer.update();

        let elapsed_time = start_time.elapsed().as_secs();
        let time_left = max_time.saturating_sub(elapsed_time);

        // Escucha entradas
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        process_events(&window, &mut player, &maze);

        // Verifica el tiempo restante
        let elapsed_time = start_time.elapsed().as_secs();
        let time_left = 30u64.saturating_sub(elapsed_time);

        // Verifica la condición de victoria
        if check_win_condition(&player, &maze) {
            // Detén la música de fondo y reproduce el sonido de victoria
            background_music.stop();
			show_win_screen(&mut window, &mut framebuffer);
            let win_sound = AudioPlayer::new("assets/celebration_sound.mp3");
            win_sound.play();
            println!("¡Felicidades! Has ganado el juego.");
            std::thread::sleep(Duration::from_secs(15)); // Espera 5 segundos
            break;
        }

        // Verificar si el tiempo se agotó
        if time_left == 0 {
            background_music.stop();
			show_defeat_screen(&mut window, &mut framebuffer);
            let lose_sound = AudioPlayer::new("assets/explosion_sound.mp3");
            lose_sound.play();
            println!("Tiempo agotado. Has perdido el juego.");
            std::thread::sleep(std::time::Duration::from_secs(5));
            break;
        }

        // Limpia el framebuffer
        framebuffer.clear();

        // Renderiza
        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze);
        } else {
            let mut z_buffer = vec![f32::INFINITY; framebuffer.width];
            render3d(&mut framebuffer, &player, &mut z_buffer, &maze);
            render_enemies(&mut framebuffer, &player, &mut z_buffer);
            render_ui(&mut framebuffer);
        }

		// Agrega el renderizado del minimapa aquí
		render_minimap(&mut framebuffer, &maze, &player);

        // Mostrar FPS y tiempo restante en la esquina superior izquierda
        window.set_title(&format!(
            "FPS: {:.1} | Tiempo restante: {}s",
            timer.get_fps(),
            time_left
        ));

        // Actualiza la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

