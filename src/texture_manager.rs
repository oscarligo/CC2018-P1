use raylib::prelude::*;

pub struct TextureManager {
    log: Image,
    cobblestone: Image,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            log: Image::load_image("assets/oak_log_wall.png")
                .expect("Failed to load oak log texture"),
            cobblestone: Image::load_image("assets/cobblestone_wall.png")
                .expect("Failed to load cobblestone texture"),
        }
    }

    pub fn get_pixel_color(&self, ch: char, u: f32, v: f32) -> Color {
        let image = match ch {
            '+' => &self.log,
            '-' | '|' | 'g' | '#' => &self.cobblestone,
            _ => return Color::WHITE,
        };
        let x = ((u.clamp(0.0, 1.0) * image.width() as f32) as i32).min(image.width() - 1);
        let y = ((v.clamp(0.0, 1.0) * image.height() as f32) as i32).min(image.height() - 1);
        image.get_color(x, y)
    }
}
