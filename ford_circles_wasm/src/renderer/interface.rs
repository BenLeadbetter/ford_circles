use crate::{
    renderer::circle_renderer::{
        CircleRenderer, 
        InputAction,
        LoopFeedback,
    },
};
use luminance_web_sys::WebSysWebGL2Surface;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Interface {
    surface: WebSysWebGL2Surface,
    actions: std::vec::Vec<InputAction>,
    renderer: Option<CircleRenderer>,
}

#[wasm_bindgen]
impl Interface {
    fn new(mut surface: WebSysWebGL2Surface) -> Self {
        let renderer = CircleRenderer::bootstrap(&mut surface);

        let map_js_value = |v: Result<JsValue, JsValue>| {
            v
                .ok()
                .and_then(|jsv| jsv.as_f64())
                .map(|f| f as u32)
                .unwrap()
        };
        let width = map_js_value(surface.window.inner_width());
        let height = map_js_value(surface.window.inner_height());
       
        Interface {
            surface,
            actions: vec![InputAction::Resized{ width, height }],
            renderer: Some(renderer),
        }
    }

    pub fn enqueue_quit_action(&mut self) {
        self.actions.push(InputAction::Quit);
    }

    pub fn enqueue_resize_action(&mut self, width: u32, height: u32) {
        self.actions.push(InputAction::Resized{ width, height });
    }

    pub fn enqueue_wheel_action(&mut self, delta: f32) {
        self.actions.push(InputAction::Wheel{ delta });
    }

    pub fn enqueue_cursor_moved_action(&mut self, x: f32, y: f32) {
        self.actions.push(InputAction::CursorMoved{ x, y });
    }

    pub fn render(&mut self) -> bool {
        match self.renderer.take() {
            Some(val) => { 
                let feedback = val.render_frame(
                    self.surface.back_buffer().expect("WebGL backbuffer"),
                    self.actions.iter().cloned(),
                    &mut self.surface,
                );
                self.actions.clear();
                match feedback {
                    LoopFeedback::Continue(stepped) => {
                        self.renderer = Some(stepped);
                        true
                    }
                    LoopFeedback::Exit => false
                }
            },
            None => false
        }
    }
}

#[wasm_bindgen]
pub fn get_interface(canvas_name: &str) -> Interface {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();

    log::info!("creating the WebGL2 interface..");
    let surface = WebSysWebGL2Surface::new(canvas_name).expect("WebGL2 canvas");
    Interface::new(surface)
}