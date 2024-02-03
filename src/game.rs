use glam::{vec2, vec3, Mat4};

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self) {
        self.time_step.tick();
    }

    pub fn draw(&self, g: Graphics) {
        let shape = Shape::default()
        .with_vertices(vec![
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0], 
                normal: [0.0, 0.0, 0.0],
            },
        ])
        .with_indices(vec![
            0, 1, 2, 2, 1, 3
        ]);

        g
        .draw(&shape, Mat4::IDENTITY)
        .draw(&shape, Mat4::from_translation(vec3(0.5, 0.0, 0.0)))
        .draw_rectangle(vec2(0.0, 0.0), vec2(0.5, 0.5));
    }
}