use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

// Retrieves WebGL context from a given canvas element
fn canvas_context(canvas: &HtmlCanvasElement) -> WebGlRenderingContext {
    canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap()
}

// Initializes WebGL and clears the canvas
#[wasm_bindgen(start)]
pub fn start() {
    // Get canvas element from HTML document
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Obtain WebGL context
    let gl = canvas_context(&canvas);

    // Set viewport and clear color
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.1, 0.2, 0.3, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}
