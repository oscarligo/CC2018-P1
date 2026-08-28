use raylib::prelude::*;

pub struct TextureManager {
    wall_1: Image,
    wall_2: Image,
    sprite: Image,
    floor: Image,
    portal_frames: [Image; 5],
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            wall_1: Image::load_image("assets/oak_log_wall.png")
                .expect("Failed to load oak log texture"),
            wall_2: Image::load_image("assets/cobblestone_wall.png")
                .expect("Failed to load cobblestone texture"),
            sprite: Image::load_image("assets/oak_log_block.png")
                .expect("Failed to load first enemy frame"),
            floor: Image::load_image("assets/cobblestone_wall.png")
                .expect("Failed to load floor texture"),
            portal_frames: [
                Image::load_image("assets/nether_portal/portal_1.png").expect("Failed to load portal frame 1"),
                Image::load_image("assets/nether_portal/portal_2.png").expect("Failed to load portal frame 2"),
                Image::load_image("assets/nether_portal/portal_3.png").expect("Failed to load portal frame 3"),
                Image::load_image("assets/nether_portal/portal_4.png").expect("Failed to load portal frame 4"),
                Image::load_image("assets/nether_portal/portal_5.png").expect("Failed to load portal frame 5"),
            ],
        }
    }

    pub fn get_pixel_color(&self, ch: char, u: f32, v: f32) -> Color {
        let image = match ch {
            '+' => &self.wall_1,
            '-' | '|' | 'g' | '#' => &self.wall_2,
            _ => return Color::WHITE,
        };
        sample(image, u, v)
    }

    pub fn get_sprite_pixel(&self, ch: char, frame: usize, u: f32, v: f32) -> Color {
        let image = match ch {
            'e' if frame % 2 == 0 => &self.sprite,
            'e' => &self.floor,
            'w' => &self.portal_frames[frame % 5],
            _ => return Color::BLANK,
        };
        sample(image, u, v)
    }

    pub fn get_floor_pixel(&self, u: f32, v: f32) -> Color {
        sample(&self.floor, u, v)
    }

    pub fn get_ceiling_pixel(&self, u: f32, v: f32) -> Color {
        sample(&self.wall_1, u, v)
    }
}

fn sample(image: &Image, u: f32, v: f32) -> Color {
        let x = ((u.clamp(0.0, 1.0) * image.width() as f32) as i32).min(image.width() - 1);
        let y = ((v.clamp(0.0, 1.0) * image.height() as f32) as i32).min(image.height() - 1);
        image.get_color(x, y)
}
