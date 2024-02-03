use std::f32::consts::PI;

use crate::{graphics::{Camera, Graphics}, object::Object, time::TimeStep};

pub struct Game {
    time_step: TimeStep,
    camera: Camera,
    cube: Object,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            time_step: Default::default() ,
            camera: Camera::new(),
            cube: Object::new_cube(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(&mut self) {
        
    }

    pub fn update(&mut self) {
        let dt = self.time_step.tick();
        self.cube
        .rotate_z((PI / 4.0) * dt)
        .rotate_y((PI / 4.0) * dt);
    }

    pub fn draw(&self, g: Graphics) {
        g
        .set_camera(&Camera::new())
        .draw(&self.cube);
    }
}