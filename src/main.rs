mod framebuffer;
mod render;
mod maze;
mod player;
mod caster;
mod events;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use std::f32::consts::PI;
use std::{num, thread};
use std::time::Duration;
use maze::*;
use caster::cast_ray;  
use render::*;
use events::process_events;

fn main() {
    let window_width: i32 = 800;
    let window_height: i32 = 600;
    
    let framebuffer_width = 800;
    let framebuffer_height: u32 = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);

    let maze: Maze = load_maze("src/maze.txt"); 

    let block_size = window_width as usize / maze[0].len();
    let mut player = Player::new(Vector2::new(0.0, 0.0), PI/4.0, 5.0, 0.15, PI/3.0);
    player.set_initial_position(&maze);
    let num_rays = 5; // Number of rays to cast



    while !window.window_should_close() {
        framebuffer.clear();
        process_events(&mut window, &mut player, &maze, block_size);
        render_maze(&mut framebuffer, &maze, block_size);
        render_player(&mut framebuffer, &mut player);

        for i in 0..num_rays {
            let angle_offset = (i as f32 - (num_rays as f32 / 2.0)) * (player.fov / num_rays as f32);
            let ray_angle = player.angle + angle_offset;
            cast_ray(&mut framebuffer, &Player { angle: ray_angle, ..player }, &maze, block_size);
        }

    
        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));

    }
    
}
