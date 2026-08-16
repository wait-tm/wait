use crate::renderer::vertex::Vertex;

pub(super) const CUBE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.0, -2.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.0, -2.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, 1.0, -2.5],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-0.5, 1.0, -2.5],
        color: [1.0, 1.0, 0.0],
    },

    Vertex {
        position: [-0.5, 0.0, -3.5],
        color: [1.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.0, -3.5],
        color: [0.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.5, 1.0, -3.5],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 1.0, -3.5],
        color: [0.2, 0.4, 1.0],
    },
];

pub(super) const CUBE_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,

    4, 7, 6,
    4, 6, 5,

    0, 3, 7,
    0, 7, 4,

    1, 5, 6,
    1, 6, 2,

    3, 2, 6,
    3, 6, 7,

    0, 4, 5,
    0, 5, 1,
];