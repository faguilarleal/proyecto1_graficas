use crate::caster::cast_ray;
use crate::texture::{Texture};
use crate::maze::load_maze;
use crate::framebuffer::Framebuffer; 
use crate::player::Player; 
use crate::Vec2;
use crate::Enemy;

use std::f32::consts::PI;
use std::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;




static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/img1.jpg")));
static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/cat.png")));
static mut COUNT_GATOS: u32 = 0;

fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)
}

fn detect_collision(player: &Player, enemy: &mut Enemy, threshold: f32) {
    let distance = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();
    
    if distance < threshold {
        // println!("{}, " ,"se vuelve true");
        enemy.collect(); // Marca el enemigo como recolectado
        // sumar uno al cont de gatos 
        unsafe {
            COUNT_GATOS += 1; // Incrementa el contador de gatos
        }
    }
}

pub fn getGatos() -> u32{
    unsafe {
        COUNT_GATOS
    }
}

pub fn update_enemies(player: &Player, enemies: &Lazy<Mutex<Vec<Enemy>>>, threshold: f32) {
    for enemy in enemies.lock().unwrap().iter_mut() {
        if !enemy.collected {
            detect_collision(player, enemy, threshold);
        }
    }
    
    // Elimina los enemigos recolectados
    enemies.lock().unwrap().retain(|enemy| !enemy.collected);

}


fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, enemy_pos: &Vec2, maze: &Vec<Vec<char>>, enemy: &Enemy) {
    if !enemy.collected{
    // println!("{},",enemy.collected);
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
    }}
}



pub fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, enemies: &Lazy<Mutex<Vec<Enemy>>>, maze: &Vec<Vec<char>>) {
    for enemy in enemies.lock().unwrap().iter_mut() {
        if !enemy.collected {
            render_enemy(framebuffer, player, &enemy.pos, maze, enemy);
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


pub fn render_fps(framebuffer: &mut Framebuffer, numbers: &Vec<Vec<char>>, num: usize) {
    let color = 0xffffff; // Color blanco para el texto
    framebuffer.set_current_color(color);
    text_format(framebuffer, numbers, num, color);
}

pub fn text_format(framebuffer: &mut Framebuffer, numbers: &Vec<Vec<char>>, num: usize, color: u32) {
    for i in 0..6 {
        for col in 0..numbers[i].len() {
            if numbers[i][col] != ' ' {
                render_box(framebuffer, (i + 2) * 3, (col + 50) * 3, 3, 3, color);
            }
        }
    }
    let num_str: Vec<char> = num.to_string().chars().rev().collect();
    let size = num_str.len();
    for digit_pos in 0..size {
        let digit = num_str[size - digit_pos - 1].to_digit(10).expect("Not a valid digit") as usize;
        for i in 0..6 {
            for col in 0..numbers[(6 * (digit + 1)) + i - 1].len() {
                if numbers[(6 * (digit + 1)) + i - 1][col] != ' ' {
                    render_box(framebuffer, (i + 1) * 3, (col + 50) * 3 + (45 + 12 * digit_pos), 3, 3, color);
                }
            }
        }
    }
}



pub fn render_box(framebuffer: &mut Framebuffer, xo: usize, yo: usize, w: usize, h: usize, color: u32) {
    for i in xo..xo + h {
        for j in yo..yo + w {
            framebuffer.point(j, i, color);
        }
    }
}

fn render_mini_map(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let mini_map_size = 88; // Tamaño del mini mapa en píxeles
    let mini_map_margin = 10; // Margen desde el borde de la pantalla
    let mini_map_x = framebuffer.width as usize - mini_map_size * 2 - mini_map_margin;
    let mini_map_y = mini_map_margin;

    let block_size = (mini_map_size as f32 / maze.len() as f32).ceil() as usize;

    // Dibuja el laberinto en el mini mapa
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell = maze[row][col];
            if cell != ' ' {
                let x = mini_map_x + col * block_size;
                let y = mini_map_y + row * block_size;
                for dx in 0..block_size {
                    for dy in 0..block_size {
                        framebuffer.point(x + dx, y + dy, 0xFFFFFF); // Pared blanca
                    }
                }
            }
        }
    }

    // Dibuja al jugador en el mini mapa
    let player_size = 5;
    let player_x = mini_map_x + (player.pos.x / block_size as f32) as usize;
    let player_y = mini_map_y + (player.pos.y / block_size as f32) as usize;
    for x in player_x..player_x + player_size {
        for y in player_y..player_y + player_size {
            if x >= mini_map_x && y >= mini_map_y && x < mini_map_x + mini_map_size && y < mini_map_y + mini_map_size {
                framebuffer.point(x, y, 0xFFFF00); // Jugador amarillo
            }
        }
    }
}
static ENEMIES: Lazy<Mutex<Vec<Enemy>>> = Lazy::new(|| Mutex::new(vec![
    Enemy::new(Vec2::new(250.0, 250.0)),
    Enemy::new(Vec2::new(500.0, 500.0)),
    Enemy::new(Vec2::new(850.0, 500.0)),
    // Agrega más enemigos aquí si lo deseas
]));

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
                let ty = ty.min(WALL1.height - 1);  // Asegúrate de que ty esté dentro del rango válido

                let tx = ((intersect.tx as f32 / block_size as f32) * WALL1.width as f32) as u32;
                let tx = tx.min(WALL1.width - 1);  // Asegúrate de que tx esté dentro del rango válido

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

   

    // Actualizar y renderizar enemigos
    update_enemies(player, &ENEMIES, 100.0);
    render_enemies(framebuffer, player, &ENEMIES, &maze);


     // Renderiza el mini mapa
    // Llamar a render_mini_map con block_size
    render_mini_map(framebuffer, player, &maze);
}


