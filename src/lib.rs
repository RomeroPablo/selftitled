use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

#[wasm_bindgen]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    gl: GL,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Renderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        // Get canvas by ID
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        // Initialize WebGL2 context
        let gl: GL = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into()?;

        Ok(Renderer { canvas, gl })
    }

    pub fn resize_and_render(&self, width: u32, height: u32) {
        // Set canvas dimensions
        self.canvas.set_width(width);
        self.canvas.set_height(height);

        // Reset viewport and redraw
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.render();
    }

    pub fn render(&self) {
        // Your drawing logic (currently just clearing to a color)
        self.gl.clear_color(0.38, 0.24, 0.52, 1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
    }
}
