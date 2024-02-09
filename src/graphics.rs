use std::f32::consts::PI;

use glam::{vec3, Mat4, Vec2, Vec3};

use crate::{color::Color, object::Object, renderer::{DrawCall, Mode, Primitive, RendererData}};

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

pub trait Camera {
    fn view_proj(&self) -> Mat4;
    fn mode(&self) -> Mode;
}

pub struct Camera3d {
    transform: Mat4,
    projection: Mat4,
}

impl Camera for Camera3d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform.inverse()
    }

    fn mode(&self) -> Mode {
        Mode::Mode3d
    }
}

impl Camera3d {
    pub fn new() -> Self {
        let transform = Mat4::from_translation(vec3(0.0, 0.0, 5.0));
        let projection = Mat4::perspective_rh_gl(PI / 4.0, 320.0 / 200.0, 0.01, 100.0);

        Self {
            transform,
            projection,
        }
    }

    pub fn with_transform(mut self, transform: &Mat4) -> Self {
        self.transform = *transform;
        self
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform = Mat4::from_translation(translation) * self.transform;
        self
    }

    pub fn with_rotation_x(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_x(angle) * self.transform;
        self
    }

    pub fn with_rotation_y(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_y(angle) * self.transform;
        self
    }

    pub fn with_rotation_z(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_z(angle) * self.transform;
        self
    }

    pub fn with_scale(&mut self, scale: Vec3) -> &mut Self {
        self.transform = Mat4::from_scale(scale) * self.transform;
        self
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.transform *= Mat4::from_translation(translation);
        self
    }

    pub fn rotate_x(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_x(angle);
        self
    }

    pub fn rotate_y(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_y(angle);
        self
    }

    pub fn rotate_z(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_z(angle);
        self
    }

    pub fn scale(&mut self, scale: Vec3) -> &mut Self {
        self.transform *= Mat4::from_scale(scale);
        self
    }
}

pub struct Camera2d {
    transform: Mat4,
    projection: Mat4,
}

impl Camera for Camera2d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform
    }

    fn mode(&self) -> Mode {
        Mode::Mode2d
    }
}

impl Camera2d {
    pub fn new() -> Self {
        let transform = Mat4::IDENTITY;
        let projection = Mat4::IDENTITY;

        Self {
            transform,
            projection,
        }
    }
}

pub struct Graphics<'a> {
    pub data: &'a mut RendererData,
}

impl<'a> Graphics<'a> {
    pub fn set_camera(self, camera: &dyn Camera) -> Self {
        self.data.view_proj = camera.view_proj();
        self.data.mode = camera.mode();
        self
    }

    pub fn draw_object(self, object: &Object) -> Self {
        let mode = self.data.mode;
        self.new_draw_call(
            object.vertices(),
            object.indices(),
            object.transform(),
            Primitive::Triangles,
            mode,
        )
    }

    pub fn draw_line(self, p1: Vec2, p2: Vec2, color: Color) -> Self {
        let mode = self.data.mode;
        self.new_draw_call(
            &vec![
                Vertex {
                    position: [p1.x, p1.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [p2.x, p2.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
            ], 
            &vec![
                0, 1,
            ], 
            &Mat4::IDENTITY,
            Primitive::Lines,
            mode,
        )
    }

    pub fn draw_rectangle(self, position: Vec2, size: Vec2, color: Color) -> Self {
        let mode = self.data.mode;
        self.new_draw_call(
            &vec![
                Vertex {
                    position: [position.x, position.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x + size.x, position.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x, position.y + size.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x + size.x, position.y + size.y, 0.0],
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
            ], 
            &vec![
                0, 1, 2, 1, 2, 3,
            ], 
            &Mat4::IDENTITY,
            Primitive::Triangles,
            mode,
        )
    }

    fn new_draw_call(self, vertices: &Vec<Vertex>, indices: &Vec<i32>, transform: &Mat4, primitive: Primitive, mode: Mode) -> Self {
        let previous_dc = if self.data.draw_calls_count == 0 {
            None
        } else {
            self.data.draw_calls.get(self.data.draw_calls_count - 1)
        }
        ;
        if previous_dc.map_or(true, |draw_call| {
            draw_call.model != *transform ||
            draw_call.view_proj != self.data.view_proj ||
            draw_call.primitive != primitive ||
            draw_call.mode != mode
        }) {
            // start a new draw call
            if self.data.draw_calls.len() <= self.data.draw_calls_count {
                // brand new draw call
                self.data.draw_calls.push(
                    DrawCall {
                        vertices: vertices.clone(),
                        indices: indices.clone(),
                        model: *transform,
                        view_proj: self.data.view_proj,
                        primitive,
                        mode,
                    }
                );
            } else {
                // reuse empty draw call
                self.data.draw_calls[self.data.draw_calls_count].vertices = vertices.clone();
                self.data.draw_calls[self.data.draw_calls_count].indices = indices.clone();
                self.data.draw_calls[self.data.draw_calls_count].model = *transform;
                self.data.draw_calls[self.data.draw_calls_count].view_proj = self.data.view_proj;
                self.data.draw_calls[self.data.draw_calls_count].primitive = primitive;
                self.data.draw_calls[self.data.draw_calls_count].mode = mode;
            }
    
            self.data.draw_calls_count += 1;
        } else {
            // complete existing draw call
            self.data.draw_calls[self.data.draw_calls_count].vertices.append(&mut vertices.clone());
            self.data.draw_calls[self.data.draw_calls_count].indices.append(&mut indices.clone());
        }
        
        self
    }
}