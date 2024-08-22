use crate::caster::cast_ray;
use crate::texture::{Texture};
use crate::maze::load_maze;
use crate::framebuffer::Framebuffer; 
use crate::player::Player; 
use crate::Vec2;

use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;




static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/img1.jpg")));
static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/cat.png")));

fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)
}

fn detect_collision(player: &Player, enemy_pos: &Vec2, threshold: f32) -> bool {
    let distance = ((player.pos.x - enemy_pos.x).powi(2) + (player.pos.y - enemy_pos.y).powi(2)).sqrt();
    println!("Jugador en: ({}, {}), Enemigo en: ({}, {}), Distancia: {}", 
        player.pos.x, player.pos.y, enemy_pos.x, enemy_pos.y, distance);
    distance < threshold
}


fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2) {
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x); // Ángulo del enemigo relativo al jugador
    let sprite_a_relative = sprite_a - player.a; // Ángulo relativo al jugador
    
    // Asegurarse de que el ángulo esté dentro del rango [-PI, PI]
    let sprite_a_normalized = (sprite_a_relative + PI) % (2.0 * PI) - PI;
    
    // Si el enemigo no está dentro del FOV del jugador, no lo renderizamos
    if sprite_a_normalized.abs() > player.fov / 2.0 {
        return
    }

    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();

    let screen_height = framebuffer.height as f32; 
    let screen_width = framebuffer.width as f32; 

    let sprite_size = (screen_height / sprite_d) * 100.0;

    // Calcular la posición en la pantalla
    let start_x = ((sprite_a_normalized) * (screen_width / player.fov) + (screen_width / 2.0) - (sprite_size / 2.0)) as isize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0) as isize;

    // Convertimos a unsigned integers, pero antes aseguramos que no sea negativo
    let start_x = start_x.max(0) as usize;
    let start_y = start_y.max(0) as usize;

    let end_x = (start_x as f32 + sprite_size).min(screen_width) as usize; 
    let end_y = (start_y as f32 + sprite_size).min(screen_height) as usize; 

    

    for x in start_x..end_x { 
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size as usize) as u32; 
            let ty = ((y - start_y) * 128 / sprite_size as usize) as u32; 
            // Puedes reemplazar este color con 
            let color = ENEMY.get_pixel_color(tx, ty);// una vez que todo funcione bien
            if color != 0xFFFFFF{
                framebuffer.point(x, y, color); 
            }
        }
    }
}


pub fn render_enemies(framebuffer: &mut Framebuffer, player: &Player){
    let mut enemies = vec![
        Vec2::new(280.0,280.0)
    ];

    enemies.retain(|enemy_pos| {
        if detect_collision(player, enemy_pos, 100.0) {
            false // Si hay colisión, el enemigo se elimina
        } else {
            render_enemy(framebuffer, player, enemy_pos);
            true // Si no hay colisión, el enemigo se conserva
        }
    });
}

// recibe donde va a estar, el tamaño de los cuadrados y para ponerle diferentes colores una celda
fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{    
                framebuffer.point(x,y,0xFFFFFF);
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
    let block_size = 20;

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

pub fn render3D(framebuffer: &mut Framebuffer, player: &Player, player2: &Player){
    let maze = load_maze("./archivo.txt");
    let num_rayos = framebuffer.width; 
    let block_size = 100; 

    let hh = framebuffer.height as f32/ 2.0;

    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0; 

        let stake_top = (hh - (stake_heigth / 2.0 )) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0 )) as usize;

        // if stake_top <= framebuffer.height && stake_bottom <= framebuffer.height {
            for y in stake_top..stake_bottom{
                // let ty = (y as f32 - stake_top as f32 ) / (stake_bottom as f32  - stake_top as f32 ) * 128.0;
                // let tx = intersect.tx;
                // let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
                framebuffer.point(i,y,0x000000);
            }
            for y in (stake_bottom as usize)..(framebuffer.height as usize){
                framebuffer.point(i,y as usize, 0x273a28);
            }
            for y in 0..(stake_top as usize){
                framebuffer.point(i,y as usize,0x165590);
            }
        }
        // else{
        //     for y in stake_top..stake_bottom{
        //         framebuffer.point(i,y,0x0c160c);
        //     }
        // }

        
    // }
    // render2D(framebuffer, player2);
    render_enemies(framebuffer, player);

}