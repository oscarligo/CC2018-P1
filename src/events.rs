use crate::player::Player;
use raylib::prelude::*;

pub fn process_events(window: &RaylibHandle, player: &mut Player) {
    if window.is_key_down(KeyboardKey::KEY_W) {
        player.move_forward(player.move_speed);
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        player.move_backward(player.move_speed);
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        player.rotate_left(player.rotate_speed);
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        player.rotate_right(player.rotate_speed);
    }
}   