use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(Color::BLUE);
    framebuffer.set_pixel(player.position.x as u32, player.position.y as u32);
}


