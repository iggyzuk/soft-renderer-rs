use super::vertex::Vertex;

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,  // these vertices must be homogenous w: 1
    pub indices: Vec<usize>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }
}
