use luminance::shader::types::Vec3;

pub trait Model {
    fn model_matrix(&self) -> cgmath::Matrix4<f32>;
    fn to_color(&self) -> Vec3<f32>;
}