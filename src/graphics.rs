use std::f32::consts::PI;

use glam::{vec3, Mat4, Vec2};

use crate::{object::Object, renderer::{DrawCall, Primitive, RendererData}};

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

pub trait Camera {
    fn view_proj(&self) -> Mat4;
}

pub struct Camera3d {
    transform: Mat4,
    projection: Mat4,
}

impl Camera for Camera3d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform.inverse()
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
}

pub struct Camera2d {
    transform: Mat4,
    projection: Mat4,
}

impl Camera for Camera2d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform
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
        self
    }

    pub fn draw(self, object: &Object) -> Self {
        self.new_draw_call(
            object.vertices(),
            object.indices(),
            object.transform(),
            Primitive::Triangles,
        )
    }

    pub fn draw_line(self, p1: Vec2, p2: Vec2) -> Self {
        self.new_draw_call(
            &vec![
                Vertex {
                    position: [p1.x, p1.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [p2.x, p2.y, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
            ], 
            &vec![
                0, 1,
            ], 
            &Mat4::IDENTITY,
            Primitive::Lines,
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
            Primitive::Triangles,
        )
    }

    fn new_draw_call(self, vertices: &Vec<Vertex>, indices: &Vec<i32>, transform: &Mat4, primitive: Primitive) -> Self{
        if self.data.draw_calls.len() <= self.data.draw_calls_count {
            self.data.draw_calls.push(
                DrawCall {
                    vertices: vertices.clone(),
                    indices: indices.clone(),
                    model: *transform,
                    view_proj: self.data.view_proj,
                    primitive,
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