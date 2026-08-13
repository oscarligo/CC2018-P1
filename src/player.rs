use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub position: Vector2,
    pub angle: f32,
}

impl Player {
    pub fn new(position: Vector2, angle: f32) -> Self {
        Player { position, angle }
    }
    pub fn move_forward(&mut self, distance: f32) {
        self.position.x += distance * self.angle.cos();
        self.position.y += distance * self.angle.sin();
    }

    pub fn move_backward(&mut self, distance: f32) {
        self.position.x -= distance * self.angle.cos();
        self.position.y -= distance * self.angle.sin();
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.angle -= angle;
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.angle += angle;
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }
}
