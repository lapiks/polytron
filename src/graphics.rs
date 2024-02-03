use std::f32::consts::PI;

use glam::{vec3, Mat4, Vec2};

use crate::{object::Object, renderer::{DrawCall, RendererData}};

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy)]
pub enum Primitive {
    Unknown,
    Points,
    Lines,
    LineStrips,
    LineLoops,
    Triangles,
    TriangleStrips,
    TriangleFans,
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive::Unknown
    }
}

pub struct Camera {
    transform: Mat4,
    projection: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        let transform = Mat4::from_translation(vec3(0.0, 0.0, 5.0));
        let projection = Mat4::perspective_rh_gl(PI / 4.0, 320.0 / 200.0, 0.01, 100.0);

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
    pub fn set_camera(self, camera: &Camera) -> Self {
        self.data.view_proj = camera.projection * camera.transform.inverse();
        self
    }

    pub fn draw(self, object: &Object) -> Self {
        self.new_draw_call(
            object.vertices(),
            object.indices(),
            object.transform()
        )
    }

    pub fn draw_rectangle(self, position: Vec2, size: Vec2) -> Self {
        self.new_draw_call(
            &vec![
                Vertex {
                    position: [position.x, position.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x + size.x, position.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x, position.y + size.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [position.x + size.x, position.y + size.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
            ], 
            &vec![
                0, 1, 2, 1, 2, 3,
            ], 
            &Mat4::IDENTITY,
        )
    }

    fn new_draw_call(self, vertices: &Vec<Vertex>, indices: &Vec<i32>, transform: &Mat4) -> Self{
        if self.data.draw_calls.len() <= self.data.draw_calls_count {
            self.data.draw_calls.push(
                DrawCall {
                    vertices: vertices.clone(),
                    indices: indices.clone(),
                    model: *transform,
                }
            );
        } else {
            self.data.draw_calls[self.data.draw_calls_count].vertices = vertices.clone();
            self.data.draw_calls[self.data.draw_calls_count].indices = indices.clone();
            self.data.draw_calls[self.data.draw_calls_count].model = *transform;
        }

        self.data.draw_calls_count += 1;
        self
    }
}