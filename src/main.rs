mod framebuffer;
mod render;
mod maze;
mod player;
mod caster;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use maze::*;
use caster::cast_ray;  
use render::*;

fn main() {
    let window_width: i32 = 800;
    let window_height: i32 = 600;
    
    let framebuffer_width = 800;
    let framebuffer_height: u32 = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Polygon Drawing")
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);
    

    let maze: Maze = load_maze("src/maze.txt"); 

    let block_size = window_width as usize / maze[0].len();
    let mut player = Player::new(Vector2::new(100.0, 100.0), PI / 4.0);  

    while !window.window_should_close() {
        
        framebuffer.clear();
        render_maze(&mut framebuffer, &maze, &mut player, block_size);
        render_player(&mut framebuffer, &mut player);
        cast_ray(&mut framebuffer, &mut player, &maze, block_size);
        

        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));

    }
    
}
