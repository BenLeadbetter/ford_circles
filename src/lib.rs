use wasm_bindgen::prelude::*;

type Matrix = nalgebra::base::Matrix3<f64>;
type Vector = nalgebra::base::Vector2<f64>;

// based on the nightfly vim colourscheme
// https://github.com/bluz71/vim-nightfly-colors
const BACKGROUND_COLOUR: &str = "#011627";
const CIRCLE_COLOURS: [&str; 12] = [
    "#FC514F",
    "#A1CD5E",
    "#E3D18A",
    "#82AAFF",
    "#C792EB",
    "#7FDBCA",
    "#FF5874",
    "#22C7A8",
    "#FFCB8B",
    "#86BCFF",
    "#AE81FF",
    "#F88C6C",
];

#[derive(Debug, Default)]
struct CanvasSize {
    width: f64,
    height: f64,
}

#[wasm_bindgen]
pub struct State {
    dirty: bool,
    transform: Matrix,
    canvas_size: CanvasSize,
}

#[wasm_bindgen]
pub fn init() -> State {
    State {
        dirty: true,
        transform: Matrix::identity(),
        canvas_size: Default::default(),
    }
}

#[wasm_bindgen]
pub fn update_transform_on_wheel(state: &mut State, event: &web_sys::WheelEvent) {
    web_sys::console::debug_1(&format!("update transform {:?}", event).into());

    scroll(state, event.delta_x());
    zoom(
        state,
        event.offset_x() as f64,
        1.0 + event.delta_y().atan() * 0.05,
    );

    state.dirty = true;
}

/// zoom about (x_centre, canvas centre y)
fn zoom(state: &mut State, x_centre: f64, zoom: f64) {
    let zoom_centre = Vector::new(x_centre, state.canvas_size.height / 2.0);
    let transform = &mut state.transform;

    transform.append_translation_mut(&(-1.0 * zoom_centre));
    transform.append_scaling_mut(zoom);
    transform.append_translation_mut(&zoom_centre);
}

/// scroll horizontally
fn scroll(state: &mut State, delta: f64) {
    let transform = &mut state.transform;
    transform.append_translation_mut(&Vector::new(delta, 0.0));
}

#[wasm_bindgen]
pub fn update_canvas_size(state: &mut State, canvas: &web_sys::HtmlCanvasElement) {
    web_sys::console::debug_1(&format!("update canvas size {:?}", canvas).into());

    state.canvas_size.width = canvas.width().into();
    state.canvas_size.height = canvas.height().into();

    state.transform = Matrix::identity();
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
    draw(state, context)?;
    state.dirty = false;

    web_sys::console::debug_1(&"finished render".into());

    Ok(())
}

pub fn draw(state: &State, context: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    let (x_min, x_max) = visible_range(state);
    let q_max = calc_q_max(x_min, x_max);

    web_sys::console::debug_1(&format!("draw {x_min} to {x_max}. q_max: {q_max}").into());

    for q in 1..q_max {
        let p_min = (x_min * q as f64).floor() as i64;
        let p_max = (x_max * q as f64).ceil() as i64;
        for p in p_min..p_max {
            use num_integer::Integer;
            if p.gcd(&q)  == 1 {
                draw_circle(context, p, q)?;
            }
        }
    }

    Ok(())
}

fn calc_q_max(x_min: f64, x_max: f64) -> i64 {
    const MIN_CIRCLE_RADIUS_SCREEN_RATIO: f64 = 0.01;
    let r_min = (x_max - x_min - 1.0) * 0.5 * MIN_CIRCLE_RADIUS_SCREEN_RATIO;
    (1.0 / (2.0 * r_min)).sqrt().ceil() as i64
}

fn visible_range(state: &State) -> (f64, f64) {
    let inverse = state.transform.try_inverse().unwrap();
    let data =  inverse * nalgebra::base::Matrix3x2::<f64>::new(
            0.0,
            state.canvas_size.width,
            0.0,
            state.canvas_size.height,
            1.0,
            1.0,
        );
    (data[0] - 0.5, data[3] + 0.5)
}

fn draw_circle(context: &web_sys::CanvasRenderingContext2d, p: i64, q: i64) -> Result<(), JsValue> {
    let color = CIRCLE_COLOURS[(q as usize + 2) % 12];
    let p = p as f64;
    let q = q as f64;
    let x = p / q;
    let r = 1.0 / (2.0 * q * q);
    context.begin_path();
    context.arc(x, r, r, 0.0, std::f64::consts::TAU)?;
    context.set_fill_style(&JsValue::from_str(color));
    context.fill();
    context.close_path();
    Ok(())
}

fn set_transform_to_context(
    transform: &Matrix,
    context: &web_sys::CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    web_sys::console::debug_1(&format!("set transform: {:?}", transform).into());
    let data = transform.as_slice();
    context.set_transform(data[0], data[1], data[3], data[4], data[6], data[7])
}

fn clear(state: &State, context: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    context.set_transform_with_default_dom_matrix_2d_init()?;
    context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOUR));
    context.fill_rect(0.0, 0.0, state.canvas_size.width, state.canvas_size.height);
    Ok(())
}
