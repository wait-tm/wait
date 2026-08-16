use glam::Mat4;

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(matrix: Mat4) -> Self {
        Self {
            view_projection: matrix.to_cols_array_2d(),
        }
    }
}