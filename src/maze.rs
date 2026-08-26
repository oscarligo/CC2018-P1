use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(file_path: &str) -> Maze {
    let file = File::open(file_path).expect("Failed to open maze file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
    player: &mut Player 
) {
    let x = xo ;
    let y = yo ;

    match cell {
        '+' => {
            // Draw wall
            framebuffer.set_current_color(Color::DARKGREEN);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        ' ' => {
            // Draw empty space
        }
        '|' => {
            // Handle vertical walls
            framebuffer.set_current_color(Color::GREEN);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        '-' => {
            // Handle horizontal walls  
            framebuffer.set_current_color(Color::GREEN);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        'p' => {
            // Handle player starting position
            player.set_position(Vector2::new((x + block_size / 2) as f32, (y + block_size / 2) as f32));
        }

    
        _ => {
            // Handle other characters 
        }
    }


}

pub fn render_maze(
    framebuffer: &mut Framebuffer, 
    maze: &Maze,
    player: &mut Player,    
    block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell, player);
        }
    }
}