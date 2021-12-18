use crate::core::*;
use crate::renderer::model::Model;
use cgmath::{Matrix4, Vector3};
use luminance::shader::types::Vec3;
use palette::{
    FromColor,
    Lch, 
    Srgb,
};

impl Model for Circle {
    fn model_matrix(&self) -> Matrix4<f32> {
        let translation = Vector3::new(
            self.centre.x.to_f32(),
            self.centre.y.to_f32(),
            0.0);
        cgmath::Matrix4::from_translation(translation)
        * cgmath::Matrix4::from_scale(self.radius.to_f32())
    }

    fn to_color(&self) -> Vec3<f32> {
        let hue = 1.0 / (self.radius.to_f32() * self.radius.to_f32());
        let color = Srgb::from_color(
            Lch::from_components((80.0, 128.0, hue))
        );
        Vec3::new(color.red, color.green, color.blue)
    }
}