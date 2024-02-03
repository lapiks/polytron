use glam::{Mat4, Vec2};

use crate::renderer::{DrawCall, RendererData};

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

pub struct Shape {
    primitive: Primitive,
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            primitive: Primitive::Triangles,
            vertices: Vec::default(),
            indices: Vec::default(),
        }
    }
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_primitive(mut self, primitive: Primitive) -> Self {
        self.primitive = primitive;
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn with_indices(mut self, indices: Vec<i32>) -> Self {
        self.indices = indices;
        self
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    } 

    pub fn indices(&self) -> &Vec<i32> {
        &self.indices
    } 

    pub fn primitive(&self) -> Primitive {
        self.primitive
    } 
}

pub struct Graphics<'a> {
    pub data: &'a mut RendererData,
}

impl<'a> Graphics<'a> {
    pub fn draw(self, shape: &Shape, transform: Mat4) -> Self {
        self.new_draw_call(
            &shape.vertices, 
            &shape.indices, 
            transform
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
            Mat4::IDENTITY,
        )
    }

    pub fn new_draw_call(self, vertices: &Vec<Vertex>, indices: &Vec<i32>, transform: Mat4) -> Self{
        if self.data.draw_calls.len() <= self.data.draw_calls_count {
            self.data.draw_calls.push(
                DrawCall {
                    vertices: vertices.clone(),
                    indices: indices.clone(),
                    transform,
                }
            );
        } else {
            self.data.draw_calls[self.data.draw_calls_count].vertices = vertices.clone();
            self.data.draw_calls[self.data.draw_calls_count].indices = indices.clone();
            self.data.draw_calls[self.data.draw_calls_count].transform = transform;
        }

        self.data.draw_calls_count += 1;
        self
    }
}