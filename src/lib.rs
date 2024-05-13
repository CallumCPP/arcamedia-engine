use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

#[macro_use]
mod web;
mod shader;
mod gl_objects;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_string = web::get_string("/shaders/vertex.txt").await.unwrap();
    let frag_string = web::get_string("/shaders/fragment.txt").await.unwrap();


    let vert_shader = shader::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vert_string.as_str()
    )?;

    let frag_shader = shader::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &frag_string.as_str()
    ).unwrap_or_else(|e| {
        web_sys::console::log_1(&format!("error: {e}").into());
        vert_shader.clone()
    });

    let program = shader::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourselves
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count, program);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32, shader_program: WebGlProgram) {
    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let color_location;

    match context.get_uniform_location(&shader_program, "fragColor") {
        None => {
            web_sys::console::log_1(&format!("error {}", 1).into());
        }
        Some(sp) => {
            color_location = sp;
        }
    }

    let color_location = context
        .get_uniform_location(&shader_program, "fragColor")
        .unwrap();

    context.uniform4fv_with_f32_array(Some(&color_location), vec![0.0, 1.0, 0.0, 1.0].as_mut_slice());

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}
