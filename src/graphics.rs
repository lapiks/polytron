use std::f32::consts::PI;

use glam::{vec2, vec3, Mat4, Vec2, Vec3};

use crate::{color::Color, object::Object, renderer::{DrawCall, Mode, Primitive, RendererData, IMAGE_RATIO_XY}};

#[derive(Clone, Copy, PartialEq)]
pub struct Rect2d {
    pub position: Vec2,
    pub size: Vec2,
}

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
    fn viewport(&self) -> &Rect2d;
    fn background(&self) -> Color;
}

pub struct Camera3d {
    transform: Mat4,
    projection: Mat4,
    viewport: Rect2d,
    background: Color,
}

impl Camera for Camera3d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform.inverse()
    }

    fn mode(&self) -> Mode {
        Mode::Mode3d
    }

    fn viewport(&self) -> &Rect2d {
        &self.viewport
    }

    fn background(&self) -> Color {
        self.background
    }
}

impl Camera3d {
    pub fn new() -> Self {
        let transform = Mat4::from_translation(vec3(0.0, 0.0, 5.0));
        let projection = Mat4::perspective_rh_gl(PI / 4.0, IMAGE_RATIO_XY, 0.01, 100.0);

        Self {
            transform,
            projection,
            viewport: Rect2d {
                position: vec2(0.0, 0.0),
                size: vec2(1.0, 1.0)
            },
            background: Color::black(),
        }
    }

    pub fn with_viewport(mut self, viewport: &Rect2d) -> Self {
        self.viewport = *viewport;
        self.projection = Mat4::perspective_rh_gl(
            PI / 4.0, 
            (viewport.size.x / viewport.size.y) * IMAGE_RATIO_XY, 
            0.01, 
            100.0
        );
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
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

    pub fn set_viewport(&mut self, viewport: &Rect2d) -> &mut Self {
        self.viewport = *viewport;
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
    viewport: Rect2d,
    background: Color,
}

impl Camera for Camera2d {
    fn view_proj(&self) -> Mat4 {
        self.projection * self.transform
    }

    fn mode(&self) -> Mode {
        Mode::Mode2d
    }

    fn viewport(&self) -> &Rect2d {
        &self.viewport
    }

    fn background(&self) -> Color {
        self.background
    }
}

impl Camera2d {
    pub fn new() -> Self {
        let transform = Mat4::IDENTITY;
        let projection = Mat4::IDENTITY;

        Self {
            transform,
            projection,
            viewport: Rect2d {
                position: vec2(0.0, 0.0),
                size: vec2(1.0, 1.0)
            },
            background: Color::black(),
        }
    }

    pub fn with_viewport(mut self, viewport: &Rect2d) -> Self {
        self.viewport = *viewport;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
}

pub struct Graphics<'a> {
    pub data: &'a mut RendererData,
}

impl<'a> Graphics<'a> {
    pub fn set_camera(&mut self, camera: &dyn Camera) -> &mut Self {
        self.data.view_proj = camera.view_proj();
        self.data.mode = camera.mode();
        self.data.viewport = *camera.viewport();
        self.data.background = camera.background();
        self
    }

    pub fn draw_object(&mut self, object: &Object) -> &mut Self {
        self.new_draw_call(
            object.vertices(),
            object.indices(),
            object.transform(),
            Primitive::Triangles,
        )
    }

    pub fn draw_line(&mut self, p1: Vec3, p2: Vec3, color: Color) -> &mut Self {
        self.new_draw_call(
            &vec![
                Vertex {
                    position: p1.to_array(),
                    color: color.as_array(),
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: p2.to_array(),
                    color: color.as_array(),
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

    pub fn draw_rectangle(&mut self, position: Vec2, size: Vec2, color: Color) -> &mut Self {
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
        )
    }

    fn new_draw_call(
        &mut self, 
        vertices: &Vec<Vertex>, 
        indices: &Vec<i32>, 
        transform: &Mat4, 
        primitive: Primitive, 
    ) -> &mut Self {
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
            draw_call.mode != self.data.mode ||
            draw_call.viewport != self.data.viewport
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
                        mode:  self.data.mode,
                        viewport: self.data.viewport,
                        background: self.data.background,
                    }
                );
            } else {
                // reuse empty draw call
                self.data.draw_calls[self.data.draw_calls_count].vertices = vertices.clone();
                self.data.draw_calls[self.data.draw_calls_count].indices = indices.clone();
                self.data.draw_calls[self.data.draw_calls_count].model = *transform;
                self.data.draw_calls[self.data.draw_calls_count].view_proj = self.data.view_proj;
                self.data.draw_calls[self.data.draw_calls_count].primitive = primitive;
                self.data.draw_calls[self.data.draw_calls_count].mode = self.data.mode;
                self.data.draw_calls[self.data.draw_calls_count].viewport = self.data.viewport;
                self.data.draw_calls[self.data.draw_calls_count].background = self.data.background;
            }
    
            self.data.draw_calls_count += 1;
        } else {
            let mut new_indices = indices.clone()
                .iter_mut()
                .map(|idx| {
                    *idx + self.data.draw_calls[self.data.draw_calls_count - 1].vertices.len() as i32
                })
                .collect();

            // complete existing draw call
            self.data.draw_calls[self.data.draw_calls_count - 1].indices.append(
                &mut new_indices
            );
            self.data.draw_calls[self.data.draw_calls_count - 1].vertices.append(&mut vertices.clone());

        }
        
        self
    }
}