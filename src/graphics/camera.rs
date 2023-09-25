pub struct Camera {
    rot_mat: glm::Mat4,
    trans_mat: glm::Mat4,
    scale_mat: glm::Mat4,
    pub proj_mat: glm::Mat4,
}

impl Camera {
    pub fn new(fovy: f32, aratio: f32, near: f32, far: f32) -> Camera {
        Camera {
            rot_mat: num::one(),
            trans_mat: num::one(),
            scale_mat: num::one(),
            proj_mat: glm::ext::perspective(fovy, aratio, near, far),
        }
    }

    pub fn rotate(&mut self, axis: glm::Vec3, angle: f32) {
        self.rot_mat = glm::ext::rotate(&self.rot_mat, angle, axis);
    }

    pub fn scale(&mut self, scale_vec: glm::Vec3) {
        self.scale_mat = glm::ext::scale(&self.scale_mat, scale_vec);
    }

    pub fn translate(&mut self, trans_vec: glm::Vec3) {
        self.trans_mat = glm::ext::translate(&self.trans_mat, trans_vec);
    }

    pub fn view_mat(&self) -> glm::Mat4 {
        self.trans_mat * self.scale_mat * self.rot_mat
    }
}
