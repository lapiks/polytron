use glam::{Vec2, Vec3, Vec4, Mat4};

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

#[derive(Clone, Copy)]
pub enum MatrixMode {
    Model,
    View,
    Projection,
}

impl Default for MatrixMode {
    fn default() -> Self {
        MatrixMode::Model
    }
}

pub struct Shape {
    pub primitive: Primitive,
    pub vertices: Vec<Vertex>,
    pub mvp: Mat4,
}

/// The console graphics API
pub struct Graphics {
    current_matrix_mode: MatrixMode,
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    current_primitive: Primitive,
    current_color: Vec4,
    current_normal: Vec3,
    vertices: Vec<Vertex>,
    pub shapes: Vec<Shape>,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            current_matrix_mode: MatrixMode::default(),
            model: Default::default(),
            view: Default::default(),
            projection: Default::default(),
            current_primitive: Default::default(),
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

    pub fn begin(mut self, primitive: Primitive) -> Self {
        self.current_primitive = primitive;
        self
    }

    pub fn end(mut self) -> Self {
        self.shapes.push(
            Shape {
                primitive: self.current_primitive,
                vertices: self.vertices.clone(),
                mvp: self.projection * self.view * self.model,
            }
        );

        // reset state
        self.current_primitive = Default::default();
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

    pub fn vertex3(mut self, position: Vec3) -> Self {
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

    pub fn matrix_mode(mut self, mode: MatrixMode) -> Self {
        self.current_matrix_mode = mode;
        self
    }

    pub fn translate(mut self, translation: Vec3) -> Self {
        let matrix = Mat4::from_translation(translation);
        match self.current_matrix_mode {
            MatrixMode::Model => self.model *= matrix,
            MatrixMode::View => self.view *= matrix,
            MatrixMode::Projection => self.projection *= matrix,
        }
        self
    }

    pub fn rotate(mut self, angle: f32, axis: Vec3) -> Self {
        let matrix = Mat4::from_axis_angle(axis, angle);
        match self.current_matrix_mode {
            MatrixMode::Model => self.model *= matrix,
            MatrixMode::View => self.view *= matrix,
            MatrixMode::Projection => self.projection *= matrix,
        }
        self
    }

    pub fn scale(mut self, scale: Vec3) -> Self {
        let matrix = Mat4::from_scale(scale);
        match self.current_matrix_mode {
            MatrixMode::Model => self.model *= matrix,
            MatrixMode::View => self.view *= matrix,
            MatrixMode::Projection => self.projection *= matrix,
        }
        self
    }
}