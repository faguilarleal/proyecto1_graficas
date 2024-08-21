use crate::caster::cast_ray;
use crate::texture::{Texture};
use crate::maze::load_maze;
use crate::framebuffer::Framebuffer; 
use crate::player::Player; 

use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;


static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/img1.jpg")));

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    WALL1.get_pixel_color(tx, ty) // no es necesario return 
}


// recibe donde va a estar, el tamaño de los cuadrados y para ponerle diferentes colores una celda
fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell != ' ' {
        for x in xo..xo + block_size {
            for y in yo..yo + block_size {
                framebuffer.point(x, y, 0x000000);
            }
        }
    }
}

fn render_player2d(framebuffer: &mut Framebuffer, player: &Player, block_size: usize, offset_x: usize, offset_y: usize) {
    let player_size = block_size; // Tamaño del jugador (puede ser ajustado)
    let player_x = (player.pos.x * block_size as f32) as usize + offset_x;
    let player_y = (player.pos.y * block_size as f32) as usize + offset_y;

    let x_end = player_x + player_size;
    let y_end = player_y + player_size;

    for y in player_y..y_end {
        for x in player_x..x_end {
            framebuffer.point(x, y, 0xFFFFA500);
        }
    }
}

pub fn render2D(framebuffer: &mut Framebuffer, player: &Player, offset_x: usize, offset_y: usize) {
    let maze = load_maze("./archivo.txt");
    let block_size = 50;

    // for de dos dimensiones
    for (row, row_data) in maze.iter().enumerate() {
        for (col, &cell) in row_data.iter().enumerate() {
            drawcell(framebuffer, col * block_size + offset_x, row * block_size + offset_y, block_size, cell);
        }
    }

    render_player2d(framebuffer, player, 5, offset_x, offset_y);
    let num_rayos = 5;
    for i in 0..num_rayos {
        let current_ray = i as f32 / num_rayos as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}

pub fn render3D(framebuffer: &mut Framebuffer, player: &Player, player2: &Player) {
    let maze = load_maze("./archivo.txt");
    let num_rayos = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;
    let block_size = 100;
    let inv_num_rayos = 1.0 / num_rayos as f32;

    for i in 0..num_rayos {
        let current_ray = i as f32 * inv_num_rayos;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0;

        let stake_top = (hh - (stake_heigth / 2.0)) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0)) as usize;

        // let ty_step = 200.0 / (stake_bottom - stake_top) as f32;

        // if stake_top <= framebuffer.height && stake_bottom <= framebuffer.height {
            let mut ty = 0.0;
            for y in stake_top..stake_bottom {
                // let color = cell_to_texture_color(intersect.impact, intersect.tx as u32, ty as u32);
                framebuffer.point(i, y, 0x000000);
                // ty += ty_step;
            }
            for y in stake_bottom..framebuffer.height {
                framebuffer.point(i, y, 0x165590); // Cielo
            }
            for y in 0..stake_top {
                framebuffer.point(i, y, 0x273a28); // Suelo
            }
        
        // } else {
        //     for y in stake_top..stake_bottom {
        //         framebuffer.point(i, y, 0x000000);
        //     }
        // }
    }

    // Renderizar el mapa 2D en la esquina superior izquierda del renderizado 3D
    // render2D(framebuffer, player2, 10, 10);
}

