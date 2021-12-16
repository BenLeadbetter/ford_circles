mod circle_model;
mod circle_tess;

use luminance::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    pipeline::PipelineState,
    render_state::RenderState,
    shader::{
        types::{
            Mat44,
            Vec3,
        },
        Program,
        Uniform,
    },
    tess,
    texture,
    UniformInterface,
};
use luminance_webgl::webgl2::WebGL2;
use crate::renderer::model::Model as _;

#[derive(Clone, Debug)]
pub enum InputAction {
    Quit,
    Resized { width: u32, height: u32 },
    Wheel { delta: f32 },
    CursorMoved { x: f32, y: f32 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LoopFeedback<T> {
    Continue(T),
    Exit,
}

const VERTEX_SHADER: &'static str = include_str!("shaders/circle_shader.vert.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("shaders/circle_shader.frag.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    view: Uniform<Mat44<f32>>,
    #[uniform(unbound)]
    model: Uniform<Mat44<f32>>,
    #[uniform(unbound)]
    color: Uniform<Vec3<f32>>,
}

pub struct CircleRenderer {
    program: Program<WebGL2, circle_tess::VertexSemantics, (), ShaderInterface>,
    shape: tess::Tess<WebGL2, circle_tess::Vertex, u8>,
    eye: cgmath::Point3<f32>,
    fov: f32,
    view_matrix: cgmath::Matrix4<f32>,
}

impl CircleRenderer {
    pub fn bootstrap(context: &mut impl GraphicsContext<Backend = WebGL2>) -> Self {
        let program = context
            .new_shader_program::<circle_tess::VertexSemantics, (), ShaderInterface>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
            .expect("program creation")
            .ignore_warnings();

        let circle_data = circle_tess::CircleTessData::new(40);
        let shape = context
            .new_tess()
            .set_vertices(&*circle_data.vertices)
            .set_indices(&*circle_data.indices)
            .set_mode(tess::Mode::Triangle)
            .build()
            .unwrap();
        drop(circle_data);

        let eye = cgmath::Point3::new(0.0, 0.0, 1.0);
        let fov = 1.0;

        Self {
            program,
            shape,
            eye,
            fov,
            view_matrix: Self::calculate_view(eye, fov)
        }
    }

    pub fn render_frame(
        mut self,
        back_buffer: Framebuffer<WebGL2, texture::Dim2, (), ()>,
        actions: impl Iterator<Item = InputAction>,
        context: &mut impl GraphicsContext<Backend = WebGL2>,
        circles: &crate::core::Circles,
    ) -> LoopFeedback<Self> { 
        for action in actions {
            match action {
                InputAction::Resized { 
                    width, 
                    height 
                } => self.set_fov(width as f32 / height as f32),
                _ => {},
            }
        }

        let mut program = &mut self.program;
        let shape = &self.shape;

        let render = context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |mut program_interface, uniform_interface, mut render_gate| {
                        program_interface.set(
                            &uniform_interface.view, 
                            to_mat44(&self.view_matrix));

                        for circle in circles {
                            program_interface.set(
                                &uniform_interface.model, 
                                to_mat44(&circle.model_matrix()));
                            program_interface.set(
                                &uniform_interface.color,
                                Vec3::new(0.0, 1.0, 1.0)
                            );
                            render_gate.render(&RenderState::default(), |mut tess_gate| {
                                tess_gate.render(shape)
                            })?;
                        }

                        Ok(())
                    })
                },
            )
            .assume();
        
        if render.is_ok() {
            LoopFeedback::Continue(self)
        } else {
            LoopFeedback::Exit
        }
    }

    fn set_eye(&mut self, eye: cgmath::Point3<f32>) {
        self.eye = eye;
        self.view_matrix = Self::calculate_view(self.eye, self.fov);
    }

    fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.view_matrix = Self::calculate_view(self.eye, self.fov);
    }

    fn calculate_view(eye: cgmath::Point3<f32>, fov: f32) -> cgmath::Matrix4<f32> {
        let ret = cgmath::Matrix4::look_at_rh(
            eye,
            cgmath::Point3::<f32>::new(0.0, 0.0, 0.0),
            cgmath::Vector3::<f32>::new(0.0, 1.0, 0.0),
        );
        cgmath::Matrix4::<f32>::from_nonuniform_scale(fov, 1.0, 1.0) * ret
    }
}

fn to_mat44(matrix: &cgmath::Matrix4<f32>) -> Mat44<f32> {
    Mat44::new([
        [matrix.x.x, matrix.x.y, matrix.x.z, matrix.x.w],
        [matrix.y.x, matrix.y.y, matrix.y.z, matrix.y.w],
        [matrix.z.x, matrix.z.y, matrix.z.z, matrix.z.w],
        [matrix.w.x, matrix.w.y, matrix.w.z, matrix.w.w],
    ])
}

