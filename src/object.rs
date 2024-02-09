use glam::{Mat4, Vec3};

use crate::{color::Color, graphics::Vertex, shapes::{Cube, Plane}};

pub struct Object {
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
    transform: Mat4,
}

impl Default for Object {
    fn default() -> Self {
        Self { 
            vertices: Default::default(), 
            indices: Default::default(), 
            transform: Default::default() 
        }
    }
}

impl Object {
    pub fn new_mesh(vertices: Vec<Vertex>, indices: Vec<i32>) -> Self {
        Self {
            vertices,
            indices,
            ..Default::default()
        }
    }

    pub fn new_cube(color: Color) -> Self {
        Self {
            vertices: Cube::vertices(color),
            indices: Cube::indices(),
            transform: Mat4::IDENTITY,
        }
    }

    pub fn new_plane(color: Color) -> Self {
        Self {
            vertices: Plane::vertices(color),
            indices: Plane::indices(),
            transform: Mat4::IDENTITY,
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

    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<i32> {
        &self.indices
    }
}

