use raylib::prelude::*;

pub struct TextureManager {
    log: Image,
    cobblestone: Image,
    enemy_frames: [Image; 2],
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            log: Image::load_image("assets/oak_log_wall.png")
                .expect("Failed to load oak log texture"),
            cobblestone: Image::load_image("assets/cobblestone_wall.png")
                .expect("Failed to load cobblestone texture"),
            enemy_frames: [
                Image::load_image("assets/oak_log_block.png")
                    .expect("Failed to load first enemy frame"),
                Image::load_image("assets/cobblestone_block.png")
                    .expect("Failed to load second enemy frame"),
            ],
        }
    }

    pub fn get_pixel_color(&self, ch: char, u: f32, v: f32) -> Color {
        let image = match ch {
            '+' => &self.log,
            '-' | '|' | 'g' | '#' => &self.cobblestone,
            _ => return Color::WHITE,
        };
        sample(image, u, v)
    }

    pub fn get_sprite_pixel(&self, ch: char, frame: usize, u: f32, v: f32) -> Color {
        let image = match ch {
            'e' => &self.enemy_frames[frame % 2],
            _ => return Color::BLANK,
        };
        sample(image, u, v)
    }
}

fn sample(image: &Image, u: f32, v: f32) -> Color {
        let x = ((u.clamp(0.0, 1.0) * image.width() as f32) as i32).min(image.width() - 1);
        let y = ((v.clamp(0.0, 1.0) * image.height() as f32) as i32).min(image.height() - 1);
        image.get_color(x, y)
}
