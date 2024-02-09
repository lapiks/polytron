use std::f32::consts::PI;

use glam::{vec2, vec3};

use crate::{color::Color, console::System, graphics::{Camera2d, Camera3d, Graphics}, object::Object, time::TimeStep};

pub struct Game {
    time_step: TimeStep,
    camera_3d: Camera3d,
    camera_2d: Camera2d,
    cube: Object,
    plane: Object,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            time_step: Default::default(),
            camera_3d: Camera3d::new(),
            camera_2d: Camera2d::new(),
            cube: Object::new_cube(Color::red()),
            plane: Object::new_plane(Color::white()),
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
        self.plane
        .translate(vec3(0.0, -1.0, 0.0))
        .scale(vec3(10.0, 10.0, 10.0));
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
        .draw_object(&self.cube)
        .draw_object(&self.plane)
        .set_camera(&self.camera_2d)
        .draw_line(vec2(-1.0, -1.0), vec2(1.0, 1.0), Color::green())
        .draw_rectangle(vec2(-1.0, -1.0), vec2(0.5, 0.25), Color::gray());
    }

    fn key_down(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, _repeat: bool) {
        let dt = self.time_step.delta_time();
        let speed = 5.0;
        self.camera_3d.translate( 
            match keycode {
                miniquad::KeyCode::W => vec3(0.0, 0.0, 1.0),
                miniquad::KeyCode::S => vec3(0.0, 0.0, -1.0),
                miniquad::KeyCode::D => vec3(1.0, 0.0, 0.0),
                miniquad::KeyCode::A => vec3(-1.0, 0.0, 0.0),
                _ => vec3(0.0, 0.0, 0.0)
            } 
            * dt
            * speed
        );
    }
}