use crate::renderer::{InputAction, LoopFeedback, Renderer};
use luminance::{Semantics, Vertex};
use luminance_front::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    pipeline::PipelineState,
    render_state:RenderState,
    shader,
    tess,
    texture,
    Backend,
};

const VERTEX_SHADER: &'static str = include_str!("circle_shader.vert.glsl");
const VERTEX_SHADER: &'static str = include_str!("circle_shader.frag.glsl");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "pos", repr = "[f32, 2]", wrapper = "VertexPosition")]
    Position,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, VertexPosition)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
}

const VERTICES: [Vertex; 3] = [
    Vertex::new(VertexPosition::new([0.5, -0.5])),
    Vertex::new(VertexPosition::new([0.0, 0.5])),
    Vertex::new(VertexPosition::new([-0.5, -0.5])),
]

const INDICES: [u8; 2] = [0, 1, 2];

pub struct CircleRenderer {
    program: shader::Program<Semantics, (), ()>,
    shape: Tess<Vertex>,
}

imple Renderer for CircleRenderer {
    fn bootstrap(context: &mut impl GraphicsBackend<Backend = Backend) -> Self {
        let program = context
            .new_shader_program::<Semantics, (), ()>()
            .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)
            .expect("program creation")
            .ignore_warnings();

        let shape = context
            .new_tess()
            .set_vertices(&VERTICES[..])
            .set_indices(&INDICES[..])
            .set_mode(tess::Mode::Triangle)
            .build()
            .unwrap();

        Self {
            program,
            shape,
        }
    }

    fn render_frame(
        mut self,
        back_buffer: Framebuffer<texture::Dim2, (), ()>,
        actions: impl Iterator<Item = InputAction>,
        context: &mut impl GraphicsContext<Backend = Backend>,
    ) -> LoopFeedback<Self> { 
        for action in actions {
            println!("Action: {}", action)
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
                            tess_gate.render(shape);
                        })
                    })
                },
            )
            .assume();
    }

    if render.is_ok() {
        LoopFeedback::Continue(self)
    } else {
        LoopFeedback::Exit
    }
}

