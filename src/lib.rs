use crate::shader::Shader;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use crate::object::Object;

#[macro_use]
mod web;
mod gl_objects;
mod object;
mod shader;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let gl = init_webgl();

    gl.create_vertex_array();

    let vert_src = web::get_string("/shaders/vertex.txt").await.unwrap();
    let frag_src = web::get_string("/shaders/fragment.txt").await.unwrap();

    let shader = Shader::new(&gl, vert_src.as_str(), frag_src.as_str())?;
    shader.bind();

    let vertices = vec![-0.7, -0.7, 0.7, -0.7, 0.0, 0.7];

    let mut objects: Vec<Object> = Vec::new();

    let tri1 = Object::new(&gl, &vertices, &shader, [0.0, 1.0, 0.0, 1.0]);
    objects.push(tri1);
    
    draw(&gl, &objects);

    Ok(())
}

fn init_webgl() -> WebGl2RenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap()
}

fn draw(gl: &WebGl2RenderingContext, objects: &Vec<Object>) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    for object in objects {
        object.draw();
    }
}
