use cgmath::{Matrix4, Rad, Zero};

pub struct Transform {
    pub rotation: [Rad<f32>; 3],
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            rotation: [Rad::zero(); 3],
        }
    }

    pub fn build_transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_x(self.rotation[0])
            * Matrix4::from_angle_y(self.rotation[1])
            * Matrix4::from_angle_z(self.rotation[2])
    }
}
