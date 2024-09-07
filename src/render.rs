use crate::caster::cast_ray;
use crate::texture::{Texture};
use crate::maze::load_maze;
use crate::framebuffer::Framebuffer; 
use crate::player::Player; 
use crate::Vec2;
use crate::Enemy;

use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;




static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/img1.jpg")));
static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/cat.png")));

fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)
}

fn detect_collision(player: &Player, enemy: &mut Enemy, threshold: f32) {
    let distance = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();
    if distance < threshold {
        enemy.collected = true; // Marca el enemigo como recolectado
    }
}

pub fn update_enemies(player: &Player, enemies: &mut Vec<Enemy>, threshold: f32) {
    for enemy in enemies.iter_mut() {
        detect_collision(player, enemy, threshold);
    }
    
    // Elimina los enemigos recolectados
    enemies.retain(|enemy| !enemy.collected);
}

fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, enemy_pos: &Vec2, maze: &Vec<Vec<char>>) {
    let sprite_a = (enemy_pos.y - player.pos.y).atan2(enemy_pos.x - player.pos.x); // Ángulo del enemigo relativo al jugador
    let sprite_a_relative = sprite_a - player.a; // Ángulo relativo al jugador

    // Asegurarse de que el ángulo esté dentro del rango [-PI, PI]
    let sprite_a_normalized = (sprite_a_relative + PI) % (2.0 * PI) - PI;

    // Si el enemigo no está dentro del FOV del jugador, no lo renderizamos
    if sprite_a_normalized.abs() > player.fov / 2.0 {
        return;
    }

    // Verificamos si hay alguna pared entre el jugador y el enemigo
    let ray_to_enemy = cast_ray(framebuffer, maze, player, sprite_a, 100, false);

    // Si la distancia a la pared es menor que la distancia al enemigo, no renderizamos al enemigo
    let distance_to_enemy = ((player.pos.x - enemy_pos.x).powi(2) + (player.pos.y - enemy_pos.y).powi(2)).sqrt();
    if ray_to_enemy.distance < distance_to_enemy {
        return; // Hay una pared bloqueando al enemigo, no lo renderizamos
    }

    // Renderizado del enemigo si no está bloqueado por una pared
    let sprite_d = distance_to_enemy;
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
            let color = ENEMY.get_pixel_color(tx, ty);
            if color != 0xFFFFFF {
                framebuffer.point(x, y, color);
            }
        }
    }
}

pub fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, enemies: &Vec<Enemy>, maze: &Vec<Vec<char>>) {
    for enemy in enemies.iter() {
        if !enemy.collected {
            render_enemy(framebuffer, player, &enemy.pos, maze);
        }
    }
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



pub fn render3D(framebuffer: &mut Framebuffer, player: &Player, player2: &Player) {
    let maze = load_maze("./archivo.txt");
    let num_rayos = framebuffer.width;
    let block_size = 100;

    let hh = framebuffer.height as f32 / 2.0;

    for i in 0..num_rayos {
        let current_ray = i as f32 / num_rayos as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0;
        let stake_top = (hh - (stake_heigth / 2.0)) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0)) as usize;

        // Verificar que stake_top y stake_bottom estén dentro del rango de la altura del framebuffer
        if stake_top <= framebuffer.height && stake_bottom <= framebuffer.height {
            for y in stake_top..stake_bottom {
                // Cálculo de ty: ajuste basado en la altura proyectada de la pared
                let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * WALL1.height as f32) as u32;
                
                // Cálculo de tx: depende de la intersección horizontal (impacto) con la pared
                let tx = ((intersect.tx as f32 / block_size as f32) * WALL1.width as f32) as u32;

                // Obtener el color de la textura correspondiente a tx y ty
                let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);

                // Dibujar el pixel en el framebuffer
                framebuffer.point(i, y, color);
            }

            // Dibujar el piso (por debajo de la pared)
            for y in (stake_bottom as usize)..(framebuffer.height as usize) {
                framebuffer.point(i, y as usize, 0x273a28);  // Color del piso
            }

            // Dibujar el cielo (por encima de la pared)
            for y in 0..(stake_top as usize) {
                framebuffer.point(i, y as usize, 0x165590);  // Color del cielo
            }
        } else {
            // Si stake_top o stake_bottom están fuera del rango, no se dibuja la pared
            for y in stake_top..stake_bottom {
                framebuffer.point(i, y, 0x0c160c);  // Color de fondo en caso de error
            }
        }
    }

    // Enemigos en el laberinto
    let mut enemies = vec![
        Enemy::new(Vec2::new(250.0, 250.0)),
        // Agrega más enemigos aquí si lo deseas
    ];

    // Actualizar y renderizar enemigos
    update_enemies(player, &mut enemies, 100.0);
    render_enemies(framebuffer, player, &enemies, &maze);
}
