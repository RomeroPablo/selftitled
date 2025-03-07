use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGlRenderingContext as GL, WebGlProgram, WebGlShader,
};
use js_sys::Date;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let gl = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();

    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        uniform float angle;
        void main() {
            float c = cos(angle);
            float s = sin(angle);
            gl_Position = vec4(
                position.x * c - position.y * s,
                position.x * s + position.y * c,
                position.z,
                1.0
            );
        }
        "#,
    ).unwrap();

    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(0.8, 0.3, 0.4, 1.0);
        }
        "#,
    ).unwrap();

    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();
    gl.use_program(Some(&program));

    let vertices: [f32; 9] = [
        0.0,  0.5, 0.0,
       -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
    ];

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vertex_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_array, GL::STATIC_DRAW);
    }

    let position_loc = gl.get_attrib_location(&program, "position") as u32;
    gl.enable_vertex_attrib_array(position_loc);
    gl.vertex_attrib_pointer_with_i32(position_loc, 3, GL::FLOAT, false, 0, 0);

    let angle_loc = gl.get_uniform_location(&program, "angle").unwrap();

    // Start the animation loop properly
    animate(gl, angle_loc);
}

// Correct animation loop setup
fn animate(gl: GL, angle_loc: web_sys::WebGlUniformLocation) {
    //let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let f: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>> 
        = std::rc::Rc::new(std::cell::RefCell::new(None));

    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = Date::now() as f32 / 1000.0;
        gl.uniform1f(Some(&angle_loc), time);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        // request next frame
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Cannot create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap() {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap())
    }
}

fn link_program(gl: &GL, vs: &WebGlShader, fs: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Cannot create program")?;
    gl.attach_shader(&program, vs);
    gl.attach_shader(&program, fs);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap() {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap())
    }
}
