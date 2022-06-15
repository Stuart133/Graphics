use cgmath::{Matrix4, Rad, Zero};

pub struct Transform {
    pub rotation: [Rad<f32>; 3],
    pub scale: [f32; 3],
    pub translate: [f32; 3],
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            rotation: [Rad::zero(); 3],
            scale: [1.0; 3],
            translate: [0.0; 3],
        }
    }

    pub fn build_transform_matrix(&self) -> Matrix4<f32> {
        // TODO - Figure out the right rotation axis for a single matrix
        Matrix4::from_angle_x(self.rotation[0])
            * Matrix4::from_angle_y(self.rotation[1])
            * Matrix4::from_angle_z(self.rotation[2])
            * Matrix4::from_translation(self.translate.into())
            * Matrix4::from_nonuniform_scale(self.scale[0], self.scale[1], self.scale[2])
    }

    #[inline]
    pub fn as_uniform(&self) -> [[f32; 4]; 4] {
        self.build_transform_matrix().into()
    }
}
