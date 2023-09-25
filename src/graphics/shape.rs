use glium::implement_vertex;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_coords);

pub trait Shape {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices(&self) -> &[u32];
    fn primitive(&self) -> glium::index::PrimitiveType;
}

pub struct Rectangle {
    pub vertices: Vec<Vertex>,
    trans_mat: glm::Mat4,
    scale_mat: glm::Mat4,
    rot_mat: glm::Mat4,
}

impl Rectangle {
    pub fn new(position: glm::Vec3, width: f32, height: f32) -> Rectangle {
        let vertices = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        Rectangle {
            vertices,
            trans_mat: glm::ext::translate(&num::one(), position),
            scale_mat: glm::ext::scale(&num::one(), glm::Vec3::new(width, height, 1.0)),
            rot_mat: num::one(),
        }
    }

    pub fn model_mat(&self) -> glm::Mat4 {
        self.trans_mat * self.scale_mat * self.rot_mat
    }
}

pub trait AsOpenGLMatrix4 {
    fn to_opengl_matrix(&self) -> [[f32; 4]; 4];
}

impl AsOpenGLMatrix4 for glm::Matrix4<f32> {
    fn to_opengl_matrix(&self) -> [[f32; 4]; 4] {
        let mut result = [[0.0; 4]; 4];

        for (i, v) in self.as_array().iter().enumerate() {
            result[i] = *v.as_array();
        }

        result
    }
}

impl Shape for Rectangle {
    fn vertices(&self) -> Vec<Vertex> {
        let model_mat = self.model_mat();
        self.vertices
            .iter()
            .map(|v| Vertex {
                position: (model_mat
                    * glm::Vec4::new(v.position[0], v.position[1], v.position[2], 1.0))
                .truncate(3)
                .as_array_mut()
                .to_owned(),
                tex_coords: v.tex_coords,
                color: v.color,
            })
            .collect()
    }

    fn indices(&self) -> &[u32] {
        &[0, 1, 2, 0, 2, 3]
    }

    fn primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::TrianglesList
    }
}

pub struct Cube {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub trans_mat: glm::Mat4,
    pub rot_mat: glm::Mat4,
    pub scale_mat: glm::Mat4,
}

impl Cube {
    pub fn new(coords: glm::Vec3, width: f32, height: f32, depth: f32) -> Cube {
        let scale_mat = glm::ext::scale(&num::one(), glm::Vec3::new(width, height, depth));
        let trans_mat = glm::ext::translate(&num::one(), coords);
        let rot_mat: glm::Mat4 = num::one();

        let vertices = vec![
            //Front
            Vertex {
                // 0
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.5],
            },
            Vertex {
                // 1
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.5],
            },
            Vertex {
                // 2
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                // 3
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                // 4
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.5],
            },
            Vertex {
                // 5
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.5],
            },
            Vertex {
                // 6
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                // 7
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                // 8
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.5],
            },
            Vertex {
                // 9
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.5],
            },
            Vertex {
                // 10
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                // 11
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                // 12
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.5],
            },
            Vertex {
                // 13
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.5],
            },
            Vertex {
                // 14
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                // 15
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 1.0, 1.0],
                tex_coords: [0.5, 0.0],
            },
        ];

        let indices = vec![
            4, 5, 6, 4, 6, 7, // Back
            0, 1, 2, 0, 2, 3, // Front
            4, 0, 3, 4, 3, 7, // Left
            1, 5, 6, 1, 6, 2, // Right
            8, 9, 10, 8, 10, 11, // Top
            12, 13, 14, 12, 14, 15, //Bottom
        ];

        Self {
            vertices,
            indices,
            scale_mat,
            trans_mat,
            rot_mat,
        }
    }

    pub fn model_mat(&self) -> glm::Mat4 {
        self.trans_mat * self.scale_mat * self.rot_mat
    }
}

impl Shape for Cube {
    fn vertices(&self) -> Vec<Vertex> {
        let model_mat = self.model_mat();
        self.vertices
            .iter()
            .map(|v| Vertex {
                position: (model_mat
                    * glm::Vec4::new(v.position[0], v.position[1], v.position[2], 1.0))
                .truncate(3)
                .as_array_mut()
                .to_owned(),
                tex_coords: v.tex_coords,
                color: v.color,
            })
            .collect()
    }

    fn indices(&self) -> &[u32] {
        self.indices.as_slice()
    }

    fn primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::TrianglesList
    }
}
