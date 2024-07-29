import * as rs from './wasm/ford_circles_rs.js';

let state = {};

async function start() {
    await initWasm();
    initState();
    setupEventHandlers();

    console.info('ford circles initialized');

    render();
}

async function initWasm() {
    const response = await fetch('/wasm/ford_circles_rs_bg.wasm');
    const bytes = await response.arrayBuffer();
    await rs.default(bytes);
}

function initState() {
    state.rsState = rs.init();
    state.canvas = document.getElementById('canvas');
    if (!state.canvas) {
        throw new Error('Canvas element not found');
    }
    state.context = canvas.getContext('2d');
    if (!state.context) {
        throw new Error('2D context not found');
    }
    rs.update_canvas_size(state.rsState, state.canvas);

}

function setupEventHandlers() {
    state.canvas.addEventListener('wheel', (event) => {
        event.preventDefault();
        rs.update_transform_on_wheel(state.rsState, event);
    });
}

function render() {
    rs.render(state.rsState, state.context);
    requestAnimationFrame(render);
}

window.onload = start;
