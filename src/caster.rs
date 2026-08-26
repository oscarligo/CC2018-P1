use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    block_size: usize,
) {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::WHITESMOKE);

    loop{
        let cos = d * player.angle.cos();
        let sin = d * player.angle.sin();
        
        let x = (player.position.x + cos) as usize;
        let y = (player.position.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' && maze[j][i] != 'p' {
            break;  
        }

        framebuffer.set_pixel(x as u32, y as u32);

        d += 10.0;
    }
    
}