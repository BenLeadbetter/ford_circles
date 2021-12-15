use luminance::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    pipeline::PipelineState,
    render_state::RenderState,
    shader,
    tess,
    texture,
    Semantics, 
    Vertex
};
use luminance_webgl::webgl2::WebGL2;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct Vertex {
    pos: VertexPosition,
}

const VERTICES: [Vertex; 3] = [
    Vertex::new(VertexPosition::new([0.5, -0.5])),
    Vertex::new(VertexPosition::new([0.0, 0.5])),
    Vertex::new(VertexPosition::new([-0.5, -0.5])),
];

const INDICES: [u8; 3] = [0, 1, 2];

pub struct CircleRenderer {
    program: shader::Program<WebGL2, VertexSemantics, (), ()>,
    shape: tess::Tess<WebGL2, Vertex, u8>,
}


impl CircleRenderer {
    pub fn bootstrap(context: &mut impl GraphicsContext<Backend = WebGL2>) -> Self {
        let program = context
            .new_shader_program::<VertexSemantics, (), ()>()
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

