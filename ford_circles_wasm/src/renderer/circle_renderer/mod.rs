mod circle_model;
mod circle_tess;

use luminance::{
    blending::{
        Blending,
        Equation,
        Factor,
    },
    context::GraphicsContext,
    framebuffer::Framebuffer,
    pipeline::PipelineState,
    render_state::RenderState,
    shader::{
        types::{
            Mat44,
            Vec3,
            Vec2,
        },
        Program,
        Uniform,
    },
    tess,
    texture,
    UniformInterface,
};
use luminance_webgl::webgl2::WebGL2;
use crate::{
    core::*,
    calculate_circles,
    renderer::model::Model as _,
};

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
    #[uniform(unbound)]
    resolution: Uniform<Vec2<f32>>,
}

pub struct CircleRenderer {
    program: Program<WebGL2, circle_tess::VertexSemantics, (), ShaderInterface>,
    shape: tess::Tess<WebGL2, circle_tess::Vertex, u8>,
    eye: cgmath::Point3<f32>,
    resolution: [u32; 2],
    scale: f32,
    view_matrix: cgmath::Matrix4<f32>,
    circles: crate::core::Circles,
}

impl CircleRenderer {
    pub fn bootstrap(context: &mut impl GraphicsContext<Backend = WebGL2>) -> Self {
        let program = context
            .new_shader_program::<circle_tess::VertexSemantics, (), ShaderInterface>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
            .expect("program creation")
            .ignore_warnings();

        let circle_data = circle_tess::CircleTessData::new(10);
        let shape = context
            .new_tess()
            .set_vertices(&*circle_data.vertices)
            .set_indices(&*circle_data.indices)
            .set_mode(tess::Mode::Triangle)
            .build()
            .unwrap();
        drop(circle_data);

        let eye = cgmath::Point3::new(0.2, 0.0, 1.0);
        let scale = 1.0;
        let view_matrix = Self::calculate_view(eye, scale);

        Self {
            program,
            shape,
            eye,
            resolution: [640, 480],
            scale,
            view_matrix,
            circles: calculate_circles::in_view(
                &Circle {
                    centre: RationalPoint {
                        x: Rational::new(0, 1),
                        y: Rational::new(0, 1),
                    },
                    radius: Rational::new(1, 1),
                },
                Rational::new(1, 200),
            ),
        }
    }

    pub fn render_frame(
        mut self,
        back_buffer: Framebuffer<WebGL2, texture::Dim2, (), ()>,
        actions: impl Iterator<Item = InputAction>,
        context: &mut impl GraphicsContext<Backend = WebGL2>,
    ) -> LoopFeedback<Self> { 
        for action in actions {
            match action {
                InputAction::Resized { 
                    width, 
                    height 
                } => self.set_resolution([width, height]),
                InputAction::Wheel { 
                    delta 
                } => {
                    let mapped = 1.0 
                        - 2.0 / std::f32::consts::PI
                        * (delta / 100.0).atan();
                    self.set_scale(self.scale * mapped);
                }
                _ => {},
            }
        }

        let mut program = &mut self.program;
        let shape = &self.shape;

        let render = context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().
                    set_clear_color([0.02, 0.0, 0.04, 1.0]),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |mut program_interface, uniform_interface, mut render_gate| {
                        program_interface.set(
                            &uniform_interface.view, 
                            to_mat44(&self.view_matrix));
                        program_interface.set(
                            &uniform_interface.resolution,
                            Vec2::new(
                                self.resolution[0] as f32, 
                                self.resolution[1] as f32,
                        ));

                        let render_state = &RenderState::default()
                            .set_blending(Blending{
                                equation: Equation::Additive,
                                src: Factor::SrcAlpha,
                                dst: Factor::SrcAlphaComplement,
                            }).
                            set_depth_test(None);

                        for circle in &self.circles {
                            program_interface.set(
                                &uniform_interface.model, 
                                to_mat44(&circle.model_matrix()));

                            program_interface.set(
                                &uniform_interface.color,
                                circle.to_color()
                            );

                            render_gate.render(&render_state, |mut tess_gate| {
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

    fn _set_eye(&mut self, eye: cgmath::Point3<f32>) {
        self.eye = eye;
        self.view_matrix = Self::calculate_view(self.eye, self.scale);
        self.calculate_circles();
        todo!();
    }

    fn set_resolution(&mut self, resolution: [u32; 2]) {
        self.resolution = resolution;
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.view_matrix = Self::calculate_view(self.eye, self.scale);
        self.calculate_circles();
    }

    fn calculate_view(eye: cgmath::Point3<f32>, scale: f32) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, -0.4, 0.0))
        * cgmath::Matrix4::look_to_rh(
            eye,
            cgmath::Vector3::<f32>::new(0.0, 0.0, -1.0),
            cgmath::Vector3::<f32>::new(0.0, 1.0, 0.0),
        )
        * cgmath::Matrix4::from_scale(scale)
    }

    fn calculate_circles(&mut self) {
        let radius = Rational::approximate_float(5.0 / self.scale).unwrap();
        self.circles = calculate_circles::in_view(
            &Circle {
                centre: RationalPoint { 
                    x: Rational::approximate_float(self.eye.x).unwrap(),
                    y: Rational::approximate_float(self.eye.y).unwrap(),
                },
                radius,
            },
            radius * Rational::new(1, 200),
        );
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