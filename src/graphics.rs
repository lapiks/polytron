use glam::{Vec2, Vec3, Vec4};

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy)]
pub enum Mode {
    Unknown,
    Points,
    Lines,
    LineStrips,
    LineLoops,
    Triangles,
    TriangleStrips,
    TriangleFans,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Unknown
    }
}

pub struct Shape {
    pub mode: Mode,
    pub vertices: Vec<Vertex>,
}

/// The console graphics API
pub struct Graphics {
    current_mode: Mode,
    current_color: Vec4,
    current_normal: Vec3,
    vertices: Vec<Vertex>,
    pub shapes: Vec<Shape>,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            current_mode: Default::default(),
            current_color: Default::default(),
            current_normal: Default::default(),
            vertices: Default::default(),
            shapes: Default::default(),
        }
    }
}

impl Graphics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn begin(mut self, mode: Mode) -> Self {
        self.current_mode = mode;
        self
    }

    pub fn end(mut self) -> Self {
        self.shapes.push(
            Shape {
                mode: self.current_mode,
                vertices: self.vertices.clone()
            }
        );

        // reset state
        self.current_mode = Default::default();
        self.current_color = Default::default();
        self.current_normal = Default::default();
        self.vertices.clear();

        self
    }

    pub fn vertex2(mut self, position: Vec2) -> Self {
        self.vertices.push(
            Vertex {
                position: Vec3::new(position.x, position.y, 0.0).to_array(),
                color: self.current_color.to_array(),
                normal: self.current_normal.to_array()
            }
        );
        self
    }

    pub fn vertex3(mut self, position: Vec3) -> Self{
        self.vertices.push(
            Vertex {
                position: position.to_array(),
                color: self.current_color.to_array(),
                normal: self.current_normal.to_array()
            }
        );
        self
    }
    
    pub fn color3(mut self, color: Vec3) -> Self {
        self.current_color = Vec4::new(color.x, color.y, color.z, 1.0);
        self
    }

    pub fn color4(mut self, color: Vec4) -> Self {
        self.current_color = color;
        self
    }

    pub fn normal(mut self, normal: Vec3) -> Self {
        self.current_normal = normal;
        self
    }
}