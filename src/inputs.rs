use std::collections::HashMap;

pub struct Inputs {
    keys: HashMap<miniquad::KeyCode, bool>,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
        }
    }

    pub fn key(&self, keycode: miniquad::KeyCode) -> bool {
        match self.keys.get(&keycode) {
            Some(b) => *b,
            None => false,
        }
    }

    pub fn key_down_event(&mut self, keycode: miniquad::KeyCode) {
        self.keys.insert(keycode, true);
    }

    pub fn key_up_event(&mut self, keycode: miniquad::KeyCode) {
        self.keys.insert(keycode, false);
    }
}