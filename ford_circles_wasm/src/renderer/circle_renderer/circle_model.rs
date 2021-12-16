use crate::core::*;
use crate::renderer::model::Model;
use cgmath::{Matrix4, Vector3};

impl Model for Circle {
    fn model_matrix(&self) -> Matrix4<f32> {
        let translation = Vector3::new(
            self.centre.x.to_f32(),
            self.centre.y.to_f32(),
            0.0);
        cgmath::Matrix4::from_translation(translation)
        * cgmath::Matrix4::from_scale(self.radius.to_f32())
    }
}