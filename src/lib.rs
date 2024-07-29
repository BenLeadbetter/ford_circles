use wasm_bindgen::prelude::*;

type Transform = nalgebra::base::Matrix3<f64>;
type Vector = nalgebra::base::Vector2<f64>;

#[derive(Debug, Default)]
struct CanvasSize {
    width: f64,
    height: f64,
}

#[wasm_bindgen]
pub struct State {
    dirty: bool,
    transform: Transform,
    canvas_size: CanvasSize,
}

#[wasm_bindgen]
pub fn init() -> State {
    State {
        dirty: true,
        transform: Transform::identity(),
        canvas_size: Default::default(),
    }
}

#[wasm_bindgen]
pub fn update_transform(state: &mut State, event: &web_sys::WheelEvent) {
    web_sys::console::debug_1(&format!("update transform {:?}", event).into());
    state.transform.append_translation_mut(&Vector::new(event.delta_x(), 0.0));
    state.dirty = true;
}

#[wasm_bindgen]
pub fn update_canvas_size(state: &mut State, canvas: &web_sys::HtmlCanvasElement) {
    web_sys::console::debug_1(&format!("update canvas size {:?}", canvas).into());

    state.canvas_size.width = canvas.width().into();
    state.canvas_size.height = canvas.height().into();

    state.transform = Transform::identity();
    state.transform.append_nonuniform_scaling_mut(&Vector::new(
        state.canvas_size.height / 2.0,
        -state.canvas_size.height / 2.0,
    ));
    state
        .transform
        .append_translation_mut(&Vector::new(0.0, state.canvas_size.height / 2.0));

    state.dirty = true;
}

#[wasm_bindgen]
pub fn render(
    state: &mut State,
    context: &web_sys::CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    if !state.dirty {
        return Ok(());
    }

    web_sys::console::debug_1(&"start render".into());

    clear(state, context)?;

    set_transform_to_context(&state.transform, context)?;

    draw_circle(context, 0.0, 1.0)?;

    state.dirty = false;

    web_sys::console::debug_1(&"finished render".into());

    Ok(())
}

fn draw_circle(context: &web_sys::CanvasRenderingContext2d, p: f64, q: f64) -> Result<(), JsValue> {
    web_sys::console::debug_1(&format!("draw circle: ({p}, {q})").into());
    let x = p / q;
    let r = 1.0 / (2.0 * q * q);
    context.begin_path();
    context.arc(x, r, r, 0.0, std::f64::consts::TAU)?;
    // todo: a colour scheme
    context.set_fill_style(&JsValue::from_str("red"));
    context.fill();
    context.close_path();
    Ok(())
}

fn set_transform_to_context(
    transform: &Transform,
    context: &web_sys::CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    web_sys::console::debug_1(&format!("set transform: {:?}", transform).into());
    let data = transform.as_slice();
    context.set_transform(data[0], data[1], data[3], data[4], data[6], data[7])
}

fn clear(state: &State, context: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    context.set_transform_with_default_dom_matrix_2d_init()?;
    context.clear_rect(0.0, 0.0, state.canvas_size.width, state.canvas_size.height);
    Ok(())
}
