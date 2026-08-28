mod framebuffer;
mod render;
mod maze;
mod player;
mod caster;
mod events;
mod texture_manager;
mod enemy;
mod screens;
mod portal;

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
use screens::*;
use portal::Portal;

#[derive(Clone, Copy)]
enum Screen {
    Main,
    Worlds(usize),
    Game,
}

const WORLD_MAPS: [&str; 2] = ["src/world_1.txt", "src/world_2.txt"];

fn main() {
    let window_width: i32 = 1080;
    let window_height: i32 = 600;
    
    let framebuffer_width = window_width as u32;
    let framebuffer_height: u32 = window_height as u32;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);

    let mut maze: Maze = load_maze(WORLD_MAPS[0]);

    let block_size = 10;
    let player_movement_speed = block_size as f32 / 5.0;
    let player_rotation_speed = block_size as f32 / 50.0;
    let mut player = Player::new(Vector2::new(0.0, 0.0), PI/4.0, player_movement_speed, player_rotation_speed, PI/3.0);
    player.set_initial_position(&maze, block_size);
    let num_rays = 5; 

    let texture_manager = TextureManager::new();
    let mut enemies = Enemy::from_maze(&maze, block_size);
    let mut portals = Portal::from_maze(&maze, block_size);
    let mut current_world = 0;
    let main_screen = window.load_texture(&thread, "assets/main_screen.png")
        .expect("Failed to load main screen");
    let world_screens = [
        window.load_texture(&thread, "assets/world_1.png")
            .expect("Failed to load world 1 screen"),
        window.load_texture(&thread, "assets/world_2.png")
            .expect("Failed to load world 2 screen"),
    ];
    let mut screen = Screen::Main;



    while !window.window_should_close() {
        match screen {
            Screen::Main => {
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    screen = Screen::Worlds(0);
                }
                draw_screen(&mut window, &thread, &main_screen, window_width, window_height);
            }
            Screen::Worlds(mut selected) => {
                if window.is_key_pressed(KeyboardKey::KEY_W)
                    || window.is_key_pressed(KeyboardKey::KEY_S)
                {
                    selected = next_world(selected);
                }

                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_world = selected;
                    maze = load_maze(WORLD_MAPS[selected]);
                    player.set_initial_position(&maze, block_size);
                    enemies = Enemy::from_maze(&maze, block_size);
                    portals = Portal::from_maze(&maze, block_size);
                    window.disable_cursor();
                    screen = Screen::Game;
                } else {
                    screen = Screen::Worlds(selected);
                }

                draw_screen(
                    &mut window,
                    &thread,
                    &world_screens[selected],
                    window_width,
                    window_height,
                );
            }
            Screen::Game => {
                framebuffer.clear();
                process_events(&mut window, &mut player, &maze, block_size);
                if player_on_marker(&player, &maze, block_size, 'w') {
                    current_world = next_world(current_world);
                    maze = load_maze(WORLD_MAPS[current_world]);
                    player.set_initial_position(&maze, block_size);
                    enemies = Enemy::from_maze(&maze, block_size);
                    portals = Portal::from_maze(&maze, block_size);
                }

                let delta_time = window.get_frame_time();
                enemies.iter_mut().for_each(|enemy| enemy.update(delta_time));
                portals.iter_mut().for_each(|portal| portal.update(delta_time));
                let depth_buffer = render::render3d(
                    &mut framebuffer,
                    &player,
                    &maze,
                    block_size,
                    current_world,
                    &texture_manager,
                );
                render::render_sprites(
                    &mut framebuffer,
                    &player,
                    &enemies,
                    &portals,
                    block_size,
                    &depth_buffer,
                    &texture_manager,
                );

                render_maze(&mut framebuffer, &maze, block_size);
                render_player(&mut framebuffer, &mut player);

                for i in 0..num_rays {
                    let current_ray = i as f32 / num_rays as f32;
                    let angle_offset = (current_ray - 0.5) * player.fov;
                    cast_ray(
                        &mut framebuffer,
                        &player,
                        &maze,
                        block_size,
                        angle_offset,
                        true,
                        8.0,
                    );
                }

                framebuffer.swap_buffers(&mut window, &thread);
            }
        }
        thread::sleep(Duration::from_millis(16));
    }
}

fn player_on_marker(player: &Player, maze: &Maze, block_size: usize, marker: char) -> bool {
    maze.get(player.position.y as usize / block_size)
        .and_then(|row| row.get(player.position.x as usize / block_size))
        == Some(&marker)
}
