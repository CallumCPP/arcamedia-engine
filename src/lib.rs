use crate::input::{input, Input};
use crate::object::player::Player;
use crate::object::rect::Rect;
use crate::object::textured_rect::TexturedRect;
use crate::object_manager::{om, ObjectManager};
use crate::shader_manager::{sm, ShaderManager};
use crate::texture_manager::{tm, TextureManager};
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

    let player = Player::new(
        [-100.0, 500.0].into(),
        [100.0, 200.0].into(),
        0.0,
        [0.0, 1.0, 0.0, 1.0],
        tm().get_texture("player.png").await,
    )
    .await;
    object!(player);

    let rect = Rect::new(
        [0.0, -2000.0].into(),
        [4000.0, 10.0].into(),
        0.0,
        [1.0, 1.0, 1.0, 1.0],
        true,
    )
    .await;
    object!(rect);

    let rect = Rect::new(
        [0.0, 2300.0].into(),
        [4000.0, 10.0].into(),
        0.0,
        [1.0, 1.0, 1.0, 1.0],
        true,
    )
    .await;
    object!(rect);

    let rect = Rect::new(
        [-2000.0, 0.0].into(),
        [10.0, 4000.0].into(),
        0.0,
        [1.0, 1.0, 1.0, 1.0],
        true,
    )
    .await;
    object!(rect);

    let rect = Rect::new(
        [2000.0, 0.0].into(),
        [10.0, 4000.0].into(),
        0.0,
        [1.0, 1.0, 1.0, 1.0],
        true,
    )
    .await;
    object!(rect);

    let rect = TexturedRect::new(
        [-1000.0, 500.0].into(),
        [500.0, 500.0].into(),
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("no texture.png").await,
        true,
    )
    .await;
    object!(rect);

    let rect = TexturedRect::new(
        [1000.0, 500.0].into(),
        [1024.0, 512.0].into(),
        1.0,
        [1.0, 1.0, 1.0, 1.0],
        tm().get_texture("test.png").await,
        true,
    )
    .await;
    object!(rect);

    let mut last_time = performance.now();
    loop {
        // log!("-----------");
        let delta_time = (performance.now() - last_time) / 1000.0;
        last_time = performance.now();
        log!("FPS: {}", 1.0 / delta_time);

        // gl().clear_color(random() as f32, random() as f32, random() as f32, 1.0);
        gl().clear_color(0.1, 0.1, 0.1, 1.0);
        gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // let mut timer = performance.now();
        om().tick(delta_time);
        // log!("Tick took {} milliseconds", performance.now() - timer);
        // timer = performance.now();
        om().draw();
        // log!("Draw took {} milliseconds", performance.now() - timer);

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
