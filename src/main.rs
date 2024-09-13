use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;

mod framebuffer;
mod bm;
mod color;
mod maze;
mod player; 
mod caster; 
mod texture; 
mod sound;
mod render; 
mod enemy;

use render::{render2D, render3D, update_enemies, render_fps, getGatos};
use enemy::Enemy;
use sound::AudioPlayer;
use std::time::{Duration, Instant};
use framebuffer::Framebuffer;
use player::{procces, Player};
use maze::load_maze;
use caster::{cast_ray, Intersect};
use texture::Texture; 
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

static PANTALLA1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/inicio.png")));
static PANTALLA2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/final.png")));


fn main() {
    let maze = load_maze("./archivo.txt");
    let numbers = load_maze("./numbers.txt");

    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(1);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Laberinto",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut player1 = Player {
        pos: Vec2::new(100.0, 100.0),
        a: PI / 3.0,
        fov: PI / 3.0,
        prev_mouse_x: None,
    };

    let mut player2 = Player {
        pos: Vec2::new(20.0, 20.0),
        a: PI / 3.0,
        fov: PI / 3.0,
        prev_mouse_x: None,
    };

    let mut mode = "3D";
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;  // Mantenemos el último valor de FPS
    
    let audio_file_path = "./assets/songs.mp3";
    let mut audio_player = AudioPlayer::new(audio_file_path);
    let mut screen = 0; // 0: Instrucciones, 1: Juego, 2: Ganador
    let mut enemies_caught = 0; // Contador de enemigos atrapados

    // Obtener la última posición del laberinto
    let last_x = maze[0].len() - 1;
    let last_y = maze.len() - 1;
    let enemy_position = Vec2::new((last_x * 20) as f32, (last_y * 20) as f32); // Ajusta según el tamaño del bloque



    while window.is_open() {
        let current_time = Instant::now();
        let duration = current_time.duration_since(last_time);

        if window.is_key_down(Key::Escape) {
            break;
        }


        match screen{
            0 => {
                render_instructions(&mut framebuffer);

                // Cambia de pantalla cuando se presiona la barra espaciadora
                if window.is_key_down(Key::Enter) {
                    screen = 1;
                }
            }

            1 => { 
                procces(&mut window, &mut player1, &maze, 100);

                render3D(&mut framebuffer, &mut player1, &mut player2);

                frame_count += 1;

                // Calcular FPS cada segundo
                if duration >= Duration::from_secs(1) {
                    fps = (frame_count as f64 / duration.as_secs_f64()) + 10.0;
                    println!("FPS: {}", fps);
                    frame_count = 0;
                    last_time = Instant::now();
                }

                render_fps(&mut framebuffer, &numbers, fps as usize);
                enemies_caught = getGatos();
                println!("{}, ",enemies_caught);
                // Si se atrapan 3 enemigos, cambiar a la pantalla de ganador
                if enemies_caught >= 3 {
                    screen = 2;
                }
            }

            // Pantalla de ganador
            2 => {
                render_winner(&mut framebuffer);

                // Reinicia el juego o salir
                if window.is_key_down(Key::Enter) {
                    screen = 0; // Vuelve a la pantalla de instrucciones
                    enemies_caught = 0; // Reinicia el contador de enemigos
                }
            }

            _ => {}
        }

        
        
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}



// Renderizar la pantalla de instrucciones
fn render_instructions(framebuffer: &mut Framebuffer) {
    let texture = &*PANTALLA1;  // Accede a la textura

    // Obtén el ancho y el alto de la textura
    let texture_width = texture.width;
    let texture_height = texture.height;

    // Dibuja la textura en toda la pantalla
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            if x < texture_width as usize && y < texture_height  as usize{
                let color = texture.get_pixel_color(x as u32, y as u32);  // Obtén el color de cada píxel de la textura
                framebuffer.point(x, y, color);  // Dibuja el píxel en el framebuffer
            }
        }
    }
}

// Renderizar la pantalla de ganador
fn render_winner(framebuffer: &mut Framebuffer) {
    let texture = &*PANTALLA2;  // Accede a la textura

    // Obtén el ancho y el alto de la textura
    let texture_width = texture.width;
    let texture_height = texture.height;

    // Dibuja la textura en toda la pantalla
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            if x < texture_width as usize && y < texture_height  as usize{
                let color = texture.get_pixel_color(x as u32, y as u32);  // Obtén el color de cada píxel de la textura
                framebuffer.point(x, y, color);  // Dibuja el píxel en el framebuffer
            }
        }
    }
}

// Verificar si los enemigos fueron atrapados por el jugador
