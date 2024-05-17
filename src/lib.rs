use crate::object::rect::Rect;
use crate::object::Object;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use crate::camera::Camera;
use crate::shader_manager::{ShaderManager, sm};

#[macro_use]
mod web;
mod gl_objects;
mod object;
mod shader;
mod camera;
mod shader_manager;

static mut GL: Option<Box<WebGl2RenderingContext>> = None;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let tmp_gl = Box::new(init_webgl());

    unsafe {
        GL = Some(tmp_gl);
    }

    ShaderManager::init();

    let mut objects: Vec<Box<dyn Object>> = Vec::new();

    let rect1 = Rect::new([-100.0, -100.0], [1.0, 1.0], 2.0, [0.0, 1.0, 0.0, 1.0]);
    objects.push(Box::new(rect1));

    let rect2 = Rect::new([-1000.0, 500.0], [0.5, 0.5], 1.0, [1.0, 0.0, 0.0, 1.0]);
    objects.push(Box::new(rect2));

    let camera = Camera::new([1000.0, 100.0], 1.0);

    loop {
        sm().update_camera(&camera);
        tick(&mut objects);
        async_std::task::sleep(Duration::from_millis(100)).await;
    }

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

fn gl() -> &'static WebGl2RenderingContext {
    unsafe { GL.as_deref().expect("WebGL2 Context not initialized") }
}

fn tick(objects: &mut Vec<Box<dyn Object>>) {
    gl().clear_color(0.0, 0.0, 0.0, 1.0);
    gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    for object in objects.iter_mut() {
        object.tick();
        object.draw();
    }
}
