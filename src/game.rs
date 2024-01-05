use glam::{vec3, vec2};

use crate::graphics::{Graphics, Mode};

pub struct Game {

}

impl Game {
    pub fn update(&mut self) {

    }

    pub fn draw(&self, g: Graphics) -> Graphics {
        g.begin(Mode::Triangles)
        .color3(vec3(1.0, 0.0, 0.0))
        .vertex2(vec2(-0.5, -0.5))
        .color3(vec3(0.0, 1.0, 0.0))
        .vertex2(vec2(0.5, -0.5))
        .color3(vec3(0.0, 0.0, 1.0))
        .vertex2(vec2(0.0, 0.5))
        .end()
    }
}