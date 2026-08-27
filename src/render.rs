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
    texture_manager: &mut TextureManager
) {
    let num_rays = framebuffer.width;
    let height = framebuffer.height as f32;
    let half_height = height / 2.0;
    let texture_size = 16.0;

    framebuffer.set_current_color(Color::GRAY);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle_offset = (current_ray - 0.5) * player.fov;
        let intersect = cast_ray(framebuffer, player, maze, block_size, angle_offset, false,0.5);

        let corrected_distance = intersect.distance * angle_offset.cos();
        let distance_to_wall = corrected_distance.max(0.1);

        if intersect.impact == '+' {
            framebuffer.set_current_color(Color::DARKGRAY);
        } else {
            framebuffer.set_current_color(Color::GRAY);
        }

        let stake_height = (block_size as f32 * height) / distance_to_wall;

        let stake_top = (half_height - (stake_height / 2.0)).max(0.0).min(height) as u32;
        let stake_bottom = (half_height + (stake_height / 2.0)).max(0.0).min(height) as u32;

        let hit_offset = (intersect.hit_x % block_size as f32) + (intersect.hit_y % block_size as f32);
        let tx = (((hit_offset % block_size as f32) / block_size as f32) * texture_size as f32) as u32;

        for y in stake_top..stake_bottom {
            let ty = (((y  - stake_top ) / stake_height as u32) * texture_size as u32) as u32;
            
            let color = texture_manager.get_pixel_color(intersect.impact, tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i as u32, y);
        }
    }
}
