use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::*;
use crate::caster::cast_ray;
use crate::texture_manager::TextureManager;

pub fn render_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(Color::BLUE);
    framebuffer.set_pixel(player.position.x as u32, player.position.y as u32);
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    texture_manager: &TextureManager,
) {
    let num_rays = framebuffer.width;
    let height = framebuffer.height as f32;
    let half_height = height / 2.0;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle_offset = (current_ray - 0.5) * player.fov;
        let intersect = cast_ray(framebuffer, player, maze, block_size, angle_offset, false,0.5);

        let corrected_distance = intersect.distance * angle_offset.cos();
        let distance_to_wall = corrected_distance.max(0.1);

        let stake_height = (block_size as f32 * height) / distance_to_wall;
        let stake_top_raw = half_height - stake_height / 2.0;
        let stake_top = stake_top_raw.max(0.0).min(height) as u32;
        let stake_bottom = (half_height + stake_height / 2.0).max(0.0).min(height) as u32;
        let u = texture_u(
            intersect.hit_x,
            intersect.hit_y,
            intersect.vertical_hit,
            block_size as f32,
        );

        for y in stake_top..stake_bottom {
            let v = (y as f32 - stake_top_raw) / stake_height;
            let color = texture_manager.get_pixel_color(intersect.impact, u, v);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i as u32, y);
        }
    }
}

// Calculate the texture coordinate u based on the hit position and whether it was a vertical hit
fn texture_u(hit_x: f32, hit_y: f32, vertical_hit: bool, block_size: f32) -> f32 {
    let offset = if vertical_hit { hit_y } else { hit_x };
    offset.rem_euclid(block_size) / block_size
}
