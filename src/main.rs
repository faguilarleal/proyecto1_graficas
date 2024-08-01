use minifb::{Key, Window, WindowOptions};
mod framebuffer;
mod bm;
mod color;
mod maze;

use std::time::Duration;
use framebuffer::Framebuffer;
use maze::load_maze;

fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{    
                framebuffer.point(x,y,0xFFFFFF);
            }
        }
    }

}


fn render2D(framebuffer: &mut Framebuffer){
    let maze = load_maze("./archivo.txt");
    let block_size = 60;

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            drawcell(framebuffer, col * block_size,row * block_size , block_size, maze[row][col]);
        }
    }

}


fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(50);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();


    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        framebuffer.clear();
        render2D(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    

}
