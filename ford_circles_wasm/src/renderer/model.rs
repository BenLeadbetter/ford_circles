pub trait Model {
    fn model_matrix(&self) -> cgmath::Matrix4<f32>;
}