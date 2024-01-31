use glam::Mat4;

use crate::{graphics::{Graphics, Shape, Vertex}, time::TimeStep};

pub struct Game {
    time_step: TimeStep,
}

impl Default for Game {
    fn default() -> Self {
        Self { time_step: Default::default() }
    }
}

impl Game {
    pub fn update(&mut self) {
        self.time_step.tick();
    }

    pub fn draw(&self, g: Graphics) -> Graphics {
        let shape = Shape::default()
        .with_vertices(vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
        ]);

        g.draw(&shape, Mat4::IDENTITY)
    }
}