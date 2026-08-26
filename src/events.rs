use crate::player::Player;
use raylib::prelude::*;

pub fn process_events(window: &RaylibHandle, player: &mut Player) {
    if window.is_key_down(KeyboardKey::KEY_W) {
        player.move_forward();
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        player.move_backward();
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        player.rotate_left();
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        player.rotate_right();
    }
}   