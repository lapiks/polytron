use std::f32::consts::PI;

use glam::{vec2, vec3};

use crate::{color::Color, console::System, graphics::{Camera2d, Camera3d, Graphics, Rect2d}, inputs::Inputs, object::Object, time::TimeStep};

pub struct Game {
    time_step: TimeStep,
    camera_3d: Camera3d,
    camera_2d: Camera2d,
    cube: Object,
    plane: Object,
}

impl Default for Game {
    fn default() -> Self {
        let camera_3d = Camera3d::new()
        .with_viewport(&Rect2d {position: vec2(0.0, 0.0), size: vec2(0.5, 1.0)});
        let camera_2d = Camera2d::new()
        .with_viewport(&Rect2d {position: vec2(0.5, 0.0), size: vec2(0.5, 1.0)})
        .with_background(Color::new(0.1, 0.1, 0.1, 1.0));
        Self { 
            time_step: Default::default(),
            camera_3d,
            camera_2d,
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

    fn update(&mut self, inputs: &Inputs) {
        let dt = self.time_step.tick();
        self.cube
        .rotate_z((PI / 4.0) * dt)
        .rotate_y((PI / 4.0) * dt);

        let dt = self.time_step.delta_time();
        let speed = 5.0;
        self.camera_3d.translate( 
            if inputs.key(miniquad::KeyCode::W) {
                vec3(0.0, 0.0, 1.0)
            } else if inputs.key(miniquad::KeyCode::S) {
                vec3(0.0, 0.0, -1.0)
            } else if inputs.key(miniquad::KeyCode::D) {
                vec3(1.0, 0.0, 0.0)
            } else if inputs.key(miniquad::KeyCode::A) {
                vec3(-1.0, 0.0, 0.0)
            }else {
                vec3(0.0, 0.0, 0.0)
            }
            * dt
            * speed
        );
    }

    fn draw(&self, g: &mut Graphics) {
        g
        .set_camera(&self.camera_3d);

        for x in -5..6 {
            g.draw_line(
                vec3(x as f32, -1.0, -5.0), 
                vec3(x as f32, -1.0, 5.0), 
                Color::white()
            );
        }
        for z in -5..6 {
            g.draw_line(
                vec3(-5.0, -1.0, z as f32), 
                vec3(5.0, -1.0, z as f32), 
                Color::white()
            );
        }

        g.draw_object(&self.cube)
        .draw_rectangle(vec2(-1.0, -1.0), vec2(2.0, 2.0), Color::blue())
        .draw_line(vec3(-1.0, -1.0, 0.0), vec3(1.0, 1.0, 0.0), Color::green())
        .set_camera(&self.camera_2d)
        .draw_line(vec3(-1.0, -1.0, 0.0), vec3(1.0, 1.0, 0.0), Color::green())
        .draw_rectangle(vec2(-1.0, -1.0), vec2(0.5, 0.25), Color::gray());
    }

    fn key_down(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, _repeat: bool) {
        
    }
}