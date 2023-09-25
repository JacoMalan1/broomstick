use super::camera::Camera;
use super::shape::Vertex;
use glium::{backend::Facade, index::PrimitiveType, Frame, IndexBuffer, Program, VertexBuffer};

#[derive(Debug)]
pub enum BufferCreationError {
    Vertex(glium::vertex::BufferCreationError),
    Index(glium::index::BufferCreationError),
}

pub trait Transform {
    fn rotate(&mut self, axis: glm::Vec3, angle: f32);
    fn scale(&mut self, scale: glm::Vec3);
}

pub trait Draw {
    fn draw(&self, f: &mut Frame, camera: &Camera);
}

#[derive(Debug)]
pub struct DrawData {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
    pub program: Program,
}

impl DrawData {
    pub fn new(
        f: &impl Facade,
        vertices: &[Vertex],
        indices: &[u32],
        primitive: PrimitiveType,
        program: Program,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            vertex_buffer: VertexBuffer::new(f, vertices)?,
            index_buffer: IndexBuffer::new(f, primitive, indices)?,
            program,
        })
    }

    pub fn update(&mut self, vertices: &[Vertex], indices: &[u32]) {
        self.vertex_buffer.write(vertices);
        self.index_buffer.write(indices);
    }
}
