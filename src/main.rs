mod framebuffer;
mod render;
mod maze;
mod player;
mod caster;
mod events;
mod texture_manager;
mod enemy;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use std::f32::consts::PI;
use std::{thread};
use std::time::Duration;
use maze::*;
use caster::cast_ray;  
use render::*;
use events::process_events;
use texture_manager::TextureManager;
use enemy::Enemy;

fn main() {
    let window_width: i32 = 800;
    let window_height: i32 = 600;
    
    let framebuffer_width = window_width as u32;
    let framebuffer_height: u32 = window_height as u32;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .build();
    window.disable_cursor();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);

    let maze: Maze = load_maze("src/maze.txt"); 

    let block_size = 10;
    let player_movement_speed = block_size as f32 / 5.0;
    let player_rotation_speed = block_size as f32 / 50.0;
    let mut player = Player::new(Vector2::new(0.0, 0.0), PI/4.0, player_movement_speed, player_rotation_speed, PI/3.0);
    player.set_initial_position(&maze, block_size);
    let num_rays = 5; 

    let texture_manager = TextureManager::new();
    let mut enemies = Enemy::from_maze(&maze, block_size);



    while !window.window_should_close() {
        framebuffer.clear();

        process_events(&mut window, &mut player, &maze, block_size);
        enemies.iter_mut().for_each(|enemy| enemy.update(window.get_frame_time()));
        let depth_buffer = render::render3d(&mut framebuffer, &player, &maze, block_size, &texture_manager);
        render::render_enemies(
            &mut framebuffer,
            &player,
            &enemies,
            block_size,
            &depth_buffer,
            &texture_manager,
        );

        render_maze(&mut framebuffer, &maze, block_size);
        render_player(&mut framebuffer, &mut player);

        for i in 0..num_rays {
            let current_ray = i as f32 / num_rays as f32;
            let angle_offset = (current_ray - 0.5) * player.fov;
            cast_ray(&mut framebuffer,  &player, &maze,  block_size, angle_offset,true, 8.0);
        }

    
        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));

    }
    
}
