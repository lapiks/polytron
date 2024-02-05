use crate::{color::Color, graphics::Vertex};

pub struct Cube {}

impl Cube {
    pub fn vertices(color: Color) -> Vec<Vertex> {
        let p0 = [-0.5, -0.5, -0.5];
        let p1 = [0.5, -0.5, -0.5];
        let p2 = [-0.5, 0.5, -0.5];
        let p3 = [0.5, 0.5, -0.5];
        let p4 = [-0.5, -0.5, 0.5];
        let p5 = [0.5, -0.5, 0.5];
        let p6 = [-0.5, 0.5, 0.5];
        let p7 = [0.5, 0.5, 0.5];


        vec![
            // face 1 -z
            Vertex {
                position: p0,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: p1,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: p2,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: p1,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: p3,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: p2,
                color: color.as_array(),
                normal: [0.0, 0.0, -1.0],
            },
            // face 2 +z
            Vertex {
                position: p4,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: p5,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: p5,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: p7,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [0.0, 0.0, 1.0],
            },
            // face 3 -x
            Vertex {
                position: p0,
                color: color.as_array(),
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: p2,
                color: color.as_array(),
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [-1.0, 0.0, -1.0],
            },
            Vertex {
                position: p4,
                color: color.as_array(),
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: p0,
                color: color.as_array(),
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [-1.0, 0.0, 0.0],
            },
            // face 4 +x
            Vertex {
                position: p1,
                color: color.as_array(),
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: p5,
                color: color.as_array(),
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: p7,
                color: color.as_array(),
                normal: [1.0, 0.0, -1.0],
            },
            Vertex {
                position: p1,
                color: color.as_array(),
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: p3,
                color: color.as_array(),
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: p7,
                color: color.as_array(),
                normal: [1.0, 0.0, 0.0],
            },
            // face 4 -y
            Vertex {
                position: p4,
                color: color.as_array(),
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: p5,
                color: color.as_array(),
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: p0,
                color: color.as_array(),
                normal: [0.0, -1.0, -1.0],
            },
            Vertex {
                position: p5,
                color: color.as_array(),
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: p1,
                color: color.as_array(),
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: p0,
                color: color.as_array(),
                normal: [0.0, -1.0, 0.0],
            },
            // face 4 +y
            Vertex {
                position: p2,
                color: color.as_array(),
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: p3,
                color: color.as_array(),
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [0.0, 1.0, -1.0],
            },
            Vertex {
                position: p3,
                color: color.as_array(),
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: p7,
                color: color.as_array(),
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: p6,
                color: color.as_array(),
                normal: [0.0, 1.0, 0.0],
            },
        ]
    }

    pub fn indices() -> Vec<i32> {
        (0..36).collect()
    }
}
