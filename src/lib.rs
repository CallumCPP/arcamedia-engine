use crate::camera::Camera;
use crate::input::{input, Input};
use crate::object::rect::Rect;
use crate::object::textured_rect::TexturedRect;
use crate::object::Object;
use crate::shader_manager::{sm, ShaderManager};
use crate::texture_manager::{tm, TextureManager};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext};

#[macro_use]
mod web;
mod camera;
mod gl_objects;
mod input;
mod mesh;
mod object;
mod shader;
mod shader_manager;
mod texture;
mod texture_manager;
mod transform;

static mut GL: Option<Box<WebGl2RenderingContext>> = None;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let tmp_gl = Box::new(init_webgl());

    unsafe {
        GL = Some(tmp_gl);
    }

    let performance = window().unwrap().performance().unwrap();

    ShaderManager::init();
    TextureManager::init();
    Input::init();

    sm().get_shader("colored_vert.glsl", "colored_frag.glsl").await;
    sm().get_shader("textured_vert.glsl", "textured_frag.glsl").await;

    let mut objects: Vec<Box<dyn Object>> = Vec::new();

    let rect1 = Rect::new([2000.0, -100.0], [1.0, 1.0], 2.0, [0.0, 1.0, 0.0, 1.0]).await;
    objects.push(Box::new(rect1));

    let rect2 = Rect::new([-2000.0, -100.0], [1.0, 1.0], 2.0, [0.0, 1.0, 0.0, 1.0]).await;
    objects.push(Box::new(rect2));

    let rect3 = TexturedRect::new(
        [-1000.0, 500.0],
        [0.5, 0.5],
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("no texture.png").await,
    )
        .await;
    objects.push(Box::new(rect3));

    let rect4 = TexturedRect::new(
        [1000.0, 500.0],
        [0.5, 0.5],
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("test.png").await,
    )
        .await;
    objects.push(Box::new(rect4));

    let mut camera = Camera::new([1000.0, 100.0], 1.0);

    let mut last_time = performance.now();
    loop {
        let delta_time = (performance.now() - last_time) / 1000.0;
        last_time = performance.now();
        log!("FPS: {}", 1.0/delta_time);

        sm().update_camera(&camera);

        camera.tick(delta_time);
        tick(&mut objects, delta_time);

        input().flush_pressed_map();

        async_std::task::sleep(Duration::from_micros(10)).await;
    }

    // Ok(())
}

fn init_webgl() -> WebGl2RenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    gl.enable(WebGl2RenderingContext::BLEND);

    gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

    gl
}

fn gl() -> &'static WebGl2RenderingContext {
    unsafe { GL.as_deref().expect("WebGL2 Context not initialized") }
}

fn tick(objects: &mut [Box<dyn Object>], delta_time: f64) {
    gl().clear_color(0.0, 0.0, 0.0, 1.0);
    gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    for object in objects.iter_mut() {
        object.shader().bind();
        object.tick(delta_time);
        object.draw();
    }
}
