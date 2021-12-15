use luminance::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    pipeline::PipelineState,
    render_state::RenderState,
    shader,
    tess,
    texture,
};
use luminance_webgl::webgl2::WebGL2;

mod circle_tess;

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

pub struct CircleRenderer {
    program: shader::Program<WebGL2, circle_tess::VertexSemantics, (), ()>,
    shape: tess::Tess<WebGL2, circle_tess::Vertex, u8>,
}


impl CircleRenderer {
    pub fn bootstrap(context: &mut impl GraphicsContext<Backend = WebGL2>) -> Self {
        let program = context
            .new_shader_program::<circle_tess::VertexSemantics, (), ()>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
            .expect("program creation")
            .ignore_warnings();

        let circle_data = circle_tess::CircleTessData::new(20);
        let shape = context
            .new_tess()
            .set_vertices(&*circle_data.vertices)
            .set_indices(&*circle_data.indices)
            .set_mode(tess::Mode::Triangle)
            .build()
            .unwrap();
        drop(circle_data);

        Self {
            program,
            shape,
        }
    }

    pub fn render_frame(
        mut self,
        back_buffer: Framebuffer<WebGL2, texture::Dim2, (), ()>,
        actions: impl Iterator<Item = InputAction>,
        context: &mut impl GraphicsContext<Backend = WebGL2>,
    ) -> LoopFeedback<Self> { 
        for action in actions {
            log::info!("Action: {:?}", action)
        }

        let program = &mut self.program;
        let shape = &self.shape;

        let render = context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shade_gate| {
                    shade_gate.shade(program, |_, _, mut render_gate| {
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(shape)
                        })
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
}

