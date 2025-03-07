use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::{
    HtmlCanvasElement, WebGlRenderingContext as GL, WebGlProgram, WebGlShader,
};
use js_sys::Date;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();

    let gl: GL = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        uniform float angle;
        void main() {
            float s = sin(angle);
            float c = cos(angle);
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
        0.0,  0.5,  0.0,
       -0.5, -0.5,  0.0,
        0.5, -0.5,  0.0,
    ];

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vertices_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);
    }

    let position_attr = gl.get_attrib_location(&program, "position") as u32;
    gl.enable_vertex_attrib_array(position_attr);
    gl.vertex_attrib_pointer_with_i32(position_attr, 3, GL::FLOAT, false, 0, 0);

    let angle_loc = gl.get_uniform_location(&program, "angle").unwrap();

    // Start rendering loop
    render_loop(gl, angle_loc);
}

// Animation loop using requestAnimationFrame
fn render_loop(gl: GL, angle_loc: web_sys::WebGlUniformLocation) {
    fn render(gl: &GL, angle_loc: &web_sys::WebGlUniformLocation) {
        let time = (Date::now() / 1000.0) as f32;
        gl.uniform1f(Some(angle_loc), time);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.draw_arrays(GL::TRIANGLES, 0, 3);
    }

    let rc: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let rc_clone = rc.clone();

    *rc_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render(&gl, &angle_loc);
        web_sys::window()
            .unwrap()
            .request_animation_frame(
                rc.borrow().as_ref().unwrap().as_ref().unchecked_ref()
            )
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(
            rc_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        )
        .unwrap();
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or("Unknown error creating shader".into()))
    }
}

fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Unable to create shader object")?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or("Unknown error linking program".into()))
    }
}
