use nalgebra_glm::{Vec2};
use minifb::{Window, Key, MouseMode};
use std::f32::consts::PI;

use crate::maze::is_wall;


pub struct Player{
    pub pos: Vec2, 
    pub a: f32, // angle of view
    pub fov: f32, // field of view
    pub prev_mouse_x: Option<f32>,
}



impl Player {
    pub fn move_player(&mut self, dir: Vec2, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_pos = self.pos + dir;

        // Verificar colisiones
        let i = (new_pos.x / block_size as f32) as usize;
        let j = (new_pos.y / block_size as f32) as usize;

        if !is_wall(i, j, maze) {
            self.pos = new_pos;
        }
    }

    // Rotar al jugador basado en el movimiento del mouse
    pub fn rotate_with_mouse(&mut self, window: &Window) {
        if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
            if let Some(prev_x) = self.prev_mouse_x {
                let delta_x = mouse_x as f32 - prev_x;
                self.a += delta_x * 0.005;
            }

            self.prev_mouse_x = Some(mouse_x);
        }
    }
}


pub fn procces(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 15.0;
    const ROTATION_SPEED: f32 = PI/ 30.0;


    if window.is_key_down(Key::Up) {
        let dir = Vec2::new(player.a.cos() * MOVE_SPEED, player.a.sin() * MOVE_SPEED);
        player.move_player(dir, maze, block_size);
    }

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::Down) {
        let dir = Vec2::new(-player.a.cos() * MOVE_SPEED, -player.a.sin() * MOVE_SPEED);
        player.move_player(dir, maze, block_size);
    }

    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    player.rotate_with_mouse(window); // Rotar con el mouse
}