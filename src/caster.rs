use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub struct Intersec{
    pub distance: f32,
    pub impact: char,
    pub hit_x: f32,
    pub hit_y: f32,
}


pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    angle_offset: f32,
    draw_line: bool,
    cast_step: f32
) -> Intersec {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::WHITESMOKE);

    loop{
        let cos = d * (player.angle + angle_offset).cos();
        let sin = d * (player.angle + angle_offset).sin();
        
        let x = (player.position.x + cos) as usize;
        let y = (player.position.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' && maze[j][i] != 'p' {
            return Intersec {
                distance: d,
                impact: maze[j][i],
                hit_x: x as f32,
                hit_y: y as f32,
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += cast_step;
    }
    
}