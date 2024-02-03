use crate::graphics::Vertex;

pub struct Cube {}

impl Cube {
    pub fn vertices() -> Vec<Vertex> {
        vec![
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0],
            },
        ]
    }

    pub fn indices() -> Vec<i32> {
        vec![
            0, 1, 2, 
            1, 3, 2,
            0, 5, 1,
            0, 4, 5,
            2, 7, 3,
            2, 6, 7,
            5, 7, 6,
            4, 5, 6,
            1, 3, 7,
            5, 1, 7,
            0, 2, 6,
            0, 6, 4,
        ]
    }
}
