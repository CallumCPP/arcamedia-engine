use crate::input::{input, Input};
use crate::object::textured_rect::TexturedRect;
use crate::object::Object;
use crate::object_manager::{om, ObjectManager};
use crate::shader_manager::{sm, ShaderManager};
use crate::texture_manager::{tm, TextureManager};
use crate::transform::Transform;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[macro_use]
mod web;
mod camera;
mod gl_objects;
mod input;
mod mesh;
mod object;
mod object_manager;
mod shader;
mod shader_manager;
mod texture;
mod texture_manager;
mod transform;
mod vec2;

static mut GL: Option<Box<WebGl2RenderingContext>> = None;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let tmp_gl = Box::new(init_webgl());

    unsafe {
        GL = Some(tmp_gl);
    }

    let performance = web_sys::window().unwrap().performance().unwrap();

    ShaderManager::init();
    TextureManager::init();
    ObjectManager::init();
    Input::init();

    sm().get_shader("colored_vert.glsl", "colored_frag.glsl")
        .await;
    sm().get_shader("textured_vert.glsl", "textured_frag.glsl")
        .await;

    let rect = TexturedRect::new(
        [-1000.0, 500.0].into(),
        [500.0, 500.0].into(),
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("no texture.png").await,
    )
    .await;
    om().add_object(obj!(rect));

    let rect = TexturedRect::new(
        [1000.0, 500.0].into(),
        [1024.0, 512.0].into(),
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("test.png").await,
    )
    .await;
    om().add_object(obj!(rect));

    let mut last_time = performance.now();
    loop {
        let delta_time = (performance.now() - last_time) / 1000.0;
        last_time = performance.now();
        log!("FPS: {}", 1.0 / delta_time);

        gl().clear_color(0.0, 0.0, 0.0, 1.0);
        gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        om().tick(delta_time);
        om().draw();

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

    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    gl
}

fn gl() -> &'static WebGl2RenderingContext {
    unsafe { GL.as_deref().expect("WebGL2 Context not initialized") }
}
