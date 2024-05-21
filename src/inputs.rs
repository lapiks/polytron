use std::collections::HashMap;

use glam::Vec2;

pub struct Inputs {
    keys: HashMap<miniquad::KeyCode, bool>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.mouse_delta = Vec2::ZERO;
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn key(&self, keycode: miniquad::KeyCode) -> bool {
        match self.keys.get(&keycode) {
            Some(b) => *b,
            None => false,
        }
    }

    pub fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.mouse_delta.x = x - self.mouse_position.x;
        self.mouse_delta.y = y - self.mouse_position.y;
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }

    pub fn key_down_event(&mut self, keycode: miniquad::KeyCode) {
        self.keys.insert(keycode, true);
    }

    pub fn key_up_event(&mut self, keycode: miniquad::KeyCode) {
        self.keys.insert(keycode, false);
    }
}