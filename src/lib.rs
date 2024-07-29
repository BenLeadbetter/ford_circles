use wasm_bindgen::prelude::*;

type Transform = nalgebra::geometry::Transform2<f64>;

#[wasm_bindgen]
pub struct State {
    dirty: bool,
    transform: Transform,
}

#[wasm_bindgen]
pub fn init() -> State {
    State {
        dirty: true,
        transform: Transform::identity(),
    }
}

#[wasm_bindgen]
pub fn render(
    state: &mut State,
    context: &web_sys::CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    if !state.dirty {
        return Ok(());
    }

    set_transform_to_context(&state.transform, context)?;

    draw_circle(context, 0.0, 1.0)?;

    state.dirty = false;

    web_sys::console::debug_1(&"Finished render".into());

    Ok(())
}

fn draw_circle(context: &web_sys::CanvasRenderingContext2d, p: f64, q: f64) -> Result<(), JsValue> {
    web_sys::console::debug_1(&format!("draw circle: ({p}, {q})").into());
    let x = p / q;
    let r = 1.0 / (2.0 * q * q) * 100.0;
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
    web_sys::console::debug_1(&format!("set transform: {:?}", transform.matrix()).into());
    let data = transform.matrix().as_slice();
    context.set_transform(data[0], data[1], data[3], data[4], data[6], data[7])
}
