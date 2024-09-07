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

use render::{render2D, render3D};
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



fn main() {
    let maze = load_maze("./archivo.txt");
    
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(1);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Laberinto ",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut player1 = Player{
        pos: Vec2::new(100.0, 100.0),
        a: PI/3.0,
        fov: PI/3.0, 
        prev_mouse_x: None, 
    };

    let mut player2 = Player{
        pos: Vec2::new(20.0, 20.0),
        a: PI/3.0,
        fov: PI/3.0, 
        prev_mouse_x: None, 
    };

    let mut mode = "3D";
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    
    let audio_file_path = "./assets/songs.mp3"; 
    let mut audio_player = AudioPlayer::new(audio_file_path); 

    while window.is_open() {
     
        let current_time = Instant::now();
        let duration = current_time.duration_since(last_time);

        if window.is_key_down(Key::Escape) {
            break;
        }

        // if window.is_key_down(Key::M){
        //     mode = if mode == "2D" {"3D"} else {"2D"};
        // }

        procces(&mut window, &mut player1, &maze , 100); // para los controles y movimiento 
        // procces(&mut window, &mut player2, &maze , 20); // para los controles y movimiento 


        framebuffer.clear();

        // if mode == "2D"{
        //     audio_player.stop();
        //     render2D(&mut framebuffer, &mut player1);
        // }
        // else{
             // Incrementar el contador de frames
            // if !audio_player.isplaying {
            //     audio_player = AudioPlayer::new(audio_file_path);
            // }
            // audio_player.play();
        render3D(&mut framebuffer, &mut player1, &mut player2);
         
        // }
            
        frame_count += 1;
        
        // Calcular y mostrar FPS cada segundo
        if duration >= Duration::from_secs(1) {
            let fps = frame_count as f64 / duration.as_secs_f64();
            println!("FPS: {}", fps);
            println!("{}", mode);
            frame_count = 0;
            last_time = Instant::now();
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    

}
