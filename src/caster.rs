use crate::framebuffer::Framebuffer; 
use crate::player::Player;


pub struct Intersect{
    pub distance: f32, 
    pub impact: char,
    pub tx: usize,
}
pub fn cast_ray(
    framebuffer: &mut Framebuffer, 
    maze: &Vec<Vec<char>>, 
    player: &Player, 
    a: f32, 
    block_size: usize, 
    draw_line: bool, 
) -> Intersect { 
    let mut d = 0.0;
    let cos_a = a.cos();
    let sin_a = a.sin();

    loop { 
        let cos = d * cos_a;
        let sin = d * sin_a;

        let x = (player.pos.x + cos) as usize; 
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size; 
        let j = y / block_size; 
        
        let hitx = x % block_size; 
        let hity = y % block_size; 

        let maxhit = hitx.max(hity); // Determina el mayor valor entre hitx y hity

        if draw_line {
            framebuffer.point(x, y, 0x000000);
        }

        if maze[j][i] != ' ' {
            return Intersect {
                distance: d, 
                impact: maze[j][i],
                tx: maxhit,
            }
        }

        d += 0.8; 
    }
}
