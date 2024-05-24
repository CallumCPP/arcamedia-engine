use std::cell::RefCell;
use std::rc::Rc;
use crate::engine::input::input;
use crate::engine::object::player::Player;
use crate::engine::object::rect::Rect;
use crate::engine::object::textured_rect::TexturedRect;
use crate::engine::object_manager::om;
use crate::engine::texture_manager::tm;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use crate::engine::*;
use crate::engine::timer::Timer;

mod engine;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    Engine::init();

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

    let mut timer = Timer::new();
    loop {
        // log!("-----------");
        let delta_time = timer.elapsed_reset() / 1000.0;
        // log!("FPS: {}", 1.0 / delta_time);

        // gl().clear_color(random() as f32, random() as f32, random() as f32, 1.0);
        gl().clear_color(0.1, 0.1, 0.1, 1.0);
        gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let mut timer2 = Timer::new();
        om().tick(delta_time);
        log!("Tick took {} milliseconds", timer2.elapsed_reset());
        om().draw();
        log!("Draw took {} milliseconds", timer2.elapsed_reset());

        input().flush_pressed_map();

        async_std::task::sleep(Duration::from_micros(10)).await;
    }

    // Ok(())
}
