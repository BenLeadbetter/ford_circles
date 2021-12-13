use luminance_front::Backend;
use luminance::{
    backend::framebuffer::{FramebufferBackBuffer, Framebuffer},
    context::GraphicsContext,
    texture,
};

pub trait Renderer<B = Backend>: std::marker::Sized 
where B: FramebufferBackBuffer,
{
    fn bootstrap(
        context: &mut impl GraphicsContext<Backend = B>,
    ) -> Self;

    fn render_frame(
        self,
        back_buffer: Framebuffer<B, texture::Dim2, (), ()>,
        actions: impl Iterator<Item = InputAction>,
        context: &mut impl GraphicsContext<Backend = B>,
    ) -> LoopFeedback<Self>;
}

#[derive(Clone, Debug)]
pub enum InputAction {
    Quit,
    Resized { width: u32, height: u32 },
    MouseWheel { delta: if32 }
    CursorMoved { x: f32, y: f32 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LoopFeedback<T> {
    Continue(T),
    Exit,
}