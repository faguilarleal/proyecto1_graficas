use crate::caster::cast_ray;
use crate::texture::{Texture};
use crate::maze::load_maze;
use crate::framebuffer::Framebuffer; 
use crate::player::Player; 

use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;




static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/img1.jpg")));


fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)
    
}

// recibe donde va a estar, el tamaño de los cuadrados y para ponerle diferentes colores una celda
fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{    
                framebuffer.point(x,y,0x000000);
            }
        }
    }

}

fn render_player2d(framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
    let player_size = block_size ; // Tamaño del jugador (puede ser ajustado)
    let player_x = player.pos.x as usize;
    let player_y = player.pos.y as usize;

    for y in player_y..(player_y + player_size) {
        for x in player_x..(player_x + player_size) {
            framebuffer.point(x, y, 0xFFFFA500);
        }
    }
}

pub fn render2D(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./archivo.txt");
    let block_size = 50;

    // for de dos dimensiones
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            drawcell(framebuffer, col * block_size,row * block_size , block_size, maze[row][col]);
        }
    }

    render_player2d(framebuffer, player, 5);

    let num_rayos = 5; 
    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}

pub fn render3D(framebuffer: &mut Framebuffer, player: &Player){
    let maze = load_maze("./archivo.txt");
    let num_rayos = framebuffer.width; 
    let block_size = 50; 

    let hh = framebuffer.height as f32/ 2.0;

    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0; 

        let stake_top = (hh - (stake_heigth / 2.0 )) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0 )) as usize;

        if stake_top <= framebuffer.height && stake_bottom <= framebuffer.height {
            for y in stake_top..stake_bottom{
                let ty = (y as f32 - stake_top as f32 ) / (stake_bottom as f32  - stake_top as f32 ) * 128.0;
                let tx = intersect.tx;
                let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
                framebuffer.point(i,y,color);
            }
            for y in (stake_bottom as usize)..(framebuffer.height as usize){
                framebuffer.point(i,y as usize,0x165590);
            }
            for y in 0..(stake_top as usize){
                framebuffer.point(i,y as usize,0x273a28);
            }
        }else{
            for y in stake_top..stake_bottom{
                framebuffer.point(i,y,0x0c160c);
            }
        }

        
    }

}