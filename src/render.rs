use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{Maze, render_maze};

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(Color::BLUE);
    framebuffer.set_pixel(player.position.x as u32, player.position.y as u32);
}




pub fn render_2d(
    framebuffer: &mut Framebuffer,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
) {
    render_maze(framebuffer, maze,  player, block_size);
    render_player(framebuffer, player);
    

}

