use super::shape::{Shape, Vertex};
use crate::graphics::drawable::{Draw, DrawData};
use crate::scene::SceneObject;
use glium::{backend::Facade, uniform, DrawParameters, Program, Surface};
use log::{debug, error};
use std::ops::Add;

#[derive(Debug)]
pub struct Mesh {
    draw_data: Option<DrawData>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub primitive: glium::index::PrimitiveType,
}

#[derive(Debug)]
pub enum MeshCombinationError {
    IncompatiblePrimitives,
}

impl Mesh {
    pub fn new(
        f: &impl Facade,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        primitive: glium::index::PrimitiveType,
    ) -> anyhow::Result<Self> {
        Ok(Mesh {
            draw_data: Some(DrawData::new(
                f,
                &vertices,
                &indices,
                primitive,
                Program::from_source(
                    f,
                    r#"
                        #version 330 core
                        precision highp float;

                        in vec3 position;
                        in vec4 color;
                        in vec2 tex_coords;

                        uniform mat4 model;
                        uniform mat4 view;
                        uniform mat4 projection;

                        out vec4 vertex_color;
                        out vec2 v_tex_coord;

                        void main() {
                            vertex_color = color;
                            gl_Position = projection * view * model * vec4(position, 1.0);
                            v_tex_coord = tex_coords;
                        }
                    "#,
                    r#"
                        #version 330 core
                        precision highp float;

                        in vec4 vertex_color;
                        in vec2 v_tex_coord;
                        out vec4 out_color;

                        uniform sampler2D texture_sampler;

                        void main() {
                            out_color = texture(texture_sampler, v_tex_coord);
                        }
                    "#,
                    None,
                )?,
            )?),
            primitive,
            vertices,
            indices,
        })
    }

    pub fn combine_with(mut self, other: Mesh) -> Result<Self, MeshCombinationError> {
        if self.primitive != other.primitive {
            Err(MeshCombinationError::IncompatiblePrimitives)
        } else {
            debug!(
                "Total vertices before combination: {}",
                self.vertices.len() + other.vertices.len()
            );
            let mut indices = Vec::new();
            let mut vertices = Vec::new();

            let it = self
                .indices
                .iter()
                .map(|i| (i, self.vertices.as_slice()[*i as usize]))
                .chain(
                    other
                        .indices
                        .iter()
                        .map(|i| (i, other.vertices[*i as usize])),
                );

            'outer: for x in it {
                for (i, _) in vertices.iter().enumerate() {
                    if x.1 == vertices[i] {
                        indices.push(i as u32);
                        continue 'outer;
                    }
                }

                indices.push(vertices.len() as u32);
                vertices.push(x.1);
            }

            self.vertices = vertices;
            self.indices = indices;

            debug!("Total vertices after combination: {}", self.vertices.len());
            Ok(self)
        }
    }

    pub fn from(f: &impl Facade, shape: impl Shape) -> anyhow::Result<Self> {
        Ok(Mesh::new(
            f,
            shape.vertices(),
            Vec::from(shape.indices()),
            shape.primitive(),
        )?)
    }
}

impl Add for Mesh {
    type Output = Result<Mesh, MeshCombinationError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine_with(rhs)
    }
}

impl Draw for Mesh {
    fn draw(&self, f: &mut glium::Frame, _camera: &super::camera::Camera) {
        if let Err(err) = f.draw(
            &self.draw_data.as_ref().unwrap().vertex_buffer,
            &self.draw_data.as_ref().unwrap().index_buffer,
            &self.draw_data.as_ref().unwrap().program,
            &uniform! {},
            &DrawParameters::default(),
        ) {
            error!("Couldn't draw mesh! {err:?}");
        }
    }
}

impl SceneObject for Mesh {}
