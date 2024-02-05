use std::f32::consts::PI;

use glam::vec2;

use crate::{color::Color, console::System, graphics::{Camera2d, Camera3d, Graphics}, object::Object, time::TimeStep};

pub struct Game {
    time_step: TimeStep,
    camera_3d: Camera3d,
    camera_2d: Camera2d,
    cube: Object,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            time_step: Default::default() ,
            camera_3d: Camera3d::new(),
            camera_2d: Camera2d::new(),
            cube: Object::new_cube(Color::red()),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}

impl System for Game {
    fn init(&mut self) {
        
    }

    fn update(&mut self) {
        let dt = self.time_step.tick();
        self.cube
        .rotate_z((PI / 4.0) * dt)
        .rotate_y((PI / 4.0) * dt);
    }

    fn draw(&self, g: Graphics) {
        g
        .set_camera(&self.camera_3d)
        .draw(&self.cube)
        .set_camera(&self.camera_2d)
        .draw_line(vec2(-1.0, -1.0), vec2(1.0, 1.0), Color::green())
        .draw_rectangle(vec2(-1.0, -1.0), vec2(0.5, 2.0), Color::gray());
    }
}